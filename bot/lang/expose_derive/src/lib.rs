use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

#[proc_macro_derive(Expose, attributes(expose, expose_as, expose_method, expose_property))]
pub fn expose_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_expose_derive(&ast)
}

fn impl_expose_derive(ast: &syn::DeriveInput) -> TokenStream {
    let data = match &ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Expose can only be derived on structs"),
    };
    let name = &ast.ident;
    let exposed_attributes = data
        .fields
        .iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("expose") || attr.path.is_ident("expose_as"))
        })
        .map(|field| {
            let ident = field.ident.as_ref().expect("field must have an identifier");
            let string = if let Some(expose_as) = field
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("expose_as"))
            {
                match &expose_as.parse_meta().unwrap() {
                    syn::Meta::NameValue(syn::MetaNameValue {
                        lit: syn::Lit::Str(lit),
                        ..
                    }) => lit.value(),
                    _ => panic!("expose_as must be a string literal"),
                }
            } else {
                ident.to_string()
            };
            quote! {
                #string => Ok((&self.#ident).into()),
            }
        });
    let exposed_methods = ast
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("expose_method"))
        .map(|attr| {
            let s = match &attr.parse_meta().unwrap() {
                syn::Meta::NameValue(syn::MetaNameValue {
                    lit: syn::Lit::Str(lit),
                    ..
                }) => lit.value(),
                _ => panic!("expose_method must be a string literal"),
            };
            let (ident, string) = if s.contains(" as ") {
                let segments = s.split(" as ").collect::<Vec<_>>();
                if segments.len() != 2 {
                    panic!("expose_method with alias must be of the form \"ident as string\"");
                }
                (segments[0], segments[1])
            } else {
                (s.as_str(), s.as_str())
            };
            let ident: Ident = syn::parse_str(ident).unwrap();
            quote! {
                #string => Ok((#name::#ident).into()),
            }
        });
    let exposed_properties = ast
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("expose_property"))
        .map(|attr| {
            let s = match &attr.parse_meta().unwrap() {
                syn::Meta::NameValue(syn::MetaNameValue {
                    lit: syn::Lit::Str(lit),
                    ..
                }) => lit.value(),
                _ => panic!("expose_property must be a string literal"),
            };
            let (ident, string) = if s.contains(" as ") {
                let segments = s.split(" as ").collect::<Vec<_>>();
                if segments.len() != 2 {
                    panic!("expose_property with alias must be of the form \"ident as string\"");
                }
                (segments[0], segments[1])
            } else {
                (s.as_str(), s.as_str())
            };
            let ident: Ident = syn::parse_str(ident).unwrap();
            quote! {
                #string => self.#ident().into(),
            }
        });
    let gen = quote! {
        impl lang::Expose for #name {
            fn get_attr(&self, _ctx: &lang::Context, ident: &str) -> lang::ValueResult {
                match ident {
                    #( #exposed_attributes )*
                    #( #exposed_properties)*
                    _ => Err(lang::RuntimeError::AttributeNotFound(ident.to_string())),
                }
            }

            fn get_static_attr(&self, _ctx: &lang::Context, ident: &str) -> lang::ValueResult {
                match ident {
                    #( #exposed_methods )*
                    _ => Err(lang::RuntimeError::StaticAttributeNotFound(ident.to_string())),
                }
            }
        }

        impl From<#name> for lang::Value {
            fn from(value: #name) -> Self {
                lang::Value::AttrVar(lang::Var::new(value))
            }
        }

        impl From<&#name> for lang::Value {
            fn from(value: &#name) -> Self {
                lang::Value::AttrVar(lang::Var::new(value.clone()))
            }
        }
    };
    gen.into()
}
