use crate::{
    lib::state::State,
    models::dispenser::{Dispenser, EditDispenser},
};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Deserialize;

pub fn dispenser_scope() -> Scope {
    web::scope("/dispensers")
        .service(list_dispensers)
        .service(get_dispenser)
        .service(update_dispenser)
        .service(toggle_pin)
        .service(pour_amount)
}

#[get("")]
async fn list_dispensers(state: State) -> impl Responder {
    let state = state.lock().unwrap();
    let mut dispensers = state
        .dispensers
        .values()
        .map(Dispenser::to_json)
        .collect::<Vec<_>>();
    dispensers.sort_by_key(|d| d.position);

    HttpResponse::Ok().json(dispensers)
}

#[get("/{position}")]
async fn get_dispenser(state: State, position: web::Path<u8>) -> impl Responder {
    let state = state.lock().unwrap();
    let dispenser = state.dispensers.get(&position.into_inner());

    match dispenser {
        Some(dispenser) => HttpResponse::Ok().json(dispenser.to_json()),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{position}")]
async fn update_dispenser(
    state: State,
    position: web::Path<u8>,
    request: web::Json<EditDispenser>,
) -> impl Responder {
    let mut state = state.lock().unwrap();

    if let Some(check_position) = request.position {
        if state.dispensers.contains_key(&check_position) {
            return HttpResponse::BadRequest()
                .body(format!("Position {} is already in use", check_position));
        }
    }

    let curr_position = &position.into_inner();
    let dispenser = state.dispensers.get_mut(&curr_position);

    match dispenser {
        Some(_) => {
            let req = request.into_inner();
            state.edit_dispenser(*curr_position, &req);

            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[derive(Deserialize)]
struct PourRequest {
    amount: u8,
}

#[post("/{position}/toggle")]
async fn toggle_pin(state: State, position: web::Path<u8>) -> impl Responder {
    let mut state = state.lock().unwrap();
    let dispenser = state.dispensers.get_mut(&position.into_inner());

    match dispenser {
        Some(dispenser) => {
            dispenser.pin.toggle();
            HttpResponse::Ok().json(dispenser.to_json())
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{position}/pour")]
async fn pour_amount(
    state: State,
    position: web::Path<u8>,
    request: web::Json<PourRequest>,
) -> impl Responder {
    let mut state = state.lock().unwrap();
    let dispenser = state.dispensers.get_mut(&position.into_inner());

    match dispenser {
        Some(dispenser) => {
            dispenser.pour(request.amount);

            HttpResponse::Ok().json(dispenser.to_json())
        }
        None => HttpResponse::NotFound().finish(),
    }
}
