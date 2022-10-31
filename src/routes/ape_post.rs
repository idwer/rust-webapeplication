use actix_web::post;
use actix_web::HttpResponse;
use actix_web::Responder;

use actix_web::web::Json;

use crate::lib::ape::ape_index_from_json;

use crate::lib::structs::ApeIndexInput;

#[post("/ape")]
async fn ape_to_json_post(ape_data: Json<ApeIndexInput>) -> impl Responder {
    HttpResponse::Ok()
    .content_type("application/json")
    .json(ape_index_from_json(ape_data))
}