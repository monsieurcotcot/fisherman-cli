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
    pub gem_count: i64,
    pub profile_image_url: Option<String>,
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
            gem_count: 0,
            profile_image_url: None,
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

    /// Calcule le cooldown dynamique basé sur le niveau (40s au niv 1 vers 25s au niv 200)
    /// Si le joueur est VIP, le cooldown est divisé par 2.
    pub fn get_current_cooldown(&self) -> i64 {
        let base = 40.0;
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_player_xp_and_leveling() {
        let mut player = Player::new("test_user".to_string());
        assert_eq!(player.level, 1);
        assert_eq!(player.xp, 0);

        // Level 1 needs 1 * 40 + 50 = 90 XP
        assert_eq!(player.xp_for_next_level(), 90);

        // Add 50 XP -> No level up
        let leveled = player.add_xp(50);
        assert!(!leveled);
        assert_eq!(player.level, 1);
        assert_eq!(player.xp, 50);

        // Add 40 XP -> Total 90 XP -> Level Up
        let leveled = player.add_xp(40);
        assert!(leveled);
        assert_eq!(player.level, 2);
        assert_eq!(player.xp, 0);

        // Level 2 needs 2 * 40 + 50 = 130 XP
        assert_eq!(player.xp_for_next_level(), 130);
    }

    #[test]
    fn test_player_cooldown() {
        let mut player = Player::new("test_user".to_string());
        
        // Base cooldown at level 1 is 40 seconds
        assert_eq!(player.get_current_cooldown(), 40);

        // Level 200 cooldown calculation: 40 - 199 * (15/199) = 25 seconds
        player.level = 200;
        assert_eq!(player.get_current_cooldown(), 25);

        // VIP cuts cooldown in half
        player.vip_until = Some(Utc::now() + Duration::minutes(10));
        assert_eq!(player.get_current_cooldown(), 12); // 25 / 2 = 12
    }

    #[test]
    fn test_can_fish() {
        let mut player = Player::new("test_user".to_string());
        
        // Never fished before -> can fish
        assert!(player.can_fish());

        // Fished just now
        player.last_fishing_time = Some(Utc::now());
        assert!(!player.can_fish());
        
        // Fished 41 seconds ago (cooldown is 40s)
        player.last_fishing_time = Some(Utc::now() - Duration::seconds(41));
        assert!(player.can_fish());
    }
}
