use crate::models::dispenser::{Dispenser, UpdateDispenser};
use crate::models::drink::Drink;
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

pub type State = web::Data<Mutex<Config>>;

/// Configuration for the entire app
/// that can be read form and stored in a file
///
/// will be passed to all api routes
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub dispenser: Dispenser,
    pub drinks: Vec<Drink>,
}

impl Config {
    /// Create a new config struct from the config file
    pub fn new() -> Self {
        let file =
            fs::read_to_string("/srv/drinkmixer/config.ron").expect("Failed to read config file");

        // parse the config file
        let mut conf: Self = ron::from_str(&file).expect("Failed to parse config file");

        // initialize at stop state
        conf.dispenser.set_pushers_to_start();

        // rotate the cup holder to the first index
        conf.dispenser.rotate_cup_holder_to_index(0);

        conf
    }

    /// Create a new config struct from the config file wrapped in a mutex
    pub fn new_mutex() -> Mutex<Self> {
        Mutex::new(Self::new())
    }

    /// Update the dispenser config
    pub fn update_dispenser(&mut self, dispenser: UpdateDispenser) -> Result<(), String> {
        let res = self.dispenser.update(dispenser);
        self.save_to_file();

        res
    }

    /// Update the drinks config
    pub fn update_drinks(&mut self, drinks: Vec<Drink>) {
        self.drinks = drinks;
        self.save_to_file();
    }

    /// Save the config to the config file
    fn save_to_file(&self) {
        let file = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())
            .expect("Failed to serialize config");

        fs::write("/srv/drinkmixer/config.ron", file).expect("Failed to write config file");
    }
}
