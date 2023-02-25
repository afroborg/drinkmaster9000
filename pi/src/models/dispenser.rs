use rppal::gpio::{self, OutputPin};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dispenser {
    pub name: String,
    pub position: u8,
    #[serde(deserialize_with = "from_u8", serialize_with = "from_output_pin")]
    pub pin: OutputPin,
}

fn from_u8<'de, D>(deserializer: D) -> Result<OutputPin, D::Error>
where
    D: Deserializer<'de>,
{
    let pin_nbr = u8::deserialize(deserializer)?;
    let gpio = gpio::Gpio::new().expect("Failed to initialize GPIO");

    match gpio.get(pin_nbr) {
        Ok(pin) => Ok(pin.into_output_low()),
        Err(_) => panic!("Failed to get pin {}", pin_nbr),
    }
}

fn from_output_pin<S>(pin: &OutputPin, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u8(pin.pin())
}

impl Dispenser {
    pub fn pour(&self, amount: u8) {
        println!("Pouring {}ml from {}", amount, self.name);
    }
}
