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
    pub level: i32,
    pub xp: i64,
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
            level: 1,
            xp: 0,
        }
    }

    pub fn xp_for_next_level(&self) -> i64 {
        // Formule : Niveau^1.5 * 100 (Progression jusqu'au niveau 200)
        ((self.level as f64).powf(1.5) * 100.0) as i64
    }

    pub fn add_xp(&mut self, amount: i64) -> bool {
        self.xp += amount;
        let mut leveled_up = false;
        
        while self.xp >= self.xp_for_next_level() && self.level < 200 {
            self.xp -= self.xp_for_next_level();
            self.level += 1;
            leveled_up = true;
        }
        leveled_up
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
