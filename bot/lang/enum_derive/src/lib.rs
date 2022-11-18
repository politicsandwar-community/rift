use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Enum)]
pub fn enum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enum_derive(&ast)
}

fn impl_enum_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let values = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("Enum can only be derived on enums"),
    };
    let from_variants = values.iter().map(|v| {
        let ident = &v.ident;
        let discriminant = match &v.discriminant {
            Some((_, expr)) => expr,
            None => panic!("Enum discriminant must be specified"),
        };
        quote! {
            #discriminant => Some(Box::new(#name::#ident)),
        }
    });
    let to_variants = values.iter().map(|v| {
        let ident = &v.ident;
        let discriminant = match &v.discriminant {
            Some((_, expr)) => expr,
            None => panic!("Enum discriminant must be specified"),
        };
        quote! {
            #name::#ident => #discriminant,
        }
    });
    let static_attrs = values.iter().map(|v| {
        let ident = &v.ident;
        let string = ident.to_string();
        quote! {
            #string => Ok(#name::#ident.into()),
        }
    });
    let gen = quote! {
        impl lang::Enum for #name {
            fn from_i16(value: i16) -> Option<Box<Self>> {
                match value {
                    #( #from_variants )*
                    _ => None,
                }
            }

            fn to_i16(&self) -> i16 {
                match self {
                    #( #to_variants )*
                }
            }
        }

        impl lang::Expose for #name {
            fn get_static_attr(&self, _ctx: &lang::Context, ident: &str) -> lang::ValueResult {
                match ident {
                    #( #static_attrs )*
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
