use axum::Router;
use std::env;

// use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// mod lib;
mod routes;

#[tokio::main]
async fn main() {
    let app_router = Router::new();
    let drinks_router = axum::route("/drinks", routes::drinks::drinks);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Listening on http://{}", addr);

    let app = app_router.route("/api", drinks_router);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}
