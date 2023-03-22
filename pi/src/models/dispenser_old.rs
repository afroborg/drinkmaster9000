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

#[derive(Serialize, Deserialize)]
pub struct DispenserResponse {
    name: String,
    pub position: u8,
    pin: u8,
    state: bool,
}

#[derive(Deserialize)]
pub struct EditDispenser {
    pub name: Option<String>,
    pub position: Option<u8>,
    pub pin: Option<u8>,
}

impl Dispenser {
    pub fn pour(&mut self, amount: u8) {
        println!("Pouring {}ml from {}", amount, self.name);
        self.pin.set_high();
    }

    pub fn to_json(&self) -> DispenserResponse {
        DispenserResponse {
            name: self.name.clone(),
            position: self.position,
            pin: self.pin.pin(),
            state: self.pin.is_set_high(),
        }
    }

    pub fn edit(&mut self, edit: &EditDispenser) -> DispenserResponse {
        if let Some(name) = &edit.name {
            self.name = name.to_string();
        }

        if let Some(position) = &edit.position {
            self.position = position.to_owned();
        }

        if let Some(pin) = edit.pin {
            let gpio = gpio::Gpio::new().expect("Failed to initialize GPIO");

            match gpio.get(pin) {
                Ok(pin) => self.pin = pin.into_output_low(),
                Err(_) => panic!("Failed to get pin {}", pin),
            }
        }

        self.to_json()
    }
}
