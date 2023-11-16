pub fn cors() -> actix_cors::Cors {
    // Set bindings
    let binding = crate::CORS_METHODS.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let methods: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

    // Return cors
    actix_cors::Cors::default()
        .allow_any_origin()
        .allowed_methods(methods)
        .allow_any_header()
        .max_age(3600)
}