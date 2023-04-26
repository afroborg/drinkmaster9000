#![allow(special_module_name)]

use std::{thread, time::Duration};

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
    // print some info
    println!("Running server on [::]:8080");
    println!("Running on a {}.", DeviceInfo::new().unwrap().model());

    // set up logging
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // wait for 1 second after bootup
    thread::sleep(Duration::from_millis(1000));

    // set up the global app config
    let config = lib::config::Config::new_mutex();
    let state: State = web::Data::new(config);

    HttpServer::new(move || {
        // setup cors
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        // get the static files built from the astro project
        let files_handler = Files::new("/", "/srv/drinkmixer/static")
            .index_file("index.html")
            .prefer_utf8(true);

        // set up the api routes so that they are all under /api
        let api_scope = web::scope("/api")
            .service(routes::drinks::drinks_scope())
            .service(routes::dispenser::dispenser_scope());

        // set up the websocket routes so that they are all under /ws
        let ws_scope = routes::ws::ws_scope();

        App::new()
            .wrap(cors)
            .app_data(web::Data::clone(&state))
            .service(api_scope)
            .service(ws_scope)
            // add the static files handler
            // this should always be last so that it doesn't interfere with any api routes
            .service(files_handler)
    })
    .bind("[::]:8080")?
    .run()
    .await
}
