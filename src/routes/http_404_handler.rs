use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;

pub async fn handle_404() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
    .content_type("text/html; charset=utf-8")
    .body(StatusCode::NOT_FOUND.to_string()))
}