use rppal::gpio::{Gpio, InputPin, OutputPin};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::config::{PinConfig, PinConfigType};

pub enum PinType {
    Input(InputPin),
    Output(OutputPin),
}

impl Serialize for PinType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PinType::Input(_) => serializer.serialize_str("input"),
            PinType::Output(_) => serializer.serialize_str("output"),
        }
    }
}

impl<'de> Deserialize<'de> for PinType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // string should look like: "input 4" or "output 4"
        let s = String::deserialize(deserializer)?;

        let (pin_type, pin_nbr) = s.split_once(" ").unwrap();

        let gpio = Gpio::new().unwrap();
        let pin = gpio.get(pin_nbr.parse().unwrap()).unwrap();

        match pin_type {
            "input" => Ok(PinType::Input(pin.into_input())),
            "output" => Ok(PinType::Output(pin.into_output())),
            _ => Err(serde::de::Error::custom(
                "Cannot instantiate PinType from string",
            )),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Pin {
    pub name: String,
    pub pin: u8,
    pub pwm: bool,
    pub pin_type: PinType,
}

impl Pin {
    pub fn from_config(pin: &PinConfig, gpio: &Gpio) -> Self {
        let gpio_pin = gpio.get(pin.pin).expect("Failed to get pin");

        let pin_type = match pin.pin_type {
            PinConfigType::Input => PinType::Input(gpio_pin.into_input_pulldown()),
            PinConfigType::Output => PinType::Output(gpio_pin.into_output_low()),
        };

        Self {
            name: pin.name.clone(),
            pin: pin.pin,
            pwm: pin.pwm,
            pin_type,
        }
    }
}
