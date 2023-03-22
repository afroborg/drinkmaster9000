use rppal::gpio::{self, OutputPin};
use serde::{Deserialize, Deserializer};

pub fn from_u8<'de, D>(deserializer: D) -> Result<OutputPin, D::Error>
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

pub fn from_output_pin<S>(pin: &OutputPin, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u8(pin.pin())
}
