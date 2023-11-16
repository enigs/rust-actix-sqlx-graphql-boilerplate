pub mod cors;
pub mod env;
pub mod page;
pub mod template;

use slugify::slugify;

pub use cors::cors;
pub use env::env_filter;
pub use template::template;

/// App related variables
pub fn app_name() -> String {
    if let Ok(name) = std::env::var("APP_NAME") {
        return slugify!(&name);
    }

    slugify!("Rust Server")
}

/// Base path variable for routes
pub const BASE_PATH: &str = "";

/// Set address
pub const SERVER_ADDRESS: &str = "0.0.0.0";
pub const SERVER_PORT: u16 = 9020; // Set your port here

/// Set assets variables
pub const FROM_FAVICON: &str = "/favicon.ico";
pub const FROM_STATIC: &str = "/static";

pub const PATH_FAVICON: &str = "assets/static/media/favicon.ico";
pub const PATH_STATIC: &str = "assets/static";
pub const PATH_ERROR_404: &str = "assets/templates/errors/404.html";

/// Set attempt variables
pub const ATTEMPT_RETRY_MAX: usize = 5;
pub const ATTEMPT_RETRY_DURATION: usize = 30;

/// Set catchers variables
pub const CATCHER_CACHE_DIRECTIVES: u32 = 86400u32;
pub const CATCHER_MIME_HTML: &str = "text/html; charset=utf-8";
pub const CATCHER_TEMPLATE_404_PATH: &str = "errors/404.html";

/// CORS related variables
pub const CORS_METHODS: [&str; 5] = ["GET", "POST", "PATCH", "DELETE", "OPTIONS"];

/// Cron related variables
// pub const CRON_DURATION: &str = "0 0 0-23 * * * *"; // Every Hour
pub const CRON_DURATION: &str = "0 0 */1 * * * *"; // Every Hour
// pub const CRON_DURATION: &str = "0 0-59 * * * * *"; // every minute
// pub const CRON_DURATION: &str = "0 */1 * * * * *"; // every minute
// pub const CRON_DURATION: &str = "0 */2 * * * * *"; // every 2 minutes

/// Set handlebars variables
pub const HANDLEBARS_ASSET_PATH: &str = "./assets/templates";
pub const HANDLEBARS_EXTENSION: &str = ".hbs";

/// Locales related variables
pub const LOCALES_PATH: &str = "./assets/locales/";
pub const LOCALES_US: &str = "en-US";

/// Mailer related variables
pub const MAILER_FROM_HELLO: &str = "My Server <hello@my-server.com>";
pub const MAILER_FROM_NO_REPLY: &str = "My Server <no-reply@my-server.com>";
pub const MAILER_FROM_SUCCESS: &str = "My Server <success@my-server.com>";
pub const MAILER_TO_CONTROLLER: &str = "markhenry.liwag@gmail.com";

/// Paseto defaults
pub const PASETO_ACCESS_TOKEN_KEY_UNIT: &str = "120";
pub const PASETO_ACCESS_TOKEN_KEY_TIME: &str = "Days";
pub const PASETO_ACCESS_TOKEN_KEY_SIGNING: &str = ""; // Use generator here
pub const PASETO_REFRESH_TOKEN_KEY_UNIT: &str = "180";
pub const PASETO_REFRESH_TOKEN_KEY_TIME: &str = "Days";
pub const PASETO_REFRESH_TOKEN_KEY_SIGNING: &str = ""; // Use generator here

/// Sentry related variables
pub const SENTRY_URL: &str = "";

/// User Agent Parser related variables
pub const USER_AGENT_REGEXES: &str = "./assets/regexes.yaml";