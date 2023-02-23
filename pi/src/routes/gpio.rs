use crate::lib::{pins::PinType, state::Data};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
pub async fn list_gpio(data: Data) -> impl Responder {
    let data = data.lock().unwrap();

    HttpResponse::Ok().json(data.pins.values().collect::<Vec<_>>())
}

#[get("/{pin_nbr}")]
pub async fn get_gpio(pin_nbr: web::Path<u8>, data: Data) -> impl Responder {
    let pin_nbr = pin_nbr.into_inner();
    let data = data.lock().unwrap();
    let pin = data.pins.get(&pin_nbr).unwrap();

    match &pin.pin_type {
        PinType::Input(pin) => {
            let value = pin.read();
            HttpResponse::Ok().body(value.to_string())
        }
        PinType::Output(_) => HttpResponse::BadRequest().body("Cannot read output pin"),
    }
}

#[post("/{pin_nbr}")]
pub async fn toggle_gpio(pin_nbr: web::Path<u8>, data: Data) -> impl Responder {
    let pin_nbr = pin_nbr.into_inner();
    let mut data = data.lock().unwrap();
    let pin = data.pins.get_mut(&pin_nbr).unwrap();

    if let PinType::Output(pin) = &mut pin.pin_type {
        pin.toggle();
        HttpResponse::Ok().body("Toggled")
    } else {
        HttpResponse::BadRequest().body("Cannot toggle input pin")
    }
}
