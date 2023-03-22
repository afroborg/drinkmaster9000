use crate::models::pins::{from_output_pin, from_u8};
use rppal::gpio::OutputPin;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const PERIOD_MS: u64 = 20;

#[derive(Serialize, Deserialize)]
pub struct Servo {
    #[serde(deserialize_with = "from_u8", serialize_with = "from_output_pin")]
    pin: OutputPin,
    min_us: u64,
    max_us: u64,
    neutral_us: u64,
    start_angle: u64,
    end_angle: u64,
    is_reversed: bool,
}

impl Servo {
    pub fn goto_end(&mut self) {
        self.set_angle(self.end_angle as u8);
    }

    pub fn goto_start(&mut self) {
        self.set_angle(self.start_angle as u8);
    }

    pub fn set_angle(&mut self, angle: u8) -> u64 {
        let pulse_width = self.min_us + (angle as u64 * (self.max_us - self.min_us) / 180);

        let pulse = match self.is_reversed {
            true => 180 - pulse_width,
            false => pulse_width,
        }
        .min(self.max_us);

        let _ = self.pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(pulse),
        );

        pulse_width
    }
}
