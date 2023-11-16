pub fn env_filter() -> tracing_subscriber::EnvFilter {
    let level = match std::env::var("TRACING_LEVEL") {
        Ok(level) => level,
        Err(_) => "error".to_string()
    };

    tracing_subscriber::EnvFilter::from(level)
}