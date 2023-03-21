use crate::models::dispenser::Dispenser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub dispensers: Vec<Dispenser>,
}

impl Config {
    pub fn new() -> Self {
        let file =
            fs::read_to_string("/srv/drinkmixer/config.ron").expect("Failed to read config file");

        let config = ron::from_str(&file).expect("Failed to parse config file");

        config
    }
}
