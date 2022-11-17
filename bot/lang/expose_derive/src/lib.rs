use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Expose, attributes(expose, expose_as))]
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
    let exposed = data
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
    let gen = quote! {
        impl lang::Expose for #name {
            fn get_attr(&self, _ctx: &lang::Context, ident: &str) -> lang::ValueResult {
                match ident {
                    #( #exposed )*
                    _ => Err(lang::RuntimeError::AttributeNotFound(ident.to_string())),
                }
            }
        }

        impl From<#name> for lang::Value {
            fn from(value: #name) -> Self {
                lang::Value::AttrVar(lang::Var::new(value))
            }
        }
    };
    gen.into()
}
