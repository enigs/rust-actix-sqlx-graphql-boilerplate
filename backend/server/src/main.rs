pub(crate) mod pages;

use actix::Actor;
use actix_web::{App, HttpServer, web::{self, Data}};
use actix_web::middleware::{Compress, Logger, NormalizePath, TrailingSlash};
use autometrics::prometheus_exporter;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

use library::Core;
use library::ActixTokenParser;
use library::scheduler::Scheduler;

// Do not use async or framework's macro here as per sentry's instruction
// https://docs.sentry.io/platforms/rust/
// Lazy initialize sentry
static SENTRY: Lazy<Option<sentry::ClientInitGuard>> = Lazy::new(|| {
    match config::SENTRY_URL.is_empty() {
        true => None::<sentry::ClientInitGuard>,
        false => Some(sentry::init((config::SENTRY_URL, sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        })))
    }
});

// Lazy initialize broadcaster
static BROADCASTER: Lazy<Arc<library::sse::Broadcaster>> = Lazy::new(|| {
    library::sse::Broadcaster::create()
});

// Async Multi-Threaded Main
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Access lazy initialization of sentry
    if let Some(sentry) = &*SENTRY {
        let _ = sentry;
    }

    // Set sse broadcaster
    let broadcaster = &*BROADCASTER;

    // Initialize log tracer
    if let Err(logger) = tracing_log::LogTracer::init() {
        sentry::capture_error(&logger);
    }

    // Set tracing subscriber
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(
        config::app_name(),
        non_blocking_writer
    );

    // Initialize tracing
    use tracing_subscriber::layer::SubscriberExt;
    let subscriber = tracing_subscriber::Registry::default()
        .with(config::env_filter())
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(bunyan_formatting_layer);

    // Set global default subscriber
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Initialize autometrics
    prometheus_exporter::init();

    // Initialize core library
    let core = Core::init()
        .await
        .expect("Core library failed to initialize...");

    // Set scheduler core
    Scheduler::builder()
        .set_core(&core)
        .set_duration(config::CRON_DURATION)
        .clone()
        .start();

    // Retrieve graphql schemas
    let schema = resolver::schema(&core, broadcaster);

    // Instantiate actix web server
    HttpServer::new(move || {
        App::new()
            // Include app data
            .app_data(Data::new(Arc::clone(&core)))
            .app_data(Data::new(Arc::clone(broadcaster)))
            .app_data(Data::new(schema.clone()))

            // Set cors
            .wrap(config::cors())

            // Wrap middlewares
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Always))

            // Create metrics endpoint
            .service(web::scope("/metrics")
                .wrap(ActixTokenParser::controller())
                .service(pages::metrics)
                .default_service(web::route().to(config::page::async_not_found)))

            // Include routes
            .service(
                web::scope(config::BASE_PATH)
                    .wrap(Compress::default())
                    .service(pages::health_check)
                    .service(pages::favicon)
                    .service(pages::events)
                    .service(pages::broadcast)
                    .service(pages::static_files())
                    .service(pages::playground())
                    .service(pages::resolvers())
                    .default_service(web::route().to(config::page::async_not_found))
            )

            // Set default service
            .default_service(web::route().to(config::page::async_not_found))
    })
        .bind((config::SERVER_ADDRESS, config::SERVER_PORT))?
        .run()
        .await
}
