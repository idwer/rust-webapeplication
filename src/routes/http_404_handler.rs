use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;

// https://www.google.com/search?q=actix+post+return+404
// https://www.google.com/search?q=actix+default_service+return+404+in+browser
// https://users.rust-lang.org/t/error-404-unfound-routes-actix-web/46484/3
pub async fn handle_404() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
    .content_type("text/html; charset=utf-8")
    .body(StatusCode::NOT_FOUND.to_string()))
}