use axum::routing::{get, post};
use axum::Router;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

mod lib;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .with_state(Arc::new(lib::state::AppState::new()))
        .route("/drinks", get(routes::drinks::drinks))
        .route("/drinks/:pin", get(routes::drinks::drinks_by_pin))
        .route("/drinks/:pin", post(routes::drinks::toggle_drink_pin));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
