use actix_web::HttpRequest;
use async_graphql::Request;
use async_graphql_actix_web::GraphQLRequest;
use user_agent_parser::UserAgentParser;

use crate::BearerToken;
use crate::UserAgent;

/// Parse graphql token
pub fn graphql(req: &HttpRequest, gql: GraphQLRequest, uap: &UserAgentParser) -> Request {
    // Create new request context
    let mut request = gql.into_inner();

    // Retrieve bearer token if available
    if let Some(token) = bearer_token(req) {
        request = request.data(token);
    }

    // Retrieve user agent if available
    request = request.data(user_agent(req, uap));

    // Return request
    request
}

/// Parse token header
pub fn bearer_token(req: &HttpRequest) -> Option<BearerToken> {
    if let Some(token) = req.headers().get("Authorization") {
        if let Ok(token) = token.to_str() {
            if let Some(token) = bearer_token_value(token) {
                return Some(BearerToken::new(token));
            }
        }
    }

    None
}

/// Extract bearer token
fn bearer_token_value(auth_header: &str) -> Option<String> {
    const BEARER_PREFIX: &str = "Bearer ";

    // Check if the auth header starts with the "Bearer " prefix
    if let Some(stripped) = auth_header.strip_prefix(BEARER_PREFIX) {
        // Extract the token value after the prefix
        let token = stripped.trim();

        // Check if the token value is not empty
        if !token.is_empty() {
            return Some(token.to_string());
        }
    }

    // Check if the auth header starts with the "bearer " prefix
    if let Some(stripped) = auth_header.strip_prefix(&BEARER_PREFIX.to_lowercase()) {
        // Extract the token value after the prefix
        let token = stripped.trim();

        // Check if the token value is not empty
        if !token.is_empty() {
            return Some(token.to_string());
        }
    }

    None
}

// Retrieve user agent
pub fn user_agent(req: &HttpRequest,  uap: &UserAgentParser) -> UserAgent {
    // Retrieve ip address
    let mut user_agent = UserAgent {
        ip: req
            .connection_info()
            .realip_remote_addr()
            .map( | item| item.to_string()),
        ..Default::default()
    };

    // Get user agent header
    if let Some(ua) = req.headers().get("user-agent") {
        if let Ok(ua) = ua.to_str() {
            let ua_product =  uap.parse_product(ua);
            let ua_os = uap.parse_os(ua);
            let ua_device = uap.parse_device(ua);
            let ua_cpu = uap.parse_cpu(ua);
            let ua_engine = uap.parse_engine(ua);

            // Set product
            user_agent.product.name = ua_product.name.map(|item| item.to_string());
            user_agent.product.major = ua_product.major.map(|item| item.to_string());
            user_agent.product.minor = ua_product.minor.map(|item| item.to_string());
            user_agent.product.patch = ua_product.patch.map(|item| item.to_string());

            // Set os
            user_agent.os.name = ua_os.name.map(|item| item.to_string());
            user_agent.os.major = ua_os.major.map(|item| item.to_string());
            user_agent.os.minor = ua_os.minor.map(|item| item.to_string());
            user_agent.os.patch = ua_os.patch.map(|item| item.to_string());
            user_agent.os.patch_minor = ua_os.patch_minor.map(|item| item.to_string());

            // Set device
            user_agent.device.name = ua_device.name.map(|item| item.to_string());
            user_agent.device.brand = ua_device.brand.map(|item| item.to_string());
            user_agent.device.model = ua_device.model.map(|item| item.to_string());

            // Set architecture
            user_agent.cpu.architecture = ua_cpu.architecture.map(|item| item.to_string());

            // Set engine
            user_agent.engine.name = ua_engine.name.map(|item| item.to_string());
            user_agent.engine.major = ua_engine.major.map(|item| item.to_string());
            user_agent.engine.minor = ua_engine.minor.map(|item| item.to_string());
            user_agent.engine.patch = ua_engine.patch.map(|item| item.to_string());
        }
    }

    // Return user agent
    user_agent
}

pub fn change_ext<S: ToString, E: ToString>(s: S, extension: E) -> String {
    // Create bindings
    let s_bindings = s.to_string();
    let extension_bindings = extension.to_string().to_lowercase().replace('.', "");

    // Modify string
    let mut path = std::path::PathBuf::from(s_bindings);

    // Prune the file name
    let _ = path.set_extension("");

    // Retrieve filename
    let filename = path
        .file_name()
        .map_or(String::default(), |filename| {
            filename
                .to_str()
                .map(|str| str.to_string())
                .unwrap_or_default()
        });

    match extension_bindings.is_empty() {
        true => filename,
        false => format!("{filename}.{extension_bindings}")
    }
}

pub fn ext_from_mime<T: ToString>(value: T) -> &'static str {
    return match value.to_string().to_lowercase().as_str() {
        "audio/aac" => ".aac",
        "application/x-abiword" => ".abw",
        "application/x-freearc" => ".arc",
        "video/x-msvideo" => ".avi",
        "application/vnd.amazon.ebook" => ".azw",
        "application/octet-stream" => ".bin",
        "image/bmp" => ".bmp",
        "application/x-bzip2" => ".bz2",
        "application/x-csh" => ".csh",
        "text/css" => ".css",
        "text/csv" => ".csv",
        "application/msword" => ".doc",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => ".docx",
        "application/vnd.ms-fontobject" => ".eot",
        "application/epub+zip" => ".epub",
        "application/gzip" => ".gz",
        "image/gif" => ".gif",
        "image/avif" => ".avif",
        "text/html" => ".html",
        "image/vnd.microsoft.icon" => ".ico",
        "text/calendar" => ".ics",
        "application/java-archive" => ".jar",
        "image/jpeg" => ".jpg",
        "text/javascript" => ".js",
        "application/json" => ".json",
        "application/ld+json" => ".jsonld",
        "audio/midi" => ".mid",
        "audio/x-midi" => ".midi",
        "audio/mpeg" => ".mp3",
        "video/mpeg" => ".mpeg",
        "application/vnd.apple.installer+xml" => ".mpkg",
        "application/vnd.oasis.opendocument.presentation" => ".odp",
        "application/vnd.oasis.opendocument.spreadsheet" => ".ods",
        "application/vnd.oasis.opendocument.text" => ".odt",
        "audio/ogg" => ".oga",
        "video/ogg" => ".ogv",
        "application/ogg" => ".ogx",
        "audio/opus" => ".opus",
        "font/otf" => ".otf",
        "image/png" => ".png",
        "application/pdf" => ".pdf",
        "application/x-httpd-php" => ".php",
        "application/vnd.ms-powerpoint" => ".ppt",
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => ".pptx",
        "application/vnd.rar" => ".rar",
        "application/rtf" => ".rtf",
        "application/x-sh" => ".sh",
        "image/svg+xml" => ".svg",
        "application/x-shockwave-flash" => ".swf",
        "application/x-tar" => ".tar",
        "image/tiff" => ".tif",
        "video/mp2t" => ".ts",
        "font/ttf" => ".ttf",
        "text/plain" => ".txt",
        "application/vnd.visio" => ".vsd",
        "audio/wav" => ".wav",
        "audio/webm" => ".weba",
        "video/webm" => ".webm",
        "image/webp" => ".webp",
        "font/woff" => ".woff",
        "font/woff2" => ".woff2",
        "application/xhtml+xml" => ".xhtml",
        "application/vnd.ms-excel" => ".xls",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => ".xlsx",
        "application/xml" => ".xml",
        "application/vnd.mozilla.xul+xml" => ".xul",
        "application/zip" => ".zip",
        "video/3gpp" => ".3gp",
        "audio/3gpp" => ".3gp",
        "video/3g2" => ".3g2",
        "audio/3g2" => ".3g2",
        "application/x-7z-compressed" => ".7z",
        _ => ""
    }
}