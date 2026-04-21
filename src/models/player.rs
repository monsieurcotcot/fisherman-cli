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
    pub vip_until: Option<DateTime<Utc>>,
    pub junk_count: i64,
    pub banana_count: i64,
    pub postcard_count: i64,
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
            vip_until: None,
            junk_count: 0,
            banana_count: 0,
            postcard_count: 0,
        }
    }

    pub fn is_vip(&self) -> bool {
        match self.vip_until {
            Some(until) => until > Utc::now(),
            None => false,
        }
    }

    pub fn xp_for_next_level(&self) -> i64 {
        // Nouvelle formule linéaire : Niveau * 40 + 50
        // Permet d'atteindre le Niv 200 de manière réaliste pour un viewer régulier.
        (self.level as i64 * 40) + 50
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

    /// Calcule le cooldown dynamique basé sur le niveau (30s au niv 1 vers 15s au niv 200)
    /// Si le joueur est VIP, le cooldown est divisé par 2.
    pub fn get_current_cooldown(&self) -> i64 {
        let base = 30.0;
        let reduction = (self.level as f64 - 1.0) * (15.0 / 199.0);
        let mut cooldown = (base - reduction).round() as i64;
        
        if self.is_vip() {
            cooldown /= 2;
        }
        
        cooldown
    }

    pub fn get_remaining_cooldown(&self) -> i64 {
        let cooldown_seconds = self.get_current_cooldown();
        match self.last_fishing_time {
            Some(last_time) => {
                let now = Utc::now();
                let diff = now.signed_duration_since(last_time).num_seconds();
                if diff >= cooldown_seconds { 0 } else { cooldown_seconds - diff }
            }
            None => 0,
        }
    }

    pub fn can_fish(&self) -> bool {
        let cooldown_seconds = self.get_current_cooldown();
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
