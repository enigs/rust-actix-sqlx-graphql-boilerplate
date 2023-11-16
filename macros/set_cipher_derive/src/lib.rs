use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_set_cipher_trait(ast: DeriveInput) -> TokenStream {
    // Get struct identifier
    let ident = ast.ident;

    // Generate impl
    quote::quote! {
        impl #ident {
            pub fn encrypt(&self) -> anyhow::Result<Self> {
                let mut data = self.clone();

                for string in data.get_ciphers() {
                    *string = crate::Cipher::from(string.clone())
                        .set_as_decrypted()
                        .encrypt()?
                        .b64encode()?;
                }

                Ok(data)
            }

            pub fn decrypt(&self) -> anyhow::Result<Self> {
                let mut data = self.clone();

                for string in data.get_ciphers() {
                    *string = crate::Cipher::from(string.clone())
                        .decrypt()?
                        .to_string()?;
                }

                Ok(data)
            }
        }
    }
    .into()
}

#[proc_macro_derive(SetCipher)]
pub fn set_cipher_derive_macro(item: TokenStream) -> TokenStream {
    // Parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate
    impl_set_cipher_trait(ast)
}