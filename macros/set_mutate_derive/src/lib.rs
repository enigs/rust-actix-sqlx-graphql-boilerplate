use proc_macro::TokenStream;
use syn::{DeriveInput, Ident};

fn impl_set_mutate_trait(ast: DeriveInput) -> TokenStream {
    // Get struct identifier
    let ident = ast.ident;

    // Get field identifiers
    let field_idents: Vec<Ident> = match ast.data {
        syn::Data::Struct(data) => data.fields.into_iter().filter_map(|f| f.ident).collect(),
        syn::Data::Enum(_) => panic!("Enums are not supported by set_mutate."),
        syn::Data::Union(_) => panic!("Unions are not supported by set_mutate.")
    };

    // Generate impl
    quote::quote! {
        impl #ident {
            pub fn mutate(&mut self, form: &Self) -> &mut Self {
                #(
                    self.#field_idents = form.#field_idents.clone();
                )*

                self
            }
        }
    }
    .into()
}

#[proc_macro_derive(SetMutate)]
pub fn set_mutate_derive_macro(item: TokenStream) -> TokenStream {
    // Parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate
    impl_set_mutate_trait(ast)
}