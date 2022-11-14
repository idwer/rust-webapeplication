use actix_web::post;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web;
use actix_web::web::Json;
use validator::Validate;

use crate::lib::ape::ape_index_from_json;
use crate::lib::structs::ApeIndexInput;

pub async fn ape_to_json_post(ape_data: web::Json<ApeIndexInput>) -> impl Responder {
    let input = ape_data.validate();

    match input {
        Ok(_) => HttpResponse::Ok()
        .content_type("application/json")
        .json(ape_index_from_json(ape_data)),

        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}