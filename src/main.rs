use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::Logger;

mod lib;
mod routes;

use crate::routes::ape_get::ape_get_show_usage;
use crate::routes::ape_post::ape_to_json_post;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
     env_logger::init();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .service(ape_get_show_usage)
        .service(ape_to_json_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}