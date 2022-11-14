use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::Logger;
use actix_web::web;

use std::env;

mod lib;
mod routes;

use crate::routes::ape_get::ape_get_show_usage;
use crate::routes::http_404_handler::handle_404;
use crate::routes::ape_post::ape_to_json_post;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");

    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .default_service(
            web::route().to(handle_404)
        )
        // Now we can store endpoints in one place:
        .route("/ape", web::get().to(ape_get_show_usage))
        .route("/ape", web::post().to(ape_to_json_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}