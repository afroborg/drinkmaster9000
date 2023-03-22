use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::servo::{Servo, UpdateServo};

#[derive(Serialize, Deserialize)]
pub struct Dispenser {
    current_index: usize,
    angle_between: u8,      // angle between each drink dispenser
    rotation_delay_ms: u64, // delay to rotate between each drink dispenser in ms
    pour_speed_ml_ms: u8,   // ml per ms
    cup_rotator: Servo,
    pusher: Vec<Servo>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDispenser {
    pub angle_between: u8,
    pub rotation_delay_ms: u64,
    pub pour_speed_ml_ms: u8,
    pub cup_rotator: UpdateServo,
    pub pusher: Vec<UpdateServo>,
}

impl Dispenser {
    pub fn update(&mut self, update: UpdateDispenser) -> Result<(), String> {
        self.angle_between = update.angle_between;
        self.rotation_delay_ms = update.rotation_delay_ms;
        self.pour_speed_ml_ms = update.pour_speed_ml_ms;

        if let Err(err) = self.cup_rotator.update(update.cup_rotator) {
            return Err(err);
        };

        for (servo, update) in self.pusher.iter_mut().zip(update.pusher.into_iter()) {
            if let Err(err) = servo.update(update) {
                return Err(err);
            }
        }

        Ok(())
    }

    pub fn dispense(&mut self, amount: f32) -> Duration {
        for servo in self.pusher.iter_mut() {
            let _ = servo.goto_end();
        }

        Duration::from_millis((amount * self.pour_speed_ml_ms as f32) as u64)
    }

    pub fn stop(&mut self) {
        for servo in self.pusher.iter_mut() {
            let _ = servo.goto_start();
        }
    }

    pub fn push_all_to_angle(&mut self, angle: u8) {
        for servo in self.pusher.iter_mut() {
            let _ = servo.set_angle(angle);
        }
    }

    pub fn push_to_angle(&mut self, index: usize, angle: u8) {
        let _ = self.pusher[index].set_angle(angle);
    }

    /// Rotate to the cupholder to the given index
    /// returns the duration of the rotation
    pub fn rotate_cup_holder(&mut self, index: usize) -> Duration {
        let between = self.angle_between.abs_diff(self.current_index as u8);
        let angle = self.angle_between * index as u8;

        // TODO: rotate with the cupholder servo
        let _ = self.cup_rotator.set_angle(angle);

        Duration::from_millis(self.rotation_delay_ms * between as u64)
    }
}
