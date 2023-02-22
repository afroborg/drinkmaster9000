use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::Arc;

mod lib;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Arc::new(lib::state::AppState::new())))
            .service(routes::drinks::drinks)
            .service(routes::drinks::drinks_by_pin)
            .service(routes::drinks::toggle_drink_pin)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
