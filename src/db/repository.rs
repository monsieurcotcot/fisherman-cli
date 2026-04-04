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
        let row = sqlx::query("SELECT id, username, total_attempts, successful_attempts, failed_attempts, last_fishing_time FROM players WHERE username = ?")
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
            }),
            None => {
                let id = sqlx::query("INSERT INTO players (username) VALUES (?)")
                    .bind(username)
                    .execute(&self.pool)
                    .await?
                    .last_insert_rowid();

                Ok(Player {
                    id: Some(id),
                    username: username.to_string(),
                    total_attempts: 0,
                    successful_attempts: 0,
                    failed_attempts: 0,
                    last_fishing_time: None,
                })
            }
        }
    }

    pub async fn get_leaderboard(&self) -> Result<Vec<Player>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, username, total_attempts, successful_attempts, failed_attempts, last_fishing_time FROM players ORDER BY successful_attempts DESC LIMIT 10")
            .fetch_all(&self.pool)
            .await?;

        let players = rows.into_iter().map(|row| Player {
            id: Some(row.get("id")),
            username: row.get("username"),
            total_attempts: row.get("total_attempts"),
            successful_attempts: row.get("successful_attempts"),
            failed_attempts: row.get("failed_attempts"),
            last_fishing_time: row.get("last_fishing_time"),
        }).collect();

        Ok(players)
    }

    pub async fn save_attempt(&self, player: &Player, success: bool, fish: Option<Fish>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Update player stats
        sqlx::query("UPDATE players SET total_attempts = total_attempts + 1, successful_attempts = successful_attempts + ?, failed_attempts = failed_attempts + ?, last_fishing_time = ? WHERE id = ?")
            .bind(if success { 1 } else { 0 })
            .bind(if success { 0 } else { 1 })
            .bind(Utc::now())
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
