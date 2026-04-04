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
        let row = sqlx::query("SELECT id, username, total_attempts, successful_attempts, failed_attempts, last_fishing_time, level, xp FROM players WHERE username = ?")
            .bind(username)
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
            }),
            None => {
                let id = sqlx::query("INSERT INTO players (username) VALUES (?)")
                    .bind(username)
                    .execute(&self.pool)
                    .await?
                    .last_insert_rowid();

                Ok(Player::new(username.to_string()))
            }
        }
    }

    pub async fn get_leaderboard(&self) -> Result<Vec<Player>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, username, total_attempts, successful_attempts, failed_attempts, last_fishing_time, level, xp FROM players ORDER BY level DESC, xp DESC LIMIT 10")
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
        }).collect();

        Ok(players)
    }

    pub async fn add_cooldown_penalty(&self, player_id: i64) -> Result<(), sqlx::Error> {
        // On décale last_fishing_time de 5 secondes vers le futur (ou on l'initialise si inexistant)
        sqlx::query("UPDATE players SET last_fishing_time = DATETIME(COALESCE(last_fishing_time, CURRENT_TIMESTAMP), '+5 seconds') WHERE id = ?")
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn save_attempt(&self, player: &Player, success: bool, fish: Option<Fish>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Update player stats
        sqlx::query("UPDATE players SET total_attempts = total_attempts + 1, successful_attempts = successful_attempts + ?, failed_attempts = failed_attempts + ?, last_fishing_time = ?, level = ?, xp = ? WHERE id = ?")
            .bind(if success { 1 } else { 0 })
            .bind(if success { 0 } else { 1 })
            .bind(Utc::now())
            .bind(player.level)
            .bind(player.xp)
            .bind(player.id)
            .execute(&mut *tx)
            .await?;

        // Save catch if success
        if let Some(f) = fish {
            sqlx::query("INSERT INTO catches (player_id, fish_name, rarity, size, state, description) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(player.id)
                .bind(f.name)
                .bind(serde_json::to_string(&f.rarity).unwrap_or_default())
                .bind(f.size)
                .bind(f.state)
                .bind(f.description)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
