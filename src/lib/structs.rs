use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct ApeIndexInput {
    #[validate(range(min = 1, max = 300, message = "Expected numerical input for height should be between 1 and 300"))]
    pub height: i16,
    #[validate(range(min = 1, max = 300, message = "Expected numerical input for wingspan should be between 1 and 300"))]
    pub wingspan: i16,
}

#[derive(Deserialize, Serialize)]
pub struct ApeIndexOutput {
    pub height: i16,
    pub wingspan: i16,
    pub ape_index: f32,
}
