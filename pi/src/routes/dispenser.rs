use crate::{
    lib::config::State,
    models::dispenser::{Dispenser, UpdateDispenser},
};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};

pub fn dispenser_scope() -> Scope {
    web::scope("/dispenser")
        .service(get_dispenser)
        .service(edit_dispenser)
        .service(set_angle)
        .service(set_angle_index)
}

/// Get the current dispenser configuration
/// GET /api/dispenser
#[get("")]
async fn get_dispenser(data: State) -> impl Responder {
    let config = data.lock().unwrap();

    HttpResponse::Ok().json(&config.dispenser)
}

/// Edit the dispenser configuration
/// POST /api/dispenser
#[post("")]
async fn edit_dispenser(data: State, request: web::Json<UpdateDispenser>) -> impl Responder {
    let mut config = data.lock().unwrap();

    if let Err(err) = config.update_dispenser(request.into_inner()) {
        return HttpResponse::BadRequest().body(err);
    }

    HttpResponse::Ok().json(&config.dispenser)
}

/// Set the angle of all servos
/// POST /api/dispenser/angle/{angle}
#[post("/angle/{angle}")]
async fn set_angle(data: State, angle: web::Path<u8>) -> impl Responder {
    let mut config = data.lock().unwrap();
    config.dispenser.push_all_to_angle(angle.into_inner());

    HttpResponse::Ok().json(&config.dispenser)
}

/// Set the angle of a specific servo
/// POST /api/dispenser/{index}/angle/{angle}
#[post("/{index}/angle/{angle}")]
async fn set_angle_index(data: State, params: web::Path<(usize, u8)>) -> impl Responder {
    let mut config = data.lock().unwrap();
    let (index, angle) = params.into_inner();

    config.dispenser.push_to_angle(index, angle);

    HttpResponse::Ok().json(&config.dispenser)
}