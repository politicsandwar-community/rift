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
        cache_type,
        cache_unwrap
    )
)]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_model_derive(&ast)
}

fn get_table_attr(ast: &syn::DeriveInput) -> String {
    match ast.attrs.iter().find(|attr| attr.path.is_ident("table")) {
        Some(attr) => match attr.parse_meta().unwrap() {
            syn::Meta::NameValue(meta) => match meta.lit {
                syn::Lit::Str(lit) => lit.value(),
                _ => panic!("table attribute must be a string literal"),
            },
            _ => panic!("table attribute must be a string literal"),
        },
        None => panic!("Model derives must have a table attribute"),
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
    let table = get_table_attr(ast);
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
    let columns: Vec<String> = data
        .fields
        .iter()
        .filter(|field| !field.attrs.iter().any(|attr| attr.path.is_ident("no_save")))
        .map(|field| {
            let no_type_check = field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("no_type_check"));
            let column = field
                .ident
                .as_ref()
                .expect("fields must have an identifier")
                .to_string();
            if no_type_check {
                no_type_check_columns.push(column.clone());
            }
            column
        })
        .collect();
    let insert_names = columns.join(", ");
    let insert_values: Vec<String> = (0..columns.len()).map(|i| format!("${}", i + 1)).collect();
    let insert_values = insert_values.join(", ");
    let returning_names = columns
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
    let update_names = Iterator::enumerate(columns.iter())
        .map(|(i, column)| format!("{} = ${}", column, i + 1))
        .collect::<Vec<String>>()
        .join(", ");
    let update_where = primary_keys
        .iter()
        .map(|key| {
            format!(
                "{} = ${}",
                key,
                columns
                    .iter()
                    .position(|c| c == key)
                    .unwrap_or_else(|| panic!("primary key {} not present in struct", key))
                    + 1
            )
        })
        .collect::<Vec<String>>()
        .join(" AND ");
    let update_statement = format!("UPDATE {table} SET {update_names} WHERE {update_where};");
    let struct_values = columns
        .iter()
        .map(|column| syn::parse_str(format!("self.{}", column).as_str()).unwrap())
        .collect::<Vec<syn::Expr>>();
    let select_query = format!("SELECT * FROM {table};");
    let query_func: Expr = syn::parse_str(if no_type_check_columns.is_empty() {
        "query_as"
    } else {
        "query_as_unchecked"
    })
    .unwrap();
    let gen = quote! {
        #[async_trait::async_trait]
        impl crate::traits::Model for #name {
            const TABLE: &'static str = #table;
            type Key = #cache_type;
            type Map = dashmap::DashMap<Self::Key, Self>;

            async fn save(&mut self, data: &crate::structs::data::Data, insert: bool) -> Result<(), crate::types::Error> {
                if (insert) {
                    let result = sqlx::#query_func!(Self, #insert_statement, #(#struct_values),*)
                        .fetch_one(&data.pool)
                        .await?;
                    self.clone_from(&result);
                } else {
                    sqlx::#query_func!(Self, #update_statement, #(#struct_values),*)
                        .fetch_one(&data.pool)
                        .await?;
                }
                Ok(())
            }

            async fn select_all_as_map(pool: &sqlx::Pool<sqlx::Postgres>) -> Self::Map {
                let res = sqlx::#query_func!(Self, #select_query)
                    .fetch_all(pool)
                    .await
                    .unwrap();
                let map = dashmap::DashMap::with_capacity(res.len());
                for i in res {
                    map.insert(i.#cache_id, i);
                }
                map
            }
        }
    };
    gen.into()
}
