use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

#[get("/ape")]
async fn ape_get_show_usage() -> impl Responder {
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body("Using the endpoint, height and wingspan are in centimeter:\ncurl --header \'Content-Type: application/json\' http://127.0.0.1:8080/ape -X POST -d \'{\"height\": 200, \"wingspan\": 200}\'")
}