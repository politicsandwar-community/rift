use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

#[proc_macro_derive(
    Model,
    attributes(
        table,
        no_save,
        primary_key,
        no_type_check,
        cache_id,
        cache_name,
        cache_type,
        cache_unwrap,
        field,
        field_custom,
        field_no_update,
        subscriptions,
        has_pnwkit,
        slice,
    )
)]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_model_derive(&ast)
}

fn get_struct_attr(ast: &syn::DeriveInput, name: &str) -> String {
    match ast.attrs.iter().find(|attr| attr.path.is_ident(name)) {
        Some(attr) => match attr.parse_meta().unwrap() {
            syn::Meta::NameValue(meta) => match meta.lit {
                syn::Lit::Str(lit) => lit.value(),
                _ => panic!("{} attribute must be a string literal", name),
            },
            _ => panic!("{} attribute must be a string literal", name),
        },
        None => panic!("Models must have a {} attribute", name),
    }
}

fn get_option_attr(ast: &syn::DeriveInput, name: &str) -> Option<String> {
    match ast.attrs.iter().find(|attr| attr.path.is_ident(name)) {
        Some(attr) => match attr.parse_meta().unwrap() {
            syn::Meta::NameValue(meta) => match meta.lit {
                syn::Lit::Str(lit) => Some(lit.value()),
                _ => panic!("attribute must be a string literal"),
            },
            _ => panic!("attribute must be a string literal"),
        },
        None => None,
    }
}

fn impl_model_derive(ast: &syn::DeriveInput) -> TokenStream {
    let data = match &ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Modal can only be derived on structs"),
    };
    let name = &ast.ident;
    let table = get_struct_attr(ast, "table");
    let cache_name = get_struct_attr(ast, "cache_name");
    let pnwkit_field = get_option_attr(ast, "has_pnwkit");
    let primary_keys: Vec<String> = get_option_attr(ast, "primary_key")
        .unwrap_or_else(|| "id".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let cache_id = get_option_attr(ast, "cache_id");
    if primary_keys.len() > 1 && cache_id.is_none() {
        panic!("if specifying more than one primary key, you must specify a cache_id");
    }
    let cache_id: Expr = syn::parse_str(
        format!(
            "{}{}",
            cache_id.unwrap_or_else(|| primary_keys[0].clone()).as_str(),
            if get_option_attr(ast, "cache_unwrap").is_some() {
                ".unwrap()"
            } else {
                ""
            },
        )
        .as_str(),
    )
    .unwrap();
    let cache_type: Expr = syn::parse_str(
        get_option_attr(ast, "cache_type")
            .unwrap_or_else(|| "i32".to_string())
            .as_str(),
    )
    .unwrap();
    let mut no_type_check_columns: Vec<String> = vec![];
    let mut slice_columns: Vec<String> = vec![];
    let insert_columns: Vec<String> =
        data.fields
            .iter()
            .filter(|field| {
                !field.attrs.iter().any(|attr| {
                    attr.path.is_ident("no_save") || attr.path.is_ident("auto_increment")
                })
            })
            .map(|field| {
                let no_type_check = field
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident("no_type_check"));
                let slice = field.attrs.iter().any(|attr| attr.path.is_ident("slice"));
                let column = field
                    .ident
                    .as_ref()
                    .expect("fields must have an identifier")
                    .to_string();
                if no_type_check {
                    no_type_check_columns.push(column.clone());
                }
                if slice {
                    slice_columns.push(column.clone());
                }
                column
            })
            .collect();
    let insert_names = insert_columns.join(", ");
    let insert_values: Vec<String> = (0..insert_columns.len())
        .map(|i| format!("${}", i + 1))
        .collect();
    let insert_values = insert_values.join(", ");
    let returning_names = insert_columns
        .iter()
        .map(|c| {
            if no_type_check_columns.contains(c) {
                format!(r#"{c} as "{c}: _""#)
            } else {
                c.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join(", ");
    let insert_statement: Expr = syn::parse_str(
        format!(
            "r#\"{}\"#",
            format_args!(
                "INSERT INTO {table} ({insert_names}) VALUES ({insert_values}) RETURNING {returning_names};"
            )
        )
        .as_str(),
    )
    .unwrap();

    let update_columns: Vec<String> = data
        .fields
        .iter()
        .filter(|field| !field.attrs.iter().any(|attr| attr.path.is_ident("no_save")))
        .map(|field| {
            let no_type_check = field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("no_type_check"));
            let slice = field.attrs.iter().any(|attr| attr.path.is_ident("slice"));
            let column = field
                .ident
                .as_ref()
                .expect("fields must have an identifier")
                .to_string();
            if no_type_check {
                no_type_check_columns.push(column.clone());
            }
            if slice {
                slice_columns.push(column.clone());
            }
            column
        })
        .collect();
    let update_names = Iterator::enumerate(update_columns.iter())
        .map(|(i, column)| format!("{} = ${}", column, i + 1))
        .collect::<Vec<String>>()
        .join(", ");
    let update_where = primary_keys
        .iter()
        .map(|key| {
            format!(
                "{} = ${}",
                key,
                update_columns
                    .iter()
                    .position(|c| c == key)
                    .unwrap_or_else(|| panic!("primary key {} not present in struct", key))
                    + 1
            )
        })
        .collect::<Vec<String>>()
        .join(" AND ");
    let update_statement = format!("UPDATE {table} SET {update_names} WHERE {update_where};");
    let insert_values = insert_columns
        .iter()
        .map(|c| {
            if slice_columns.contains(c) {
                syn::parse_str(format!("self.{}.as_ref().map(|v| &v[..])", c).as_str()).unwrap()
            } else {
                syn::parse_str(format!("self.{}", c).as_str()).unwrap()
            }
        })
        .collect::<Vec<syn::Expr>>();
    let update_values = update_columns
        .iter()
        .map(|c| {
            if slice_columns.contains(c) {
                syn::parse_str(format!("self.{}.as_ref().map(|v| &v[..])", c).as_str()).unwrap()
            } else {
                syn::parse_str(format!("self.{}", c).as_str()).unwrap()
            }
        })
        .collect::<Vec<syn::Expr>>();
    let select_query = format!("SELECT * FROM {table};");

    let mut delete_where = Vec::new();
    let mut where_counter = 1;
    for key in primary_keys.iter() {
        delete_where.push(format!("{} = ${}", key, where_counter));
        where_counter += 1;
    }
    let delete_where = delete_where.join(" AND ");
    let delete_values = primary_keys
        .iter()
        .map(|column| syn::parse_str(format!("self.{}", column).as_str()).unwrap())
        .collect::<Vec<syn::Expr>>();
    let delete_query = format!("DELETE FROM {table} WHERE {delete_where};");

    let query_func: Expr = syn::parse_str(if no_type_check_columns.is_empty() {
        "query_as"
    } else {
        "query_as_unchecked"
    })
    .unwrap();

    let get_cache_name: Expr = syn::parse_str(format!("get_{}", cache_name).as_str()).unwrap();
    let insert_cache_name: Expr =
        syn::parse_str(format!("insert_{}", cache_name).as_str()).unwrap();
    let update_cache_name: Expr =
        syn::parse_str(format!("update_{}", cache_name).as_str()).unwrap();
    let delete_cache_name: Expr =
        syn::parse_str(format!("remove_{}", cache_name).as_str()).unwrap();

    let pnwkit_fields = data.fields.iter().filter_map(|field| {
        let alias = field.attrs.iter().find(|attr| attr.path.is_ident("field"));
        let name = field
            .ident
            .as_ref()
            .expect("fields must have an identifier")
            .to_string();
        let field_custom = field
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("field_custom"));
        if name == "lock" || field_custom.is_some() {
            return None;
        }
        Some(if let Some(alias) = alias {
            let alias = alias
                .parse_args::<syn::LitStr>()
                .expect("field attribute must be a string");
            alias.value()
        } else {
            #[allow(clippy::redundant_clone)]
            name.clone()
        })
    });

    let object_exprs: Vec<(String, Option<Expr>)> = data
        .fields
        .iter()
        .map(|field| {
            let alias = field.attrs.iter().find(|attr| attr.path.is_ident("field"));
            let field_custom = field
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("field_custom"));
            let field_no_update = field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("field_no_update"));
            let name = field
                .ident
                .as_ref()
                .expect("fields must have an identifier")
                .to_string();
            let field = if let Some(alias) = alias {
                let alias = alias
                    .parse_args::<syn::LitStr>()
                    .expect("field attribute must be a string");
                alias.value()
            } else {
                #[allow(clippy::redundant_clone)]
                name.clone()
            };
            if name == "lock" {
                return ("lock: std::sync::Arc::new(tokio::sync::Mutex::new(())),".to_string(), None);
            }
            if let Some(field_custom) = field_custom {
                let field_custom = field_custom
                    .parse_args::<syn::LitStr>()
                    .expect("field_custom attribute must be a string")
                    .value();
                (
                    format!(
                        "{}: {},",
                        &name,
                        field_custom.replace(
                            "{get}",
                            format!("o.get(\"{field}\").expect(\"expecting field {field}\").value().into()").as_str()
                        )
                    ),
                    if field_no_update {
                        None
                    } else {
                        Some(
                            syn::parse_str::<Expr>(
                                format!(
                                    "self.{} = {}",
                                    &name,
                                    field_custom.replace(
                                        "{get}",
                                        format!("o.get(\"{field}\").expect(\"expecting field {field}\").value().into()").as_str()
                                    )
                                )
                                .as_str(),
                            )
                            .unwrap()
                        )
                    }
                )
            } else {
                (
                    format!("{name}: o.get(\"{field}\").expect(\"expecting field {field}\").value().into(),"),
                    if field_no_update {
                        None
                    } else {
                        Some(
                            syn::parse_str::<Expr>(
                                format!(
                                    "self.{name} = o.get(\"{field}\").expect(\"expecting field {field}\").value().into()",
                                )
                                .as_str(),
                            )
                            .unwrap()
                        )
                    }
                )
            }
        })
        .collect();
    let create_from_object: Expr = if ast
        .attrs
        .iter()
        .any(|attr| attr.path.is_ident("has_pnwkit"))
    {
        syn::parse_str(
            format!(
                "Self {{ {} }}",
                object_exprs
                    .iter()
                    .map(|(i, _)| i.clone())
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .as_str(),
        )
        .unwrap()
    } else {
        syn::parse_str("unimplemented!()").unwrap()
    };
    let un: Expr = syn::parse_str("unimplemented!()").unwrap();
    let update_from_object = if ast
        .attrs
        .iter()
        .any(|attr| attr.path.is_ident("has_pnwkit"))
    {
        object_exprs
            .iter()
            .filter(|(_, i)| i.is_some())
            .map(|(_, i)| i.as_ref().unwrap())
            .collect::<Vec<_>>()
    } else {
        vec![&un]
    };

    let subscriptions = if let Some(name) = get_option_attr(ast, "subscriptions") {
        let name: Expr = syn::parse_str(name.as_str()).unwrap();
        quote! {

            fn start_subscriptions(d: &crate::structs::data::Data) {
                // CREATE
                let data = d.clone();
                tokio::spawn(async move {
                    let sub = data.kit.subscribe(pnwkit::SubscriptionModel::#name, pnwkit::SubscriptionEvent::Create).await.expect("subscription failed");
                    while let Some(obj) = sub.next().await {
                        let mut value = data.cache.#get_cache_name(&obj.get("id").unwrap().value().as_i32().unwrap());
                        if let Some(mut value) = value {
                            let _lock = value.lock(&data).await;
                            value.update_from_object(obj);
                            if let Err(e) = value.save(&data, false).await {
                                panic!("error saving object: {}", e);
                            }
                        } else {
                            let mut value = #name::create_from_object(obj);
                            if let Err(e) = value.save(&data, true).await {
                                panic!("error saving object: {}", e);
                            }
                        }
                    }
                });
                // UPDATE
                let data = d.clone();
                tokio::spawn(async move {
                    let sub = data.kit.subscribe(pnwkit::SubscriptionModel::#name, pnwkit::SubscriptionEvent::Update).await.expect("subscription failed");
                    while let Some(obj) = sub.next().await {
                        let mut value = data.cache.#get_cache_name(&obj.get("id").unwrap().value().as_i32().unwrap());
                        if let Some(mut value) = value {
                            let _lock = value.lock(&data).await;
                            value.update_from_object(obj);
                            if let Err(e) = value.save(&data, false).await {
                                panic!("error saving object: {}", e);
                            }
                        } else {
                            let mut value = #name::create_from_object(obj);
                            if let Err(e) = value.save(&data, true).await {
                                panic!("error saving object: {}", e);
                            }
                        }
                    }
                });
                // DELETE
                let data = d.clone();
                tokio::spawn(async move {
                    let sub = data.kit.subscribe(pnwkit::SubscriptionModel::#name, pnwkit::SubscriptionEvent::Delete).await.expect("subscription failed");
                    while let Some(obj) = sub.next().await {
                        let mut value = data.cache.#get_cache_name(&obj.get("id").unwrap().value().as_i32().unwrap());
                        if let Some(mut value) = value {
                            let _lock = value.lock(&data).await;
                            if let Err(e) = value.delete(&data).await {
                                panic!("error deleting object: {}", e);
                            }
                        } else {
                            let mut value = #name::create_from_object(obj);
                            if let Err(e) = value.delete(&data).await {
                                panic!("error deleting object: {}", e);
                            }
                        }
                    }
                });
            }
        }
    } else {
        quote! {
            fn start_subscriptions(data: &crate::structs::data::Data) {}
        }
    };

    let lock_cache_name: Expr = syn::parse_str(format!("lock_{}", cache_name).as_str()).unwrap();

    let refresh_from_api = match pnwkit_field {
        Some(pnwkit_field) => quote! {
            use pnwkit::{field, Field};
            println!("REFRESH FROM API {}", stringify!(#name));

            let mut query = data.kit.paginator(field(#pnwkit_field).will_paginate().set_argument("first".to_string(), 500.into())#(.add_field_leaf(#pnwkit_fields))*);
            while let Some(obj) = query.next(data.kit.as_ref()).await {
                let obj: pnwkit::Object = obj.into();
                let mut value = data.cache.#get_cache_name(&obj.get("id").unwrap().value().as_i32().unwrap());
                if let Some(mut value) = value {
                    let _lock = value.lock(&data).await;
                    value.update_from_object(obj);
                    value.save(&data, false).await?;
                } else {
                    let mut value = #name::create_from_object(obj);
                    value.save(&data, true).await?;
                }
            }
            println!("REFRESHED FROM API {}", stringify!(#name));
            Ok(())
        },
        None => quote! {
            Ok(())
        },
    };

    let gen = quote! {
        #[async_trait::async_trait]
        impl crate::traits::Model for #name {
            type Key = #cache_type;
            type Map = dashmap::DashMap<Self::Key, Self>;

            async fn save(&mut self, data: &crate::structs::data::Data, insert: bool) -> Result<(), crate::types::Error> {
                if (insert) {
                    let result = sqlx::#query_func!(#name, #insert_statement, #(#insert_values),*)
                        .fetch_one(data.pool.as_ref())
                        .await?;
                    data.cache.#insert_cache_name(self.#cache_id, self.clone());
                    self.clone_from(&result);
                } else {
                    sqlx::#query_func!(Self, #update_statement, #(#update_values),*)
                        .execute(data.pool.as_ref())
                        .await?;
                    data.cache.#update_cache_name(&self.#cache_id, &self);
                }
                Ok(())
            }

            async fn delete(&self, data: &crate::structs::data::Data) -> Result<(), crate::types::Error> {
                sqlx::query!(#delete_query, #(#delete_values),*)
                    .execute(data.pool.as_ref())
                    .await?;
                data.cache.#delete_cache_name(&self.#cache_id);
                Ok(())
            }

            async fn select_all_as_map(pool: &sqlx::Pool<sqlx::Postgres>) -> Self::Map {
                let res = sqlx::#query_func!(#name, #select_query)
                    .fetch_all(pool)
                    .await
                    .unwrap();
                let map = dashmap::DashMap::with_capacity(res.len());
                for i in res {
                    map.insert(i.#cache_id, i);
                }
                map
            }

            fn create_from_object(o: pnwkit::Object) -> Self {
                #create_from_object
            }

            fn update_from_object(&mut self, o: pnwkit::Object) {
                #(#update_from_object;)*
            }

            #subscriptions

            async fn refresh_from_api(data: &crate::structs::data::Data) -> Result<(), crate::types::Error> {
                #refresh_from_api
            }

            async fn lock(&self, data: &crate::structs::data::Data) -> crate::structs::LockGuard<Self::Key> {
                data.cache.#lock_cache_name(&self.#cache_id).await
            }
        }

        impl std::hash::Hash for #name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.#cache_id.hash(state);
            }
        }

        impl std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.#cache_id == other.#cache_id
            }
        }

        impl std::cmp::Eq for #name {}
    };
    gen.into()
}
