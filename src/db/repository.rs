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
    #[serde(default)]
    pub gold: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BananaKingRecord {
    pub id: i64,
    pub player_id: i64,
    pub username: String,
    pub crowned_at: DateTime<Utc>,
    pub dethroned_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTrophy {
    pub id: Option<i64>,
    pub player_id: i64,
    pub username: String,
    pub season: String,
    pub trophy_tier: String,
    pub level: i32,
    pub unlocked_at: String,
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
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Gemme%') as gem_count \
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
            gem_count: row.get("gem_count"),
            profile_image_url: row.get("profile_image_url"),
            gold: row.get("gold"),
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
        let gold_val = backup.gold.unwrap_or(0);
        let id = sqlx::query("INSERT INTO players (username, total_attempts, successful_attempts, failed_attempts, level, xp, vip_until, gold) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?) \
            ON CONFLICT(username) DO UPDATE SET \
            total_attempts = excluded.total_attempts, \
            successful_attempts = excluded.successful_attempts, \
            failed_attempts = excluded.failed_attempts, \
            level = excluded.level, \
            xp = excluded.xp, \
            vip_until = excluded.vip_until, \
            gold = excluded.gold")
            .bind(&backup.username.to_lowercase())
            .bind(backup.total_attempts)
            .bind(backup.successful_attempts)
            .bind(backup.failed_attempts)
            .bind(backup.level)
            .bind(backup.xp)
            .bind(backup.vip_until)
            .bind(gold_val)
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
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Gemme%') as gem_count \
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
                gem_count: r.get("gem_count"),
                profile_image_url: r.get("profile_image_url"),
                gold: r.get("gold"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_expired_vips(&self) -> Result<Vec<Player>, sqlx::Error> {
        let rows = sqlx::query("SELECT *, 0 as junk_count, 0 as banana_count, 0 as postcard_count, 0 as gem_count \
            FROM players WHERE vip_until IS NOT NULL AND vip_until < ?")
            .bind(Utc::now())
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(|row| Player {
            id: Some(row.get::<i64, _>("id")),
            username: row.get("username"),
            total_attempts: row.get("total_attempts"),
            successful_attempts: row.get("successful_attempts"),
            failed_attempts: row.get("failed_attempts"),
            last_fishing_time: row.get::<Option<DateTime<Utc>>, _>("last_fishing_time"),
            level: row.get("level"),
            xp: row.get("xp"),
            vip_until: row.get::<Option<DateTime<Utc>>, _>("vip_until"),
            junk_count: 0,
            banana_count: 0,
            postcard_count: 0,
            gem_count: 0,
            profile_image_url: row.get("profile_image_url"),
            gold: row.get("gold"),
        }).collect())
    }

    pub async fn remove_vip_status(&self, player_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE players SET vip_until = NULL WHERE id = ?")
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_or_create_player(&self, username: &str) -> Result<Player, sqlx::Error> {
        let username_lower = username.to_lowercase();
        let row = sqlx::query("SELECT p.*, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND is_junk = 1) as junk_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Banana%') as banana_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Gemme%') as gem_count \
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
                gem_count: row.get("gem_count"),
                profile_image_url: row.get("profile_image_url"),
                gold: row.get("gold"),
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
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Gemme%') as gem_count \
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
            gem_count: row.get("gem_count"),
            profile_image_url: row.get("profile_image_url"),
            gold: row.get("gold"),
        }).collect();

        Ok(players)
    }

    pub async fn update_player_profile_image(&self, player_id: i64, profile_image_url: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE players SET profile_image_url = ? WHERE id = ?")
            .bind(profile_image_url)
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
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
            let is_banana = f.name == "Pristine Banana 1" || f.name == "Pristine Banana 2";
            if is_banana {
                sqlx::query("DELETE FROM catches WHERE fish_name = ? AND player_id != ?")
                    .bind(&f.name)
                    .bind(player.id)
                    .execute(&mut *tx)
                    .await?;
            }
            sqlx::query("INSERT INTO catches (player_id, fish_name, rarity, size, weight, state, description, stream_title, is_junk) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(player.id)
                .bind(&f.name)
                .bind(serde_json::to_string(&f.rarity).unwrap_or_default())
                .bind(f.size)
                .bind(f.weight)
                .bind(f.state)
                .bind(f.description)
                .bind(f.stream_title)
                .bind(f.is_junk)
                .execute(&mut *tx)
                .await?;

            if is_banana {
                let banana_count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM catches WHERE player_id = ? AND (fish_name = 'Pristine Banana 1' OR fish_name = 'Pristine Banana 2')"
                )
                .bind(player.id)
                .fetch_one(&mut *tx)
                .await?;

                if banana_count == 2 {
                    let is_already_king: i64 = sqlx::query_scalar(
                        "SELECT COUNT(*) FROM banana_kings_history WHERE player_id = ? AND dethroned_at IS NULL"
                    )
                    .bind(player.id)
                    .fetch_one(&mut *tx)
                    .await?;

                    if is_already_king == 0 {
                        // Dethrone any existing active King
                        sqlx::query("UPDATE banana_kings_history SET dethroned_at = CURRENT_TIMESTAMP WHERE dethroned_at IS NULL")
                            .execute(&mut *tx)
                            .await?;

                        // Insert new King record
                        sqlx::query("INSERT INTO banana_kings_history (player_id, username, dethroned_at) VALUES (?, ?, NULL)")
                            .bind(player.id)
                            .bind(&player.username)
                            .execute(&mut *tx)
                            .await?;
                    }
                }
            }
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn check_and_execute_banana_theft(
        &self,
        current_player_id: i64,
        banana_name: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let row: Option<sqlx::sqlite::SqliteRow> = sqlx::query(
            "SELECT p.id, p.username FROM catches c JOIN players p ON c.player_id = p.id WHERE c.fish_name = ? AND p.id != ?"
        )
        .bind(banana_name)
        .bind(current_player_id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(r) = row {
            use sqlx::Row;
            let old_player_id: i64 = r.get("id");
            let old_username: String = r.get("username");
            
            sqlx::query("DELETE FROM catches WHERE fish_name = ? AND player_id = ?")
                .bind(banana_name)
                .bind(old_player_id)
                .execute(&mut *tx)
                .await?;

            sqlx::query("UPDATE banana_kings_history SET dethroned_at = CURRENT_TIMESTAMP WHERE player_id = ? AND dethroned_at IS NULL")
                .bind(old_player_id)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;
            Ok(Some(old_username))
        } else {
            sqlx::query("DELETE FROM catches WHERE fish_name = ? AND player_id != ?")
                .bind(banana_name)
                .bind(current_player_id)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;
            Ok(None)
        }
    }

    pub async fn has_caught_banana(&self, player_id: i64, banana_name: &str) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM catches WHERE player_id = ? AND fish_name = ?")
            .bind(player_id)
            .bind(banana_name)
            .fetch_one(&self.pool)
            .await?;
        Ok(count > 0)
    }

    async fn check_and_update_banana_king_status(
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        player_id: i64,
    ) -> Result<(), sqlx::Error> {
        let username: String = sqlx::query_scalar("SELECT username FROM players WHERE id = ?")
            .bind(player_id)
            .fetch_one(&mut **tx)
            .await?;

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM catches WHERE player_id = ? AND (fish_name = 'Pristine Banana 1' OR fish_name = 'Pristine Banana 2')"
        )
        .bind(player_id)
        .fetch_one(&mut **tx)
        .await?;

        if count == 2 {
            let is_already_king: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM banana_kings_history WHERE player_id = ? AND dethroned_at IS NULL"
            )
            .bind(player_id)
            .fetch_one(&mut **tx)
            .await?;

            if is_already_king == 0 {
                // Dethrone any existing active King
                sqlx::query("UPDATE banana_kings_history SET dethroned_at = CURRENT_TIMESTAMP WHERE dethroned_at IS NULL")
                    .execute(&mut **tx)
                    .await?;

                // Insert new King record
                sqlx::query("INSERT INTO banana_kings_history (player_id, username, dethroned_at) VALUES (?, ?, NULL)")
                    .bind(player_id)
                    .bind(&username)
                    .execute(&mut **tx)
                    .await?;
            }
        } else {
            // If they are currently the active king, dethrone them
            sqlx::query("UPDATE banana_kings_history SET dethroned_at = CURRENT_TIMESTAMP WHERE player_id = ? AND dethroned_at IS NULL")
                .bind(player_id)
                .execute(&mut **tx)
                .await?;
        }
        Ok(())
    }

    pub async fn execute_gold_sale(
        &self,
        seller_id: i64,
        catch_ids: &[i64],
        gold_earned: i64,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        for &id in catch_ids {
            sqlx::query("DELETE FROM catches WHERE id = ? AND player_id = ?")
                .bind(id)
                .bind(seller_id)
                .execute(&mut *tx)
                .await?;
        }

        sqlx::query("UPDATE players SET gold = gold + ? WHERE id = ?")
            .bind(gold_earned)
            .bind(seller_id)
            .execute(&mut *tx)
            .await?;

        Self::check_and_update_banana_king_status(&mut tx, seller_id).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn execute_direct_trade(
        &self,
        catch_id: i64,
        seller_id: i64,
        buyer_id: i64,
        price: i64,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Transfer catch ownership
        let rows_affected = sqlx::query("UPDATE catches SET player_id = ? WHERE id = ? AND player_id = ?")
            .bind(buyer_id)
            .bind(catch_id)
            .bind(seller_id)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        // 2. Transfer gold
        sqlx::query("UPDATE players SET gold = gold - ? WHERE id = ?")
            .bind(price)
            .bind(buyer_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE players SET gold = gold + ? WHERE id = ?")
            .bind(price)
            .bind(seller_id)
            .execute(&mut *tx)
            .await?;

        // 3. Update Banana King status for both players
        Self::check_and_update_banana_king_status(&mut tx, seller_id).await?;
        Self::check_and_update_banana_king_status(&mut tx, buyer_id).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn execute_barter_trade(
        &self,
        catch_id_a: i64,
        player_id_a: i64,
        catch_id_b: i64,
        player_id_b: i64,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Swap ownership
        let rows_a = sqlx::query("UPDATE catches SET player_id = ? WHERE id = ? AND player_id = ?")
            .bind(player_id_b)
            .bind(catch_id_a)
            .bind(player_id_a)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        let rows_b = sqlx::query("UPDATE catches SET player_id = ? WHERE id = ? AND player_id = ?")
            .bind(player_id_a)
            .bind(catch_id_b)
            .bind(player_id_b)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        if rows_a == 0 || rows_b == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        // 2. Update Banana King status for both players
        Self::check_and_update_banana_king_status(&mut tx, player_id_a).await?;
        Self::check_and_update_banana_king_status(&mut tx, player_id_b).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_banana_kings_history(&self) -> Result<Vec<BananaKingRecord>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, player_id, username, crowned_at, dethroned_at FROM banana_kings_history ORDER BY crowned_at DESC, id DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let history = rows.into_iter().map(|row| BananaKingRecord {
            id: row.get("id"),
            player_id: row.get("player_id"),
            username: row.get("username"),
            crowned_at: row.get("crowned_at"),
            dethroned_at: row.get("dethroned_at"),
        }).collect();

        Ok(history)
    }

    pub async fn purge_all_data(&self) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Supprimer d'abord les index et tables existantes
        sqlx::query("DROP INDEX IF EXISTS idx_catches_player_id").execute(&mut *tx).await?;
        sqlx::query("DROP INDEX IF EXISTS idx_players_username").execute(&mut *tx).await?;
        sqlx::query("DROP INDEX IF EXISTS idx_trophies_player_id").execute(&mut *tx).await?;
        sqlx::query("DROP TABLE IF EXISTS catches").execute(&mut *tx).await?;
        sqlx::query("DROP TABLE IF EXISTS banana_kings_history").execute(&mut *tx).await?;
        sqlx::query("DROP TABLE IF EXISTS player_trophies").execute(&mut *tx).await?;
        sqlx::query("DROP TABLE IF EXISTS players").execute(&mut *tx).await?;

        // Supprimer toutes les entrées de sqlite_sequence
        let _ = sqlx::query("DELETE FROM sqlite_sequence").execute(&mut *tx).await;

        // Recréer la table players
        sqlx::query(
            "CREATE TABLE players (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                total_attempts INTEGER DEFAULT 0,
                successful_attempts INTEGER DEFAULT 0,
                failed_attempts INTEGER DEFAULT 0,
                last_fishing_time DATETIME,
                level INTEGER DEFAULT 1,
                xp INTEGER DEFAULT 0,
                vip_until DATETIME,
                profile_image_url TEXT,
                gold INTEGER DEFAULT 0
            )"
        ).execute(&mut *tx).await?;

        // Recréer la table catches
        sqlx::query(
            "CREATE TABLE catches (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER REFERENCES players(id),
                fish_name TEXT NOT NULL,
                rarity TEXT NOT NULL,
                size REAL NOT NULL,
                weight REAL DEFAULT 0,
                state TEXT NOT NULL,
                description TEXT,
                stream_title TEXT,
                caught_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                is_junk BOOLEAN DEFAULT 0
            )"
        ).execute(&mut *tx).await?;

        // Recréer la table banana_kings_history
        sqlx::query(
            "CREATE TABLE banana_kings_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER REFERENCES players(id),
                username TEXT NOT NULL,
                crowned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                dethroned_at DATETIME
            )"
        ).execute(&mut *tx).await?;

        // Recréer la table player_trophies
        sqlx::query(
            "CREATE TABLE player_trophies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
                username TEXT NOT NULL,
                season TEXT NOT NULL,
                trophy_tier TEXT NOT NULL,
                level INTEGER DEFAULT 1,
                unlocked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(player_id, season)
            )"
        ).execute(&mut *tx).await?;

        // Recréer les index
        sqlx::query("CREATE INDEX idx_catches_player_id ON catches(player_id);").execute(&mut *tx).await?;
        sqlx::query("CREATE INDEX idx_players_username ON players(username);").execute(&mut *tx).await?;
        sqlx::query("CREATE INDEX idx_trophies_player_id ON player_trophies(player_id);").execute(&mut *tx).await?;

        tx.commit().await?;

        // Supprimer le fichier de sauvegarde s'il existe pour éviter une auto-restauration au redémarrage
        let _ = std::fs::remove_file("data/players_backup.json");

        Ok(())
    }

    pub async fn get_player_trophies(&self, player_id: i64) -> Result<Vec<PlayerTrophy>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, player_id, username, season, trophy_tier, level, strftime('%Y-%m-%dT%H:%M:%SZ', unlocked_at) as unlocked_at FROM player_trophies WHERE player_id = ? ORDER BY unlocked_at DESC")
            .bind(player_id)
            .fetch_all(&self.pool)
            .await?;

        let trophies = rows.into_iter().map(|row| PlayerTrophy {
            id: Some(row.get("id")),
            player_id: row.get("player_id"),
            username: row.get("username"),
            season: row.get("season"),
            trophy_tier: row.get("trophy_tier"),
            level: row.get("level"),
            unlocked_at: row.get("unlocked_at"),
        }).collect();

        Ok(trophies)
    }

    pub async fn reset_player_all(&self, username: &str) -> Result<(), sqlx::Error> {
        let username_lower = username.to_lowercase();
        let mut tx = self.pool.begin().await?;

        // 1. Supprimer ses trophées éternels
        sqlx::query("DELETE FROM player_trophies WHERE player_id IN (SELECT id FROM players WHERE username = ?)")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        // 2. Supprimer les poissons capturés
        sqlx::query("DELETE FROM catches WHERE player_id IN (SELECT id FROM players WHERE username = ?)")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        // 3. Réinitialiser les stats du joueur
        sqlx::query("UPDATE players SET total_attempts = 0, successful_attempts = 0, failed_attempts = 0, last_fishing_time = NULL, level = 1, xp = 0, vip_until = NULL WHERE username = ?")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn archive_and_reset_season(&self, season_name: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Calculer et enregistrer les trophées de niveau pour tous les joueurs avec level >= 10
        let players = sqlx::query("SELECT id, username, level FROM players WHERE level >= 10")
            .fetch_all(&self.pool)
            .await?;

        for p in players {
            let player_id: i64 = p.get("id");
            let username: String = p.get("username");
            let level: i32 = p.get("level");

            let tier = if level >= 150 {
                "Obsidienne"
            } else if level >= 100 {
                "Diamant"
            } else if level >= 70 {
                "Platinium"
            } else if level >= 40 {
                "Or"
            } else if level >= 20 {
                "Argent"
            } else {
                "Bronze"
            };

            // Insérer ou remplacer le trophée de niveau
            let _ = sqlx::query("INSERT OR REPLACE INTO player_trophies (player_id, username, season, trophy_tier, level) VALUES (?, ?, ?, ?, ?)")
                .bind(player_id)
                .bind(&username)
                .bind(season_name)
                .bind(tier)
                .bind(level)
                .execute(&mut *tx)
                .await;
        }

        // 2. Calculer le Trophée de la Night 🌙
        // Le joueur avec le plus de prises réussies après 22h (entre 22h et 4h)
        if let Ok(Some(row)) = sqlx::query("SELECT player_id, username, COUNT(*) as c FROM catches JOIN players ON catches.player_id = players.id WHERE is_junk = 0 AND (strftime('%H', caught_at) >= '22' OR strftime('%H', caught_at) < '04') GROUP BY player_id ORDER BY c DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await 
        {
            let player_id: i64 = row.get("player_id");
            let username: String = row.get("username");
            let _ = sqlx::query("INSERT OR REPLACE INTO player_trophies (player_id, username, season, trophy_tier, level) VALUES (?, ?, ?, ?, ?)")
                .bind(player_id)
                .bind(&username)
                .bind(format!("{} (Night 🌙)", season_name))
                .bind("Night")
                .bind(0)
                .execute(&mut *tx)
                .await;
        }

        // 3. Calculer le Trophée Roi des Voleurs 🍌
        // Le joueur avec le plus de dethronements
        if let Ok(Some(row)) = sqlx::query("SELECT player_id, username, COUNT(*) as c FROM banana_kings_history GROUP BY player_id ORDER BY c DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await 
        {
            let player_id: i64 = row.get("player_id");
            let username: String = row.get("username");
            let _ = sqlx::query("INSERT OR REPLACE INTO player_trophies (player_id, username, season, trophy_tier, level) VALUES (?, ?, ?, ?, ?)")
                .bind(player_id)
                .bind(&username)
                .bind(format!("{} (Voleur 🍌)", season_name))
                .bind("Voleur")
                .bind(0)
                .execute(&mut *tx)
                .await;
        }

        // 4. Calculer le Trophée Éboueur des Mers 🧹
        // Le joueur avec le plus de déchets
        if let Ok(Some(row)) = sqlx::query("SELECT player_id, username, COUNT(*) as c FROM catches JOIN players ON catches.player_id = players.id WHERE is_junk = 1 GROUP BY player_id ORDER BY c DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await 
        {
            let player_id: i64 = row.get("player_id");
            let username: String = row.get("username");
            let _ = sqlx::query("INSERT OR REPLACE INTO player_trophies (player_id, username, season, trophy_tier, level) VALUES (?, ?, ?, ?, ?)")
                .bind(player_id)
                .bind(&username)
                .bind(format!("{} (Éboueur 🧹)", season_name))
                .bind("Eboueur")
                .bind(0)
                .execute(&mut *tx)
                .await;
        }

        // 5. Calculer le Trophée Pêcheur Divin 👑
        // Le joueur avec le plus de poissons rares/divins
        if let Ok(Some(row)) = sqlx::query("SELECT player_id, username, COUNT(*) as c FROM catches JOIN players ON catches.player_id = players.id WHERE is_junk = 0 AND rarity IN ('divin', 'mythical', 'legendary') GROUP BY player_id ORDER BY c DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await 
        {
            let player_id: i64 = row.get("player_id");
            let username: String = row.get("username");
            let _ = sqlx::query("INSERT OR REPLACE INTO player_trophies (player_id, username, season, trophy_tier, level) VALUES (?, ?, ?, ?, ?)")
                .bind(player_id)
                .bind(&username)
                .bind(format!("{} (Pêcheur Divin 👑)", season_name))
                .bind("Divin")
                .bind(0)
                .execute(&mut *tx)
                .await;
        }

        // 6. Purger toutes les captures et historique de bananes actives
        sqlx::query("DELETE FROM catches").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM banana_kings_history").execute(&mut *tx).await?;

        // 7. Réinitialiser les statistiques actives des joueurs
        sqlx::query("UPDATE players SET total_attempts = 0, successful_attempts = 0, failed_attempts = 0, last_fishing_time = NULL, level = 1, xp = 0")
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        // Supprimer le fichier de sauvegarde backup pour éviter l'auto-restauration
        let _ = std::fs::remove_file("data/players_backup.json");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use crate::models::{Fish, Player};
    use crate::config::Rarity;

    async fn setup_db() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE players (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                total_attempts INTEGER DEFAULT 0,
                successful_attempts INTEGER DEFAULT 0,
                failed_attempts INTEGER DEFAULT 0,
                last_fishing_time DATETIME,
                level INTEGER DEFAULT 1,
                xp INTEGER DEFAULT 0,
                vip_until DATETIME,
                profile_image_url TEXT,
                gold INTEGER DEFAULT 0
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE catches (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER REFERENCES players(id),
                fish_name TEXT NOT NULL,
                rarity TEXT NOT NULL,
                size REAL NOT NULL,
                weight REAL DEFAULT 0,
                state TEXT NOT NULL,
                description TEXT,
                stream_title TEXT,
                caught_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                is_junk BOOLEAN DEFAULT 0
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE banana_kings_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER REFERENCES players(id),
                username TEXT NOT NULL,
                crowned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                dethroned_at DATETIME
            )"
        ).execute(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_player_creation_and_retrieval() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        let p1 = repo.get_or_create_player("test_user").await.unwrap();
        assert_eq!(p1.username, "test_user");
        assert_eq!(p1.level, 1);

        let p2 = repo.get_player("test_user").await.unwrap();
        assert!(p2.is_some());
        assert_eq!(p2.unwrap().id, p1.id);
    }

    #[tokio::test]
    async fn test_save_attempt() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);
        let player = repo.get_or_create_player("fisher").await.unwrap();

        let fish = Fish::new("Goldfish".to_string(), Rarity::Common, 10.0, 50.0, "good".to_string(), "A fish".to_string());
        
        let result = repo.save_attempt(&player, true, Some(fish)).await;
        assert!(result.is_ok());

        let catches = repo.get_player_catches(player.id.unwrap()).await.unwrap();
        assert_eq!(catches.len(), 1);
        assert_eq!(catches[0].name, "Goldfish");
    }

    #[tokio::test]
    async fn test_banana_theft() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);
        
        let p_a = repo.get_or_create_player("player_a").await.unwrap();
        let p_b = repo.get_or_create_player("player_b").await.unwrap();

        // 1. Player A catches Pristine Banana 1
        let b1_a = Fish::new("Pristine Banana 1".to_string(), Rarity::Divin, 20.0, 150.0, "pristine".to_string(), "Banana 1".to_string());
        repo.save_attempt(&p_a, true, Some(b1_a)).await.unwrap();

        // Verify Player A has it
        let catches_a = repo.get_player_catches(p_a.id.unwrap()).await.unwrap();
        assert!(catches_a.iter().any(|c| c.name == "Pristine Banana 1"));

        // No King yet (only 1 banana)
        let hist_0 = repo.get_banana_kings_history().await.unwrap();
        assert_eq!(hist_0.len(), 0);

        // 2. Player A catches Pristine Banana 2 (Crowning!)
        let b2_a = Fish::new("Pristine Banana 2".to_string(), Rarity::Divin, 20.5, 152.0, "pristine".to_string(), "Banana 2".to_string());
        repo.save_attempt(&p_a, true, Some(b2_a)).await.unwrap();

        // Player A should now be King
        let hist_1 = repo.get_banana_kings_history().await.unwrap();
        assert_eq!(hist_1.len(), 1);
        assert_eq!(hist_1[0].username, "player_a");
        assert!(hist_1[0].dethroned_at.is_none());

        // 3. Player B catches Pristine Banana 1 (Theft & Dethroning Player A!)
        let stolen_from = repo.check_and_execute_banana_theft(p_b.id.unwrap(), "Pristine Banana 1").await.unwrap();
        assert_eq!(stolen_from, Some("player_a".to_string()));

        // Player A should be dethroned now!
        let hist_2 = repo.get_banana_kings_history().await.unwrap();
        assert_eq!(hist_2.len(), 1);
        assert_eq!(hist_2[0].username, "player_a");
        assert!(hist_2[0].dethroned_at.is_some());

        // Player B saves their Pristine Banana 1 catch
        let b1_b = Fish::new("Pristine Banana 1".to_string(), Rarity::Divin, 21.0, 155.0, "pristine".to_string(), "Banana 1".to_string());
        repo.save_attempt(&p_b, true, Some(b1_b)).await.unwrap();

        // Verify Player A lost banana 1 but retains banana 2
        let catches_a_after = repo.get_player_catches(p_a.id.unwrap()).await.unwrap();
        assert!(!catches_a_after.iter().any(|c| c.name == "Pristine Banana 1"));
        assert!(catches_a_after.iter().any(|c| c.name == "Pristine Banana 2"));

        // Verify Player B has banana 1
        let catches_b = repo.get_player_catches(p_b.id.unwrap()).await.unwrap();
        assert!(catches_b.iter().any(|c| c.name == "Pristine Banana 1"));

        // 4. Player B catches Pristine Banana 2 (Theft!)
        let stolen_from_2 = repo.check_and_execute_banana_theft(p_b.id.unwrap(), "Pristine Banana 2").await.unwrap();
        assert_eq!(stolen_from_2, Some("player_a".to_string()));

        // Player B saves their Pristine Banana 2 catch (Crowning Player B!)
        let b2_b = Fish::new("Pristine Banana 2".to_string(), Rarity::Divin, 22.0, 160.0, "pristine".to_string(), "Banana 2".to_string());
        repo.save_attempt(&p_b, true, Some(b2_b)).await.unwrap();

        // Verify Player B has both
        let has_b1 = repo.has_caught_banana(p_b.id.unwrap(), "Pristine Banana 1").await.unwrap();
        let has_b2 = repo.has_caught_banana(p_b.id.unwrap(), "Pristine Banana 2").await.unwrap();
        assert!(has_b1);
        assert!(has_b2);

        // Verify history contains 2 entries now: Player B (active) and Player A (dethroned)
        let hist_3 = repo.get_banana_kings_history().await.unwrap();
        assert_eq!(hist_3.len(), 2);

        // Order is DESC by crowned_at, so Player B is first
        assert_eq!(hist_3[0].username, "player_b");
        assert!(hist_3[0].dethroned_at.is_none());

        assert_eq!(hist_3[1].username, "player_a");
        assert!(hist_3[1].dethroned_at.is_some());
    }

    #[tokio::test]
    async fn test_purge_all_data() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        let p_id = repo.restore_player(&crate::db::PlayerBackup {
            username: "player_a".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(0),
        }).await.unwrap();

        let fish = Fish::new("Brochet".to_string(), Rarity::Common, 12.5, 2.4, "state".to_string(), "desc".to_string());
        repo.save_attempt(&repo.get_player("player_a").await.unwrap().unwrap(), true, Some(fish)).await.unwrap();

        // Verify data exists
        assert!(repo.count_players().await.unwrap() > 0);
        let players = repo.get_all_players().await.unwrap();
        let catches = repo.get_player_catches(p_id).await.unwrap();
        assert_eq!(players.len(), 1);
        assert_eq!(catches.len(), 1);

        // Purge
        repo.purge_all_data().await.unwrap();

        // Verify data is completely empty
        assert_eq!(repo.count_players().await.unwrap(), 0);
        let players_after = repo.get_all_players().await.unwrap();
        assert_eq!(players_after.len(), 0);
    }
}
