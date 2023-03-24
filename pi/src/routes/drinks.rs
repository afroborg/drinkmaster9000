use std::{thread, time::Duration};

use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Deserialize;

use crate::{lib::config::State, models::drink::Drink};

pub fn drinks_scope() -> Scope {
    web::scope("/drinks")
        .service(get_drinks)
        .service(edit_drinks)
        .service(make_drink)
}

/// Get the list of drinks
/// GET /api/drinks
#[get("")]
async fn get_drinks(data: State) -> impl Responder {
    let config = data.lock().unwrap();

    HttpResponse::Ok().json(&config.drinks)
}

/// Edit the list of drinks
/// POST /api/drinks
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

/// Make a drink
/// POST /api/drinks/make
#[post("/make")]
async fn make_drink(data: State, request: web::Json<Vec<Ingredient>>) -> impl Responder {
    let mut config = data.lock().unwrap();

    request.into_inner().iter().for_each(|ingredient| {
        // rotate the cup holder to the correct dispenser, and get the rotation duration
        let rotate_duration = config
            .dispenser
            .rotate_cup_holder_to_index(ingredient.index);

        // wait for the cupholder to rotate
        thread::sleep(rotate_duration);

        // dispense the drink, and get the pour duration
        let pour_duration = config.dispenser.dispense(ingredient.amount);

        // wait for the drink to be poured
        thread::sleep(pour_duration);

        // stop the dispenser
        config.dispenser.stop();

        // wait 2 seconds for the drink pouring to stop
        thread::sleep(Duration::from_secs(2));
    });

    // rotate back to start index
    config.dispenser.rotate_cup_holder(0);

    HttpResponse::Ok().finish()
}
