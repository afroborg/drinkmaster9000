use serde::{Deserialize, Serialize};

use super::servo::Servo;

#[derive(Serialize, Deserialize)]
pub struct Dispenser(Vec<Servo>);

impl Dispenser {
    pub fn rotate_all(&mut self, angle: u8) {
        for servo in self.0.iter_mut() {
            servo.set_angle(angle);
        }
    }

    pub fn rotate(&mut self, index: usize, angle: u8) {
        self.0[index].set_angle(angle);
    }
}
