use crate::models::pins::{from_output_pin, from_u8};
use actix_web::rt::time;
use rppal::gpio::OutputPin;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const PERIOD_MS: u64 = 20;

#[derive(Serialize, Deserialize)]
pub struct Servo {
    #[serde(deserialize_with = "from_u8", serialize_with = "from_output_pin")]
    pin: Option<OutputPin>,
    min_us: u64,     // the pulse width in microseconds for the minimum position
    max_us: u64,     // the pulse width in microseconds for the maximum position
    neutral_us: u64, // the pulse width in microseconds for the neutral position
    start_angle: u64,
    end_angle: u64,
    is_reversed: bool, // indicating if the servo should move CW or CCW
    current_angle: Option<u64>,
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
    /// update the servo with the new values
    pub fn update(&mut self, update: UpdateServo) -> Result<(), String> {
        // kill the current pin
        self.die();

        let pin_str = update.pin.to_string();
        let mut pin_deserializer = serde_json::Deserializer::from_str(&pin_str);

        // make sure we get a pin
        let Ok(pin) = from_u8(&mut pin_deserializer) else {
            return Err("Failed to set pin".to_string());
        };

        // set the new values
        self.pin = pin;
        self.min_us = update.min_us;
        self.max_us = update.max_us;
        self.neutral_us = update.neutral_us;
        self.start_angle = update.start_angle;
        self.end_angle = update.end_angle;
        self.is_reversed = update.is_reversed;

        self.current_angle = Some(update.start_angle);

        Ok(())
    }

    /// Kill the current pin, causing it to go out of scope and resetting the GPIO
    fn die(&mut self) {
        self.pin = None;
    }

    /// Set the servo to the maximum angle
    pub fn goto_end(&mut self) -> Result<u64, String> {
        let res = self.set_angle(self.end_angle as u8);
        self.current_angle = Some(self.end_angle);

        res
    }

    /// Set the servo to the minimum angle
    pub fn goto_start(&mut self) -> Result<u64, String> {
        let res = self.set_angle(self.start_angle as u8);
        self.current_angle = Some(self.start_angle);

        res
    }

    pub async fn step_to_angle(&mut self, angle: u8) {
        let Some(current_angle) = self.current_angle else {
            println!("No current angle set: wanting angle {angle}");
            return;
        };

        let angle = angle as u64;

        let range = if current_angle < angle {
            (current_angle..=angle).collect::<Vec<_>>()
        } else {
            (angle..=current_angle).rev().collect::<Vec<_>>()
        };

        println!("Stepping from {current_angle} to {angle}");

        println!("Range: {range:?}");

        for a in range {
            let _ = self.set_angle(a as u8);
            time::sleep(Duration::from_millis(25)).await;
        }

        self.current_angle = Some(angle);
    }

    /// Set the servo to a specified angle
    pub fn set_angle(&mut self, angle: u8) -> Result<u64, String> {
        // make sure we have a pin set
        let Some(pin) = &mut self.pin else {
            return Err("Pin not set".to_string());
        };

        // get the angle based on if the servo is coing CW or CCW
        let angle = if self.is_reversed { 180 - angle } else { angle };

        // calculate the pulse width for the specified angle
        let pulse_width = self.min_us + (u64::from(angle) * (self.max_us - self.min_us) / 180);

        // send the pulse to the servo
        let _ = pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(pulse_width.min(self.max_us)),
        );

        // return the pulse width
        Ok(pulse_width)
    }
}
