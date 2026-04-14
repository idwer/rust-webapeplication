use actix_web::HttpResponse;
use actix_web::Responder;

pub async fn ape_get_show_usage() -> impl Responder {
    let body = concat!("Using the endpoint, height and wingspan are in centimeter:<br>curl --header ", r#"'Content-Type: application/json' http://127.0.0.1:3000/ape -X POST -d '{"height": 200, "wingspan": 200}'"#);

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(body)
}
