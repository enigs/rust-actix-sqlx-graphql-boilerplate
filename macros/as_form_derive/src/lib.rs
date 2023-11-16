use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(form))]
struct FormStructAttributes {
    to: syn::ExprPath,
    #[deluxe(default = None)]
    error: Option<String>
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(conversion))]
struct ConversionFieldAttributes(syn::ExprPath);

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(error))]
struct ErrorFieldAttributes(syn::Type);

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(sanitize))]
struct SanitizeFieldAttributes(syn::ExprPath);

fn extract_conversion_field_attrs(
    ast: &mut DeriveInput
) -> deluxe::Result<std::collections::HashMap<syn::Ident, ConversionFieldAttributes>> {
    let mut field_attrs: std::collections::HashMap<
        syn::Ident,
        ConversionFieldAttributes
    > = std::collections::HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            if let Ok(attrs) = deluxe::extract_attributes(field) {
                field_attrs.insert(field.ident.as_ref().unwrap().clone(), attrs);
            }
        }
    }

    Ok(field_attrs)
}

fn extract_error_field_attrs(
    ast: &mut DeriveInput
) -> deluxe::Result<std::collections::HashMap<syn::Ident, ErrorFieldAttributes>> {
    let mut field_attrs: std::collections::HashMap<
        syn::Ident,
        ErrorFieldAttributes
    > = std::collections::HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            if let Ok(attrs) = deluxe::extract_attributes(field) {
                field_attrs.insert(field.ident.as_ref().unwrap().clone(), attrs);
            }
        }
    }

    Ok(field_attrs)
}

fn extract_sanitize_field_attrs(
    ast: &mut DeriveInput
) -> deluxe::Result<std::collections::HashMap<syn::Ident, SanitizeFieldAttributes>> {
    let mut field_attrs: std::collections::HashMap<
        syn::Ident,
        SanitizeFieldAttributes
    > = std::collections::HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            if let Ok(attrs) = deluxe::extract_attributes(field) {
                field_attrs.insert(field.ident.as_ref().unwrap().clone(), attrs);
            }
        }
    }

    Ok(field_attrs)
}


fn as_form_derive_macro2(
    item: proc_macro2::TokenStream
) -> deluxe::Result<proc_macro2::TokenStream> {
    // Parse
    let mut ast: DeriveInput = syn::parse2(item)?;

    // Extract conversion field attributes
    let field_attrs: std::collections::HashMap<
        syn::Ident,
        ConversionFieldAttributes
    > = extract_conversion_field_attrs(&mut ast)?;

    let (conversion_field_name, conversion): (Vec<syn::Ident>, Vec<syn::ExprPath>) = field_attrs
        .into_iter()
        .map(|(field, conversion)|  (field, conversion.0))
        .unzip();

    // Extract conversion field attributes
    let field_attrs: std::collections::HashMap<
        syn::Ident,
        ErrorFieldAttributes
    > = extract_error_field_attrs(&mut ast)?;

    let (error_field_name, error_type): (Vec<syn::Ident>, Vec<syn::Type>) = field_attrs
        .into_iter()
        .map(|(field, error_type)|  (field, error_type.0))
        .unzip();

    // Extract sanitize field attributes
    let field_attrs: std::collections::HashMap<
        syn::Ident,
        SanitizeFieldAttributes
    > = extract_sanitize_field_attrs(&mut ast)?;

    let (sanitize_field_name, sanitize): (Vec<syn::Ident>, Vec<syn::ExprPath>) = field_attrs
        .into_iter()
        .map(|(field, sanitize)|  (field, sanitize.0))
        .unzip();

    // Extract struct attributes
    let FormStructAttributes {
        to ,
        error
    } = deluxe::extract_attributes(&mut ast)?;

    // Define impl variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // Generate
    let mut generated = quote::quote! {
        impl #impl_generics AsForm for #ident #type_generics #where_clause { }

        impl From<#ident> for #to {
            fn from(form: #ident) -> Self {
                let mut data = Self::default();

                #(
                    data.#conversion_field_name = #conversion(form.#conversion_field_name);
                )*

                data
            }
        }

        impl #ident {
            pub fn sanitize(&mut self) -> &mut Self {
                #(
                    self.#sanitize_field_name = #sanitize(&self.#sanitize_field_name);
                )*

                self
            }

            pub fn to<T: From<Self>>(&self) -> T {
                T::from(self.clone())
            }
        }
    };

    if let Some(error) = error {
        let error = syn::Ident::new(&error, proc_macro2::Span::call_site());

        generated.extend(quote::quote! {
            #[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #error {
                #(
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #error_field_name: Option<#error_type>
                ),*
            }

            impl #error {
                 pub fn is_empty(&self) -> bool {
                    *self == Self::default()
                }
            }
        });
    }

    Ok(generated)
}

#[proc_macro_derive(AsForm, attributes(form, conversion, sanitize, error))]
pub fn as_form_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    as_form_derive_macro2(item.into()).unwrap().into()
}