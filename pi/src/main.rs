#![allow(special_module_name)]
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
        let files_handler = Files::new("/", "/srv/drinkmixer/static")
            .index_file("index.html")
            .show_files_listing();

        App::new()
            .app_data(web::Data::clone(&data))
            .service(routes::dispensers::dispenser_scope())
            // this should always be last
            .service(files_handler)
    })
    .bind("[::]:8080")?
    .run()
    .await
}
