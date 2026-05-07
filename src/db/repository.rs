use sqlx::{SqlitePool, Row};
use crate::models::{Player, Fish};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerBackup {
    pub username: String,
    pub total_attempts: i64,
    pub successful_attempts: i64,
    pub failed_attempts: i64,
    pub level: i32,
    pub xp: i64,
    pub vip_until: Option<DateTime<Utc>>,
}

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all_players(&self) -> Result<Vec<Player>, sqlx::Error> {
        let rows = sqlx::query("SELECT p.*, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND is_junk = 1) as junk_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Banana%') as banana_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count \
            FROM players p")
            .fetch_all(&self.pool)
            .await?;

        let players = rows.into_iter().map(|row| Player {
            id: Some(row.get::<i64, _>("id")),
            username: row.get("username"),
            total_attempts: row.get("total_attempts"),
            successful_attempts: row.get("successful_attempts"),
            failed_attempts: row.get("failed_attempts"),
            last_fishing_time: row.get::<Option<DateTime<Utc>>, _>("last_fishing_time"),
            level: row.get("level"),
            xp: row.get("xp"),
            vip_until: row.get::<Option<DateTime<Utc>>, _>("vip_until"),
            junk_count: row.get("junk_count"),
            banana_count: row.get("banana_count"),
            postcard_count: row.get("postcard_count"),
        }).collect();

        Ok(players)
    }

    pub async fn count_players(&self) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM players")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    pub async fn restore_player(&self, backup: &PlayerBackup) -> Result<i64, sqlx::Error> {
        let id = sqlx::query("INSERT INTO players (username, total_attempts, successful_attempts, failed_attempts, level, xp, vip_until) \
            VALUES (?, ?, ?, ?, ?, ?, ?) \
            ON CONFLICT(username) DO UPDATE SET \
            total_attempts = excluded.total_attempts, \
            successful_attempts = excluded.successful_attempts, \
            failed_attempts = excluded.failed_attempts, \
            level = excluded.level, \
            xp = excluded.xp, \
            vip_until = excluded.vip_until")
            .bind(&backup.username.to_lowercase())
            .bind(backup.total_attempts)
            .bind(backup.successful_attempts)
            .bind(backup.failed_attempts)
            .bind(backup.level)
            .bind(backup.xp)
            .bind(backup.vip_until)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();
        Ok(id)
    }

    pub async fn update_player_stats_after_restore(&self, player_id: i64, successful: i64, failed: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE players SET successful_attempts = ?, failed_attempts = ? WHERE id = ?")
            .bind(successful)
            .bind(failed)
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_player(&self, username: &str) -> Result<Option<Player>, sqlx::Error> {
        let username_lower = username.to_lowercase();
        let row = sqlx::query("SELECT p.*, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND is_junk = 1) as junk_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Banana%') as banana_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count \
            FROM players p WHERE p.username = ?")
            .bind(&username_lower)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(r) = row {
            Ok(Some(Player {
                id: Some(r.get::<i64, _>("id")),
                username: r.get("username"),
                total_attempts: r.get("total_attempts"),
                successful_attempts: r.get("successful_attempts"),
                failed_attempts: r.get("failed_attempts"),
                last_fishing_time: r.get::<Option<DateTime<Utc>>, _>("last_fishing_time"),
                level: r.get("level"),
                xp: r.get("xp"),
                vip_until: r.get::<Option<DateTime<Utc>>, _>("vip_until"),
                junk_count: r.get("junk_count"),
                banana_count: r.get("banana_count"),
                postcard_count: r.get("postcard_count"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_or_create_player(&self, username: &str) -> Result<Player, sqlx::Error> {
        let username_lower = username.to_lowercase();
        let row = sqlx::query("SELECT p.*, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND is_junk = 1) as junk_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Banana%') as banana_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count \
            FROM players p WHERE p.username = ?")
            .bind(&username_lower)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Player {
                id: Some(row.get::<i64, _>("id")),
                username: row.get("username"),
                total_attempts: row.get("total_attempts"),
                successful_attempts: row.get("successful_attempts"),
                failed_attempts: row.get("failed_attempts"),
                last_fishing_time: row.get::<Option<DateTime<Utc>>, _>("last_fishing_time"),
                level: row.get("level"),
                xp: row.get("xp"),
                vip_until: row.get::<Option<DateTime<Utc>>, _>("vip_until"),
                junk_count: row.get("junk_count"),
                banana_count: row.get("banana_count"),
                postcard_count: row.get("postcard_count"),
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
        let rows = sqlx::query("SELECT p.*, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND is_junk = 1) as junk_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Banana%') as banana_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count \
            FROM players p WHERE p.total_attempts > 0 ORDER BY p.level DESC, p.xp DESC LIMIT 10")
            .fetch_all(&self.pool)
            .await?;

        let players = rows.into_iter().map(|row| Player {
            id: Some(row.get::<i64, _>("id")),
            username: row.get("username"),
            total_attempts: row.get("total_attempts"),
            successful_attempts: row.get("successful_attempts"),
            failed_attempts: row.get("failed_attempts"),
            last_fishing_time: row.get::<Option<DateTime<Utc>>, _>("last_fishing_time"),
            level: row.get("level"),
            xp: row.get("xp"),
            vip_until: row.get::<Option<DateTime<Utc>>, _>("vip_until"),
            junk_count: row.get("junk_count"),
            banana_count: row.get("banana_count"),
            postcard_count: row.get("postcard_count"),
        }).collect();

        Ok(players)
    }

    pub async fn get_player_catches(&self, player_id: i64) -> Result<Vec<Fish>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, fish_name, rarity, size, weight, state, description, stream_title, caught_at, is_junk FROM catches WHERE player_id = ? ORDER BY caught_at DESC")
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
                "divin" => crate::config::Rarity::Divin,
                _ => crate::config::Rarity::Common,
            };
            let mut fish = if row.get::<bool, _>("is_junk") {
                Fish::new_junk(
                    row.get("fish_name"),
                    rarity,
                    row.get("size"),
                    row.get("weight"),
                    row.get("state"),
                    row.get("description"),
                )
            } else {
                Fish::new(
                    row.get("fish_name"),
                    rarity,
                    row.get("size"),
                    row.get("weight"),
                    row.get("state"),
                    row.get("description"),
                )
            };
            fish.id = Some(row.get("id"));
            fish.stream_title = row.get("stream_title");
            fish.caught_at = row.get("caught_at");
            fish
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

    pub async fn save_catch_only(&self, player_id: i64, fish: Fish) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO catches (player_id, fish_name, rarity, size, weight, state, description, stream_title, is_junk) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(player_id)
            .bind(fish.name)
            .bind(serde_json::to_string(&fish.rarity).unwrap_or_default())
            .bind(fish.size)
            .bind(fish.weight)
            .bind(fish.state)
            .bind(fish.description)
            .bind(fish.stream_title)
            .bind(fish.is_junk)
            .execute(&self.pool)
            .await?;
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
            sqlx::query("INSERT INTO catches (player_id, fish_name, rarity, size, weight, state, description, stream_title, is_junk) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(player.id)
                .bind(f.name)
                .bind(serde_json::to_string(&f.rarity).unwrap_or_default())
                .bind(f.size)
                .bind(f.weight)
                .bind(f.state)
                .bind(f.description)
                .bind(f.stream_title)
                .bind(f.is_junk)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
