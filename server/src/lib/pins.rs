use rppal::gpio::{Gpio, InputPin, OutputPin};
use serde::Serialize;

#[derive(Serialize)]
pub enum PinType {
    Input(InputPin),
    Output(OutputPin),
}

#[derive(Serialize)]
pub struct Pin {
    pub nbr: u8,
    pub name: String,
    pub state: bool,
    pub pin_type: PinType,
}

impl Pin {
    pub fn input(nbr: u8, name: String) -> Pin {
        let gpio = Gpio::new().unwrap();
        let pin = gpio.get(nbr).unwrap().into_input();

        Pin {
            nbr,
            name,
            state: false,
            pin_type: PinType::Input(pin),
        }
    }

    pub fn output(nbr: u8, name: String) -> Pin {
        let gpio = Gpio::new().unwrap();
        let pin = gpio.get(nbr).unwrap().into_output();

        Pin {
            nbr,
            name,
            state: false,
            pin_type: PinType::Output(pin),
        }
    }
}
