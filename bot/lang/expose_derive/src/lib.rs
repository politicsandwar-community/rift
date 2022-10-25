use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Expose, attributes(expose))]
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
        .filter(|field| field.attrs.iter().any(|attr| attr.path.is_ident("expose")))
        .map(|field| {
            let ident = field.ident.as_ref().expect("field must have an identifier");
            let string = ident.to_string();
            quote! {
                #string => Ok(self.#ident.into()),
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
    };
    gen.into()
}
