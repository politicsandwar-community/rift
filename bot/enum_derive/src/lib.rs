use proc_macro::TokenStream;
use quote::{quote, ToTokens};

#[proc_macro_derive(Enum)]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_model_derive(&ast)
}

fn impl_model_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let values = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("Enum can only be derived on enums"),
    };
    let variants = values.iter().map(|v| {
        let ident = &v.ident;
        let discriminant = match &v.discriminant {
            Some((_, expr)) => expr,
            None => panic!("Enum discriminant must be specified"),
        };
        quote! {
            #discriminant => Some(Box::new(#name::#ident)),
        }
    });
    let gen = quote! {
        impl crate::traits::Enum for #name {
            fn from_i16(value: i16) -> Option<Box<Self>> {
                match value {
                    #( #variants )*
                    _ => None,
                }
            }
        }
    };
    gen.into()
}
