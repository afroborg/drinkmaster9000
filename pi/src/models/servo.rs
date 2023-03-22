use crate::models::pins::{from_output_pin, from_u8};
use rppal::gpio::{Gpio, OutputPin};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const PERIOD_MS: u64 = 20;

#[derive(Serialize, Deserialize)]
pub struct Servo {
    #[serde(deserialize_with = "from_u8", serialize_with = "from_output_pin")]
    pin: Option<OutputPin>,
    min_us: u64,
    max_us: u64,
    neutral_us: u64,
    start_angle: u64,
    end_angle: u64,
    is_reversed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateServo {
    pub pin: u8,
    pub min_us: u64,
    pub max_us: u64,
    pub neutral_us: u64,
    pub start_angle: u64,
    pub end_angle: u64,
    pub is_reversed: bool,
}

impl Servo {
    pub fn update(&mut self, update: UpdateServo) -> Result<(), String> {
        // kill the current pin
        self.die();

        let gpio = match Gpio::new() {
            Ok(gpio) => gpio,
            Err(e) => return Err(e.to_string()),
        };

        let pin = match gpio.get(update.pin) {
            Ok(pin) => pin,
            Err(e) => return Err(e.to_string()),
        };

        self.pin = Some(pin.into_output_low());

        self.min_us = update.min_us;
        self.max_us = update.max_us;
        self.neutral_us = update.neutral_us;
        self.start_angle = update.start_angle;
        self.end_angle = update.end_angle;
        self.is_reversed = update.is_reversed;

        Ok(())
    }

    pub fn die(&mut self) {
        self.pin = None;
    }

    pub fn goto_end(&mut self) -> Result<u64, String> {
        self.set_angle(self.end_angle as u8)
    }

    pub fn goto_start(&mut self) -> Result<u64, String> {
        self.set_angle(self.start_angle as u8)
    }

    pub fn set_angle(&mut self, angle: u8) -> Result<u64, String> {
        let Some(pin) = &mut self.pin else {
            return Err("Pin not set".to_string());
        };

        let pulse_width = self.min_us + (angle as u64 * (self.max_us - self.min_us) / 180);

        let pulse = match self.is_reversed {
            true => 180 - pulse_width,
            false => pulse_width,
        }
        .min(self.max_us);

        let _ = pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(pulse),
        );

        Ok(pulse_width)
    }
}
