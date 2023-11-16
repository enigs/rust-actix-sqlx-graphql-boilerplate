use actix_web::{HttpResponse, Result};

pub fn not_found() -> HttpResponse {
    let body = crate::template::template()
        .render(crate::CATCHER_TEMPLATE_404_PATH, &None::<String>)
        .expect("Invalid template directory");

    HttpResponse::build(actix_web::http::StatusCode::NOT_FOUND)
        .insert_header(actix_web::http::header::CacheControl(vec![
            actix_web::http::header::CacheDirective::Public,
            actix_web::http::header::CacheDirective::MaxAge(crate::CATCHER_CACHE_DIRECTIVES),
        ]))
        .content_type(crate::CATCHER_MIME_HTML)
        .body(body)
}

pub async fn async_not_found() -> Result<HttpResponse> {
    Ok(not_found())
}