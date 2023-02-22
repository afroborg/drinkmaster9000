use std::collections::HashMap;

use super::pins::Pin;

pub struct AppState {
    pub pins: HashMap<u8, Pin>,
}

impl AppState {
    pub fn new() -> Self {
        let mut pins = HashMap::new();

        pins.insert(18, Pin::input(18, "Pin 18".to_string()));

        Self { pins }
    }
}
