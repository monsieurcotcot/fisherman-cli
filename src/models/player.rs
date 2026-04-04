use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Option<i64>,
    pub username: String,
    pub total_attempts: i64,
    pub successful_attempts: i64,
    pub failed_attempts: i64,
    pub last_fishing_time: Option<DateTime<Utc>>,
}

impl Player {
    pub fn new(username: String) -> Self {
        Self {
            id: None,
            username,
            total_attempts: 0,
            successful_attempts: 0,
            failed_attempts: 0,
            last_fishing_time: None,
        }
    }

    pub fn can_fish(&self, cooldown_seconds: i64) -> bool {
        match self.last_fishing_time {
            Some(last_time) => {
                let now = Utc::now();
                let diff = now.signed_duration_since(last_time).num_seconds();
                diff >= cooldown_seconds
            }
            None => true,
        }
    }
}
