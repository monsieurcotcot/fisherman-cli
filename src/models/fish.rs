use serde::{Deserialize, Serialize};
use crate::config::Rarity;

use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fish {
    pub name: String,
    pub rarity: Rarity,
    pub size: f64,
    pub weight: f64,
    pub state: String,
    pub description: String,
    pub stream_title: Option<String>,
    pub caught_at: Option<DateTime<Utc>>,
    pub is_junk: bool,
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
            stream_title: None,
            caught_at: None,
            is_junk: false,
        }
    }

    pub fn new_junk(name: String, rarity: Rarity, size: f64, weight: f64, state: String, description: String) -> Self {
        Self {
            name,
            rarity,
            size,
            weight,
            state,
            description,
            stream_title: None,
            caught_at: None,
            is_junk: true,
        }
    }
}
