use super::config::Config;
use crate::models::dispenser::{Dispenser, EditDispenser};
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

    pub fn edit_dispenser(&mut self, position: u8, edit: &EditDispenser) {
        let dispenser = self.dispensers.get_mut(&position).unwrap();
        dispenser.edit(&edit);

        if let Some(new_position) = edit.position {
            self.move_dispenser(position, new_position);
        }
    }

    pub fn move_dispenser(&mut self, from: u8, to: u8) {
        let dispenser = self.dispensers.remove(&from).unwrap();
        self.dispensers.insert(to, dispenser);
    }
}
