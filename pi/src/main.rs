#![allow(special_module_name)]
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

    let state = lib::state::AppState::new();
    let data = web::Data::new(state);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        let files_handler = Files::new("/", "/srv/drinkmixer/static")
            .index_file("index.html")
            .prefer_utf8(true);

        let api_scope = web::scope("/api").service(routes::dispensers::dispenser_scope());

        App::new()
            .wrap(cors)
            .app_data(web::Data::clone(&data))
            .service(api_scope)
            // this should always be last
            .service(files_handler)
    })
    .bind("[::]:8080")?
    .run()
    .await
}
