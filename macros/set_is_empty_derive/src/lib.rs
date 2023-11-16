use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_set_is_empty_trait(ast: DeriveInput) -> TokenStream {
    // Get struct identifier
    let ident = ast.ident;

    if let syn::Data::Struct(_) = ast.data {
        return quote::quote! {
            impl #ident {
                fn is_empty(&self) -> bool {
                    *self == Self::default()
                }
            }
        }
            .into()
    }

    panic!("Only structs are supported by SetIsEmpty derive.");
}

#[proc_macro_derive(SetIsEmpty)]
pub fn set_is_empty_derive_macro(item: TokenStream) -> TokenStream {
    // Parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate
    impl_set_is_empty_trait(ast)
}