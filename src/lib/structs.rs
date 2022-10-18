use rocket::serde::Deserialize;
use rocket::serde::Serialize;

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