use crate::{lib::config::State, models::dispenser::UpdateDispenser};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};

pub fn scope() -> Scope {
    web::scope("/dispenser")
        .service(get_dispenser)
        .service(edit_dispenser)
        .service(set_angle)
        .service(set_angle_index)
        .service(set_cup_angle)
}

/// Get the current dispenser configuration
/// GET /api/dispenser
#[get("")]
async fn get_dispenser(data: State) -> impl Responder {
    let config = data.lock().await;

    HttpResponse::Ok().json(&config.dispenser)
}

/// Edit the dispenser configuration
/// POST /api/dispenser
#[post("")]
async fn edit_dispenser(data: State, request: web::Json<UpdateDispenser>) -> impl Responder {
    let mut config = data.lock().await;

    if let Err(err) = config.update_dispenser(request.into_inner()) {
        return HttpResponse::BadRequest().body(err);
    }

    HttpResponse::Ok().json(&config.dispenser)
}

/// Set the angle of the cup holder
/// POST /api/dispenser/cup/angle/{angle}
#[post("/cup/angle/{angle}")]
async fn set_cup_angle(data: State, angle: web::Path<u8>) -> impl Responder {
    let mut config = data.lock().await;

    config
        .dispenser
        .step_cup_holder_to_angle(angle.into_inner())
        .await;

    HttpResponse::Ok().json(&config.dispenser)
}

/// Set the angle of all servos
/// POST /api/dispenser/pusher/angle/{angle}
#[post("/pusher/angle/{angle}")]
async fn set_angle(data: State, angle: web::Path<u8>) -> impl Responder {
    let mut config = data.lock().await;

    // push all servos to the same angle
    config.dispenser.push_all_to_angle(angle.into_inner());

    HttpResponse::Ok().json(&config.dispenser)
}

/// Set the angle of a specific servo
/// POST /api/dispenser/pusher/{index}/angle/{angle}
#[post("/pusher/{index}/angle/{angle}")]
async fn set_angle_index(data: State, params: web::Path<(usize, u8)>) -> impl Responder {
    let mut config = data.lock().await;

    // get the url parameters
    let (index, angle) = params.into_inner();

    // push the servo to the specified angle
    if let Err(e) = config.dispenser.push_to_angle(index, angle) {
        return HttpResponse::BadRequest().body(e);
    };

    HttpResponse::Ok().json(&config.dispenser)
}
