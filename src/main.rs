#[macro_use] extern crate rocket;

mod lib;
mod routes;

use routes::ape::ape_to_json_post;

#[launch]
fn webapi() -> _ {
    rocket::build().mount("/api", routes![ape_to_json_post])
}