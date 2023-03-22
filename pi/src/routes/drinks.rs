use std::thread;

use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Deserialize;

use crate::{lib::config::State, models::drink::Drink};

pub fn drinks_scope() -> Scope {
    web::scope("/drinks")
        .service(get_drinks)
        .service(edit_drinks)
        .service(make_drink)
}

#[get("")]
async fn get_drinks(data: State) -> impl Responder {
    let config = data.lock().unwrap();

    HttpResponse::Ok().json(&config.drinks)
}

#[post("")]
async fn edit_drinks(data: State, request: web::Json<Vec<Drink>>) -> impl Responder {
    let mut config = data.lock().unwrap();
    config.update_drinks(request.into_inner());

    HttpResponse::Ok().json(&config.drinks)
}

#[derive(Deserialize)]
struct Ingredient {
    pub index: usize,
    pub amount: f32,
}

#[post("/make")]
async fn make_drink(data: State, request: web::Json<Vec<Ingredient>>) -> impl Responder {
    let mut config = data.lock().unwrap();

    for ingredient in request.into_inner() {
        let rotate_duration = config.dispenser.rotate_cup_holder(ingredient.index);
        // wait for the cupholder to rotate
        thread::sleep(rotate_duration);

        let pour_duration = config.dispenser.dispense(ingredient.amount);

        thread::sleep(pour_duration);

        config.dispenser.stop();
    }

    HttpResponse::Ok().finish()
}
