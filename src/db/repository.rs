use sqlx::{SqlitePool, Row};
use crate::models::{Player, Fish};
use chrono::Utc;

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_or_create_player(&self, username: &str) -> Result<Player, sqlx::Error> {
        let username_lower = username.to_lowercase();
        let row = sqlx::query("SELECT id, username, total_attempts, successful_attempts, failed_attempts, last_fishing_time, level, xp, vip_until FROM players WHERE username = ?")
            .bind(&username_lower)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Player {
                id: Some(row.get("id")),
                username: row.get("username"),
                total_attempts: row.get("total_attempts"),
                successful_attempts: row.get("successful_attempts"),
                failed_attempts: row.get("failed_attempts"),
                last_fishing_time: row.get("last_fishing_time"),
                level: row.get("level"),
                xp: row.get("xp"),
                vip_until: row.get("vip_until"),
            }),
            None => {
                let id = sqlx::query("INSERT INTO players (username) VALUES (?)")
                    .bind(&username_lower)
                    .execute(&self.pool)
                    .await?
                    .last_insert_rowid();

                let mut new_player = Player::new(username_lower);
                new_player.id = Some(id);
                Ok(new_player)
            }
        }
    }

    pub async fn get_leaderboard(&self) -> Result<Vec<Player>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, username, total_attempts, successful_attempts, failed_attempts, last_fishing_time, level, xp, vip_until FROM players WHERE total_attempts > 0 ORDER BY level DESC, xp DESC LIMIT 10")
            .fetch_all(&self.pool)
            .await?;

        let players = rows.into_iter().map(|row| Player {
            id: Some(row.get("id")),
            username: row.get("username"),
            total_attempts: row.get("total_attempts"),
            successful_attempts: row.get("successful_attempts"),
            failed_attempts: row.get("failed_attempts"),
            last_fishing_time: row.get("last_fishing_time"),
            level: row.get("level"),
            xp: row.get("xp"),
            vip_until: row.get("vip_until"),
        }).collect();

        Ok(players)
    }

    pub async fn get_player_catches(&self, player_id: i64) -> Result<Vec<Fish>, sqlx::Error> {
        let rows = sqlx::query("SELECT fish_name, rarity, size, weight, state, description FROM catches WHERE player_id = ?")
            .bind(player_id)
            .fetch_all(&self.pool)
            .await?;

        let catches = rows.into_iter().map(|row| {
            let rarity_str: String = row.get("rarity");
            let cleaned_rarity = rarity_str.trim_matches('"');
            let rarity = match cleaned_rarity.to_lowercase().as_str() {
                "uncommon" => crate::config::Rarity::Uncommon,
                "rare" => crate::config::Rarity::Rare,
                "veryrare" | "very rare" => crate::config::Rarity::VeryRare,
                "epic" => crate::config::Rarity::Epic,
                "legendary" => crate::config::Rarity::Legendary,
                "mythical" => crate::config::Rarity::Mythical,
                _ => crate::config::Rarity::Common,
            };
            Fish::new(
                row.get("fish_name"),
                rarity,
                row.get("size"),
                row.get("weight"),
                row.get("state"),
                row.get("description"),
            )
        }).collect();

        Ok(catches)
    }

    pub async fn add_cooldown_penalty(&self, player_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE players SET last_fishing_time = DATETIME(COALESCE(last_fishing_time, CURRENT_TIMESTAMP), '+5 seconds') WHERE id = ?")
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn reset_player(&self, username: &str) -> Result<(), sqlx::Error> {
        let username_lower = username.to_lowercase();
        let mut tx = self.pool.begin().await?;

        // Supprimer les poissons capturés
        sqlx::query("DELETE FROM catches WHERE player_id IN (SELECT id FROM players WHERE username = ?)")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        // Réinitialiser les stats du joueur
        sqlx::query("UPDATE players SET total_attempts = 0, successful_attempts = 0, failed_attempts = 0, last_fishing_time = NULL, level = 1, xp = 0, vip_until = NULL WHERE username = ?")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn save_attempt(&self, player: &Player, success: bool, fish: Option<Fish>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("UPDATE players SET total_attempts = total_attempts + 1, successful_attempts = successful_attempts + ?, failed_attempts = failed_attempts + ?, last_fishing_time = ?, level = ?, xp = ?, vip_until = ? WHERE id = ?")
            .bind(if success { 1 } else { 0 })
            .bind(if success { 0 } else { 1 })
            .bind(Utc::now())
            .bind(player.level)
            .bind(player.xp)
            .bind(player.vip_until)
            .bind(player.id)
            .execute(&mut *tx)
            .await?;

        if let Some(f) = fish {
            sqlx::query("INSERT INTO catches (player_id, fish_name, rarity, size, weight, state, description, stream_title) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(player.id)
                .bind(f.name)
                .bind(serde_json::to_string(&f.rarity).unwrap_or_default())
                .bind(f.size)
                .bind(f.weight)
                .bind(f.state)
                .bind(f.description)
                .bind(f.stream_title)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
