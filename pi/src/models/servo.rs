use rppal::gpio::OutputPin;
use std::{thread, time::Duration};

const PERIOD_MS: u64 = 20;

pub struct Servo<'a> {
    pin: &'a mut OutputPin,
    min_us: u64,
    max_us: u64,
    neutral_us: u64,
}

impl<'a> Servo<'a> {
    pub fn new(pin: &'a mut OutputPin, min_us: u64, max_us: u64, neutral_us: u64) -> Self {
        let _ = pin.clear_pwm();

        Self {
            pin,
            min_us,
            max_us,
            neutral_us,
        }
    }

    pub fn set_angle(&mut self, angle: u64) {
        let pulse_width = self.min_us + (angle as u64 * (self.max_us - self.min_us) / 180);

        let _ = self.pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(pulse_width as u64),
        );
    }

    pub fn set_max(&mut self) {
        let _ = self.pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(self.max_us),
        );
    }

    pub fn set_min(&mut self) {
        let _ = self.pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(self.min_us),
        );
    }

    pub fn set_neutral(&mut self) {
        let _ = self.pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(self.neutral_us as u64),
        );
    }

    pub fn step_to(&mut self, angle: u64, step: u64) {
        let mut current_angle = 0;

        while current_angle < angle {
            self.set_angle(current_angle);
            current_angle += step;
            thread::sleep(Duration::from_millis(10))
        }
    }
}
