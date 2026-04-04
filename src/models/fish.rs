use serde::{Deserialize, Serialize};
use crate::config::Rarity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fish {
    pub name: String,
    pub rarity: Rarity,
    pub size: f64,
    pub weight: f64,
    pub state: String,
    pub description: String,
}

impl Fish {
    pub fn new(name: String, rarity: Rarity, size: f64, weight: f64, state: String, description: String) -> Self {
        Self {
            name,
            rarity,
            size,
            weight,
            state,
            description,
        }
    }
}
