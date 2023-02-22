use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Arc;

use crate::lib::{pins::PinType, state::AppState};

#[get("/drinks")]
pub async fn drinks(data: web::Data<Arc<AppState>>) -> impl Responder {
    let state = data.pins.values().collect::<Vec<_>>();
    HttpResponse::Ok().json(state)
}

#[get("/drinks/{id}")]
pub async fn drinks_by_pin(id: web::Path<u8>) -> impl Responder {
    let id = id.into_inner();
    format!("Drinks by id: {}", id)
}

#[post("/drinks/{id}")]
pub async fn toggle_drink_pin(id: web::Path<u8>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let pin = data.pins.get_mut(&id).unwrap();

    match pin.pin_type {
        PinType::Input(_) => {
            // throw 400 error because pin is input
            return HttpResponse::BadRequest().body("Pin is input");
        }
        PinType::Output(gpio) => {
            gpio.toggle();
        }
    }

    let id = id.into_inner();

    return HttpResponse::Ok().body(format!("Toggle drink by id: {}", id));
}
