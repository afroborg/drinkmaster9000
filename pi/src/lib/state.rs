use super::{config::Config, pins::Pin};
use actix_web::web;
use rppal::gpio::Gpio;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type Data = web::Data<Arc<Mutex<AppState>>>;

pub struct AppState {
    pub pins: HashMap<u8, Pin>, // create a hashmap of pins that are referenced by their pin numbe
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::new();
        let mut map = HashMap::new();

        config
            .pins
            .iter()
            .map(|p| Pin::from_config(p))
            .for_each(|pin| {
                map.insert(pin.pin, pin);
            });

        Self { pins: map }
    }
}
