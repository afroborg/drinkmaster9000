use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drink {
    pub name: String,
    pub position: u8,
}
