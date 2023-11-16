use handlebars::Handlebars;

/// Retrieve template
pub fn template() -> Handlebars<'static> {
    let mut templ = Handlebars::new();

    templ.register_templates_directory(
        crate::HANDLEBARS_EXTENSION,
        crate::HANDLEBARS_ASSET_PATH
    ).expect("Invalid template directory");

    templ
}