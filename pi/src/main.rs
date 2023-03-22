#![allow(special_module_name)]

use crate::lib::config::State;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use rppal::system::DeviceInfo;

mod lib;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running server on [::]:8080");
    println!("Running on a {}.", DeviceInfo::new().unwrap().model());

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config = lib::config::Config::new_mutex();
    let state: State = web::Data::new(config);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        let files_handler = Files::new("/", "/srv/drinkmixer/static")
            .index_file("index.html")
            .prefer_utf8(true);

        let api_scope = web::scope("/api")
            .service(routes::drinks::drinks_scope())
            .service(routes::dispenser::dispenser_scope())
            .service(routes::config::config_scope());

        App::new()
            .wrap(cors)
            .app_data(web::Data::clone(&state))
            .service(api_scope)
            .service(routes::ws::ws_scope())
            // this should always be last
            .service(files_handler)
    })
    .bind("[::]:8080")?
    .run()
    .await
}
