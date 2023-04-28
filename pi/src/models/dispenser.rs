use super::servo::{Servo, UpdateServo};
use actix_web::rt::time;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Dispenser {
    current_index: usize,
    angle_between: u8,      // angle between each drink dispenser
    rotation_delay_ms: u64, // delay to rotate between each drink dispenser in ms
    pour_speed_ml_ms: u8,   // ml per ms
    refill_delay_ms: u64,   // delay it takes for the small dispenser to refill in ms
    cup_rotator: Servo,
    pusher: Vec<Servo>,
}

/// UpdateDispenser is used to update the Dispenser struct
/// necessary because the Dispenser struct contains Servo structs, which we want to represeent as u8s
#[derive(Serialize, Deserialize)]
pub struct UpdateDispenser {
    pub angle_between: u8,
    pub rotation_delay_ms: u64,
    pub pour_speed_ml_ms: u8,
    pub refill_delay_ms: u64,
    pub cup_rotator: UpdateServo,
    pub pusher: Vec<UpdateServo>,
}

impl Dispenser {
    /// Update the dispenser with the given UpdateDispenser values
    pub fn update(&mut self, update: UpdateDispenser) -> Result<(), String> {
        self.angle_between = update.angle_between;
        self.rotation_delay_ms = update.rotation_delay_ms;
        self.pour_speed_ml_ms = update.pour_speed_ml_ms;
        self.refill_delay_ms = update.refill_delay_ms;

        // update the cup rotator servo
        self.cup_rotator.update(update.cup_rotator)?;

        // update the pusher servos
        for (servo, update) in self.pusher.iter_mut().zip(update.pusher.into_iter()) {
            servo.update(update)?;
        }

        Ok(())
    }

    /// Dispense the given amount of liquid
    pub async fn dispense(&mut self, amount: f32) -> Result<(), String> {
        const MAX_DISPENSES_PER_PUSH: f32 = 35.0;

        let number_of_dispenses = (amount / MAX_DISPENSES_PER_PUSH).ceil() as u8;

        println!("Dispensing {amount}ml, meaning 35x{number_of_dispenses} times");

        for i in 1..=number_of_dispenses {
            let to_dispense = if i == number_of_dispenses {
                amount % MAX_DISPENSES_PER_PUSH
            } else {
                MAX_DISPENSES_PER_PUSH
            };

            // set all the pushers to the maximum angle
            for servo in self.pusher.iter_mut() {
                let _ = servo.goto_end();
            }

            let wait_duration = to_dispense * self.pour_speed_ml_ms as f32;

            println!("Waiting for {wait_duration} ms");

            time::sleep(Duration::from_millis(wait_duration as u64)).await;

            self.set_start();

            if i != number_of_dispenses {
                let delay = self.refill_delay_ms;

                println!("Waiting for refill: {delay} ms");

                time::sleep(Duration::from_millis(delay)).await;
            }
        }

        time::sleep(Duration::from_secs(2)).await;

        Ok(())
    }

    pub async fn initialize(&mut self) {
        for servo in self.pusher.iter_mut() {
            let _ = servo.goto_start();

            time::sleep(Duration::from_millis(100)).await;
        }

        let _ = self.cup_rotator.step_to_angle(0).await;
    }

    /// Make all the pushers go to the start position
    pub fn set_start(&mut self) {
        for servo in self.pusher.iter_mut() {
            let _ = servo.goto_start();
        }
    }

    /// Push all the pushers to the given angle
    pub fn push_all_to_angle(&mut self, angle: u8) {
        for servo in self.pusher.iter_mut() {
            let _ = servo.set_angle(angle);
        }
    }

    /// Push the a specified pusher at the given index to the given angle
    pub fn push_to_angle(&mut self, index: usize, angle: u8) -> Result<(), String> {
        if let Some(servo) = self.pusher.get_mut(index) {
            let _ = servo.set_angle(angle);
            Ok(())
        } else {
            Err(format!("No pusher at index {}", index))
        }
    }

    // step n times in the direction of the given angle
    pub async fn step_cup_holder_to_angle(&mut self, angle: u8) {
        let _ = self.cup_rotator.step_to_angle(angle).await;
    }

    /// Rotate to the cupholder to the given index
    /// returns the duration of the rotation
    pub async fn rotate_cup_holder_to_index(&mut self, index: usize) {
        let angle = self.angle_between * index as u8;

        self.step_cup_holder_to_angle(angle).await;

        time::sleep(Duration::from_millis(self.rotation_delay_ms)).await;
    }
}
