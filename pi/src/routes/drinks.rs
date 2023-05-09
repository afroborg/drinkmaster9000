use crate::{lib::config::State, models::drink::Drink};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Deserialize;

pub fn scope() -> Scope {
    web::scope("/drinks")
        .service(get_drinks)
        .service(edit_drinks)
        .service(make_drink)
}

/// Get the list of drinks
/// GET /api/drinks
#[get("")]
async fn get_drinks(data: State) -> impl Responder {
    let config = data.lock().await;

    HttpResponse::Ok().json(&config.drinks)
}

/// Edit the list of drinks
/// POST /api/drinks
#[post("")]
async fn edit_drinks(data: State, request: web::Json<Vec<Drink>>) -> impl Responder {
    let mut config = data.lock().await;
    config.update_drinks(request.into_inner());

    HttpResponse::Ok().json(&config.drinks)
}
#[derive(Deserialize)]
struct Ingredient {
    pub index: usize,
    pub amount: f32, // amount of drink to pour in ml
}

/// Make a drink
/// POST /api/drinks/make
#[post("/make")]
async fn make_drink(data: State, request: web::Json<Vec<Ingredient>>) -> impl Responder {
    let mut config = data.lock().await;

    // go to start position
    config.dispenser.pushers_down();

    println!("--- Creating new drink ---");

    for ingredient in request.into_inner() {
        config
            .dispenser
            .rotate_cup_holder_to_index(ingredient.index)
            .await;

        if let Err(err) = config.dispenser.dispense(ingredient.amount).await {
            return HttpResponse::InternalServerError().body(err);
        };
    }

    // rotate back to start index
    config.dispenser.cup_rotator_waiting_position().await;

    HttpResponse::Ok().finish()
}
