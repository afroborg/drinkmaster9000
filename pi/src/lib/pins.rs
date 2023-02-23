use rppal::gpio::{Gpio, InputPin, OutputPin};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::config::{PinConfig, PinConfigType};

#[derive(Deserialize, Serialize)]
pub enum PinType {
    Input,
    Output,
}

#[derive(Deserialize, Serialize)]
pub struct Pin {
    pub name: String,
    pub pin: u8,
    pub pwm: bool,
    pub pin_type: PinType,
}

impl Pin {
    pub fn from_config(pin: &PinConfig) -> Self {
        Self {
            name: pin.name.clone(),
            pin: pin.pin,
            pwm: pin.pwm,
            pin_type,
        }
    }
}
