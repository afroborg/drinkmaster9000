use rppal::gpio::{self, OutputPin};
use serde::{Deserialize, Deserializer};

pub fn from_u8<'de, D>(deserializer: D) -> Result<Option<OutputPin>, D::Error>
where
    D: Deserializer<'de>,
{
    let pin_nbr = u8::deserialize(deserializer)?;
    let gpio = gpio::Gpio::new().expect("Failed to initialize GPIO");

    match gpio.get(pin_nbr) {
        Ok(pin) => Ok(Some(pin.into_output_low())),
        Err(_) => Ok(None),
    }
}

pub fn from_output_pin<S>(pin: &Option<OutputPin>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match pin {
        Some(pin) => serializer.serialize_u8(pin.pin()),
        None => serializer.serialize_none(),
    }
}
