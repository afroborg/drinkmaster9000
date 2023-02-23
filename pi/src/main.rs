#![allow(special_module_name)]
use std::sync::Mutex;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use rppal::system::DeviceInfo;

mod lib;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running server on [::]:8080");
    println!("Running on a {}.", DeviceInfo::new().unwrap().model());

    let data = web::Data::new(Mutex::new(lib::state::AppState::new()));

    HttpServer::new(move || {
        use routes::*;

        App::new()
            .app_data(web::Data::clone(&data))
            .service(
                // set up methods to do with GPIO
                web::scope("/gpio")
                    .service(gpio::list_gpio)
                    .service(gpio::get_gpio)
                    .service(gpio::toggle_gpio),
            )
            // this should always be last
            .service(
                Files::new("/", "/srv/drinkmixer/static")
                    // makes sure we render the actual "index.html" file instead of a file tree
                    .index_file("index.html")
                    // if there is no "index.html", we render the file tree
                    .show_files_listing(),
            )
    })
    .bind("[::]:8080")?
    .run()
    .await
}
