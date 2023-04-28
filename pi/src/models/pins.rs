use rppal::gpio::{self, OutputPin};
use serde::{Deserialize, Deserializer};

/// Deserialize a u8 into an `OutputPin`
/// this is benificial because the pin number is stored in the config file
/// and we want the struct to be able to control the pin
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

/// Serialize an `OutputPin` into a u8
/// this is benificial because the pin number is stored in the config file and be returned as a u8 in the json
pub fn from_output_pin<S>(pin: &Option<OutputPin>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match pin {
        Some(pin) => serializer.serialize_u8(pin.pin()),
        None => serializer.serialize_none(),
    }
}
