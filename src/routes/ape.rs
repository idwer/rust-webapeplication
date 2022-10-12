use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use crate::lib::ape::ape_index_from_json;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApeIndexInput {
    pub height: u32,
    pub wingspan: u32
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApeIndexOutput {
    pub height: u32,
    pub wingspan: u32,
    pub ape_index: f32
}

#[post("/ape", format = "application/x-www-form-urlencoded", data = "<ape_data>")]
pub fn ape_to_json_post(ape_data: Json<ApeIndexInput>) -> Json<ApeIndexOutput> {
    Json(ape_index_from_json(ape_data))
}