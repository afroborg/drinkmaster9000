use super::config::Config;
use crate::models::dispenser::Dispenser;
use actix_web::web;
use std::{collections::HashMap, sync::Mutex};

pub type State = web::Data<Mutex<AppState>>;

pub struct AppState {
    pub dispensers: HashMap<u8, Dispenser>, // create a hashmap of pins that are referenced by their pin number
}

impl AppState {
    pub fn new() -> Mutex<Self> {
        let config = Config::new();
        let mut map = HashMap::new();

        config.dispensers.into_iter().for_each(|dispenser| {
            map.insert(dispenser.position, dispenser);
        });

        Mutex::new(Self { dispensers: map })
    }
}
