use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web::Json;
use validator::Validate;

use crate::lib::ape::ApeIndexInput;

pub async fn ape_to_json_post(ape_data: Json<ApeIndexInput>) -> impl Responder {
    let input = ape_data.validate();

    match input {
        Ok(_) => {
            let input = ApeIndexInput{
                height: ape_data.height,
                wingspan: ape_data.wingspan,
            };

            HttpResponse::Ok()
            .content_type("application/json")
            .json(input.ape_index_from_json())
        },

        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}