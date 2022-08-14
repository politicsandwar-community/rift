use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Model, attributes(table, no_save, primary_key))]
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
    let columns: Vec<String> = data
        .fields
        .iter()
        .filter(|field| !field.attrs.iter().any(|attr| attr.path.is_ident("no_save")))
        .map(|field| {
            field
                .ident
                .as_ref()
                .expect("fields must have an identifier")
                .to_string()
        })
        .collect();
    let insert_names = columns.join(", ");
    let insert_values: Vec<String> = (0..columns.len()).map(|i| format!("${}", i + 1)).collect();
    let insert_values = insert_values.join(", ");
    let insert_statement = format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING *;",
        table, insert_names, insert_values,
    );
    let update_statement = format!(
        "UPDATE {} SET {} WHERE {};",
        table,
        Iterator::enumerate(columns.iter())
            .map(|(i, column)| format!("{} = ${}", column, i + 1))
            .collect::<Vec<String>>()
            .join(", "),
        primary_keys
            .iter()
            .map(|key| format!(
                "{} = ${}",
                key,
                columns
                    .iter()
                    .position(|c| c == key)
                    .unwrap_or_else(|| panic!("primary key {} not present in struct", key))
                    + 1
            ))
            .collect::<Vec<String>>()
            .join(" AND ")
    );
    let struct_values = columns
        .iter()
        .map(|column| syn::parse_str(format!("self.{}", column).as_str()).unwrap())
        .collect::<Vec<syn::Expr>>();
    let gen = quote! {
        #[async_trait::async_trait]
        impl crate::traits::Model for #name {
            const TABLE: &'static str = #table;

            async fn save(&mut self, data: &crate::structs::data::Data, insert: bool) -> Result<(), crate::types::Error> {
                if (insert) {
                    let result = sqlx::query_as!(Self, #insert_statement, #(#struct_values),*)
                        .fetch_one(&data.pool)
                        .await?;
                    self.clone_from(&result);
                } else {
                    sqlx::query_as!(Self, #update_statement, #(#struct_values),*)
                        .fetch_one(&data.pool)
                        .await?;
                }
                Ok(())
            }
        }
    };
    gen.into()
}
