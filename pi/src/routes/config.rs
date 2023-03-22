use crate::{
    lib::{config::ServoConfig, state::State},
    models::dispenser_old::Dispenser,
};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};

pub fn dispenser_scope() -> Scope {
    web::scope("/config").service(get_config)
}

#[get("")]
async fn get_config(state: State) -> impl Responder {
    let state = state.lock().unwrap();

    HttpResponse::Ok().json(&state.servoConfig)
}
