use std::sync::Arc;

use axum::{
    extract::{ws, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::lib::{pins::PinType, state::AppState};

pub async fn drinks() -> impl IntoResponse {
    "Drinks"
}

pub async fn drinks_by_pin(Path(id): Path<u8>) -> impl IntoResponse {
    format!("Drinks by id: {}", id)
}

pub async fn toggle_drink_pin(
    Path(id): Path<u8>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let pin = state.pins.get_mut(&id).unwrap();

    match pin.pin_type {
        PinType::Input(_) => {
            // throw 400 error because pin is input
            Err(StatusCode::BAD_REQUEST);
        }
        PinType::Output(gpio) => {
            gpio.toggle();
        }
    }

    return format!("Toggle drink by id: {}", id);
}
