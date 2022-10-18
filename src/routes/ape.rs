use actix_web::post;
use actix_web::HttpResponse;
use actix_web::Responder;

use actix_web::web::Json;

use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::lib::ape::ape_index_from_json;

#[derive(Deserialize, Serialize)]
pub struct ApeIndexInput {
    pub height: u32,
    pub wingspan: u32
}

#[derive(Deserialize, Serialize)]
pub struct ApeIndexOutput {
    pub height: u32,
    pub wingspan: u32,
    pub ape_index: f32
}

#[post("/ape")]
async fn ape_to_json_post(ape_data: Json<ApeIndexInput>) -> impl Responder {
    HttpResponse::Ok().json(ape_index_from_json(ape_data))
}