use serde::{Deserialize, Serialize};
use std::fs;

use crate::models::dispenser::Dispenser;

#[derive(Serialize)]
pub enum PinConfigType {
    Input,
    Output,
}

impl<'de> Deserialize<'de> for PinConfigType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "input" => Ok(PinConfigType::Input),
            "output" => Ok(PinConfigType::Output),
            _ => Err(serde::de::Error::custom(
                "Cannot instantiate PinConfigType from string",
            )),
        }
    }
}

#[derive(Deserialize)]
pub struct PinConfig {
    pub name: String,
    pub pin: u8,
    pub pwm: bool,
    pub pin_type: PinConfigType,
}

#[derive(Deserialize)]
pub struct Config {
    // pub pins: Vec<PinConfig>,
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
