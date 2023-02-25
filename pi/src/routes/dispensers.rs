use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Deserialize;

use crate::lib::state::State;

pub fn dispenser_scope() -> Scope {
    web::scope("/dispensers")
        .service(list_dispensers)
        .service(get_dispenser)
        .service(update_dispenser)
        .service(pour_amount)
}

#[get("")]
async fn list_dispensers(state: State) -> impl Responder {
    let state = state.lock().unwrap();
    let dispensers = state.dispensers.values().collect::<Vec<_>>();

    HttpResponse::Ok().json(dispensers)
}

#[get("/{position}")]
async fn get_dispenser(state: State, position: web::Path<u8>) -> impl Responder {
    let state = state.lock().unwrap();
    let dispenser = state.dispensers.get(&position.into_inner());

    match dispenser {
        Some(dispenser) => HttpResponse::Ok().json(dispenser),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{position}")]
async fn update_dispenser(state: State, position: web::Path<u8>) -> impl Responder {
    let state = state.lock().unwrap();
    let dispenser = state.dispensers.get(&position.into_inner());

    match dispenser {
        Some(dispenser) => HttpResponse::Ok().json(dispenser),
        None => HttpResponse::NotFound().finish(),
    }
}

#[derive(Deserialize)]
struct PourRequest {
    amount: u8,
}

#[post("/{position}/pour")]
async fn pour_amount(
    state: State,
    position: web::Path<u8>,
    request: web::Json<PourRequest>,
) -> impl Responder {
    let state = state.lock().unwrap();
    let dispenser = state.dispensers.get(&position.into_inner());

    match dispenser {
        Some(dispenser) => HttpResponse::Ok().json(dispenser.pour(request.amount)),
        None => HttpResponse::NotFound().finish(),
    }
}
