use crate::models::dispenser::Dispenser;
use crate::models::drink::Drink;
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

pub type State = web::Data<Mutex<Config>>;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub dispenser: Dispenser,
    pub drinks: Vec<Drink>,
}

impl Config {
    pub fn new() -> Self {
        let file =
            fs::read_to_string("/srv/drinkmixer/config.ron").expect("Failed to read config file");

        let config = ron::from_str(&file).expect("Failed to parse config file");

        config
    }

    pub fn update_dispenser(&mut self, dispenser: Dispenser) {
        self.dispenser = dispenser;
        self.save_to_file();
    }

    pub fn update_drinks(&mut self, drinks: Vec<Drink>) {
        self.drinks = drinks;
        self.save_to_file();
    }

    pub fn update(&mut self, new_config: Self) {
        self.dispenser = new_config.dispenser;
        self.drinks = new_config.drinks;

        self.save_to_file();
    }

    fn save_to_file(&self) {
        let file = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())
            .expect("Failed to serialize config");

        fs::write("/srv/drinkmixer/config.ron", file).expect("Failed to write config file");
    }
}
