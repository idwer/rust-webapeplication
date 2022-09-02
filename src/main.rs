#[macro_use] extern crate rocket;

#[get("/")]
fn ape_to_json() -> &'static str {
    "placeholder response for GET"
}

#[post("/")]
fn ape_to_json_post() -> &'static str {
    "placeholder response for POST request"
}

#[launch]
fn webapi() -> _ {
    rocket::build().mount("/api", routes![ape_to_json, ape_to_json_post])
}