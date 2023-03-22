use std::time::Duration;

use crate::models::pin::{from_output_pin, from_u8};
use rppal::gpio::OutputPin;
use serde::{Deserialize, Serialize};

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

// impl<'a> Servo<'a> {
//     pub fn new(pin: &'a mut OutputPin, min_us: u64, max_us: u64, neutral_us: u64) -> Self {
//         let _ = pin.clear_pwm();

//         Self {
//             pin,
//             min_us,
//             max_us,
//             neutral_us,
//         }
//     }

//     pub fn set_angle(&mut self, angle: u64) -> u64 {
//         let pulse_width = self.min_us + (angle as u64 * (self.max_us - self.min_us) / 180);

//         let _ = self.pin.set_pwm(
//             Duration::from_millis(PERIOD_MS),
//             Duration::from_micros(pulse_width.min(self.max_us)),
//         );

//         pulse_width
//     }

//     pub fn set_max(&mut self) {
//         let _ = self.pin.set_pwm(
//             Duration::from_millis(PERIOD_MS),
//             Duration::from_micros(self.max_us),
//         );
//     }

//     pub fn set_min(&mut self) {
//         let _ = self.pin.set_pwm(
//             Duration::from_millis(PERIOD_MS),
//             Duration::from_micros(self.min_us),
//         );
//     }

//     pub fn set_neutral(&mut self) {
//         let _ = self.pin.set_pwm(
//             Duration::from_millis(PERIOD_MS),
//             Duration::from_micros(self.neutral_us as u64),
//         );
//     }
// }
