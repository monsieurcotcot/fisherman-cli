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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MuseumDiscovery {
    pub id: Option<i64>,
    pub player_id: i64,
    pub username: String,
    pub fish_name: String,
    pub rarity: String,
    pub max_size: f64,
    pub max_weight: f64,
    pub best_state: String,
    pub description: Option<String>,
    pub total_caught: i32,
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
            last_daily_reward_at: row.get::<Option<DateTime<Utc>>, _>("last_daily_reward_at"),
            consecutive_days: row.get("consecutive_days"),
            total_days: row.get("total_days"),
            coinflip_wins: row.get("coinflip_wins"),
            coinflip_losses: row.get("coinflip_losses"),
            coinflip_biggest_win: row.get("coinflip_biggest_win"),
            coinflip_biggest_loss: row.get("coinflip_biggest_loss"),
            coinflip_gold_won_total: row.get("coinflip_gold_won_total"),
            coinflip_gold_lost_total: row.get("coinflip_gold_lost_total"),
            coinflip_current_win_streak: row.get("coinflip_current_win_streak"),
            coinflip_current_loss_streak: row.get("coinflip_current_loss_streak"),
            coinflip_max_win_streak: row.get("coinflip_max_win_streak"),
            coinflip_max_loss_streak: row.get("coinflip_max_loss_streak"),
            gold_given_total: row.get("gold_given_total"),
            max_gold_held: row.get("max_gold_held"),
            language: row.get("language"),
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
                last_daily_reward_at: r.get::<Option<DateTime<Utc>>, _>("last_daily_reward_at"),
                consecutive_days: r.get("consecutive_days"),
                total_days: r.get("total_days"),
                coinflip_wins: r.get("coinflip_wins"),
                coinflip_losses: r.get("coinflip_losses"),
                coinflip_biggest_win: r.get("coinflip_biggest_win"),
                coinflip_biggest_loss: r.get("coinflip_biggest_loss"),
                coinflip_gold_won_total: r.get("coinflip_gold_won_total"),
                coinflip_gold_lost_total: r.get("coinflip_gold_lost_total"),
                coinflip_current_win_streak: r.get("coinflip_current_win_streak"),
                coinflip_current_loss_streak: r.get("coinflip_current_loss_streak"),
                coinflip_max_win_streak: r.get("coinflip_max_win_streak"),
                coinflip_max_loss_streak: r.get("coinflip_max_loss_streak"),
                gold_given_total: r.get("gold_given_total"),
                max_gold_held: r.get("max_gold_held"),
                language: r.get("language"),
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
            last_daily_reward_at: row.get::<Option<DateTime<Utc>>, _>("last_daily_reward_at"),
            consecutive_days: row.get("consecutive_days"),
            total_days: row.get("total_days"),
            coinflip_wins: row.get("coinflip_wins"),
            coinflip_losses: row.get("coinflip_losses"),
            coinflip_biggest_win: row.get("coinflip_biggest_win"),
            coinflip_biggest_loss: row.get("coinflip_biggest_loss"),
            coinflip_gold_won_total: row.get("coinflip_gold_won_total"),
            coinflip_gold_lost_total: row.get("coinflip_gold_lost_total"),
            coinflip_current_win_streak: row.get("coinflip_current_win_streak"),
            coinflip_current_loss_streak: row.get("coinflip_current_loss_streak"),
            coinflip_max_win_streak: row.get("coinflip_max_win_streak"),
            coinflip_max_loss_streak: row.get("coinflip_max_loss_streak"),
            gold_given_total: row.get("gold_given_total"),
            max_gold_held: row.get("max_gold_held"),
            language: row.get("language"),
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
                last_daily_reward_at: row.get::<Option<DateTime<Utc>>, _>("last_daily_reward_at"),
                consecutive_days: row.get("consecutive_days"),
                total_days: row.get("total_days"),
                coinflip_wins: row.get("coinflip_wins"),
                coinflip_losses: row.get("coinflip_losses"),
                coinflip_biggest_win: row.get("coinflip_biggest_win"),
                coinflip_biggest_loss: row.get("coinflip_biggest_loss"),
                coinflip_gold_won_total: row.get("coinflip_gold_won_total"),
                coinflip_gold_lost_total: row.get("coinflip_gold_lost_total"),
                coinflip_current_win_streak: row.get("coinflip_current_win_streak"),
                coinflip_current_loss_streak: row.get("coinflip_current_loss_streak"),
                coinflip_max_win_streak: row.get("coinflip_max_win_streak"),
                coinflip_max_loss_streak: row.get("coinflip_max_loss_streak"),
                gold_given_total: row.get("gold_given_total"),
                max_gold_held: row.get("max_gold_held"),
                language: row.get("language"),
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
            last_daily_reward_at: row.get::<Option<DateTime<Utc>>, _>("last_daily_reward_at"),
            consecutive_days: row.get("consecutive_days"),
            total_days: row.get("total_days"),
            coinflip_wins: row.get("coinflip_wins"),
            coinflip_losses: row.get("coinflip_losses"),
            coinflip_biggest_win: row.get("coinflip_biggest_win"),
            coinflip_biggest_loss: row.get("coinflip_biggest_loss"),
            coinflip_gold_won_total: row.get("coinflip_gold_won_total"),
            coinflip_gold_lost_total: row.get("coinflip_gold_lost_total"),
            coinflip_current_win_streak: row.get("coinflip_current_win_streak"),
            coinflip_current_loss_streak: row.get("coinflip_current_loss_streak"),
            coinflip_max_win_streak: row.get("coinflip_max_win_streak"),
            coinflip_max_loss_streak: row.get("coinflip_max_loss_streak"),
            gold_given_total: row.get("gold_given_total"),
            max_gold_held: row.get("max_gold_held"),
            language: row.get("language"),
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

    pub async fn claim_daily_reward(&self, player_id: i64, consecutive: i32, total: i32, reward_gold: i64) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("UPDATE players SET last_daily_reward_at = CURRENT_TIMESTAMP, consecutive_days = ?, total_days = ?, gold = gold + ?, max_gold_held = MAX(max_gold_held, gold + ?) WHERE id = ?")
            .bind(consecutive)
            .bind(total)
            .bind(reward_gold)
            .bind(reward_gold)
            .bind(player_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_player_catches(&self, player_id: i64) -> Result<Vec<Fish>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, fish_name, rarity, size, weight, state, description, stream_title, caught_at, is_junk, caught_by FROM catches WHERE player_id = ? ORDER BY caught_at DESC")
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
            fish.caught_by = row.get("caught_by");
            fish
        }).collect();

        Ok(catches)
    }

    pub async fn get_catch_by_id(&self, id: i64) -> Result<Option<(Fish, String)>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT c.id, c.fish_name, c.rarity, c.size, c.weight, c.state, c.description, \
                    c.stream_title, c.caught_at, c.is_junk, c.caught_by, p.username as owner_name \
             FROM catches c \
             JOIN players p ON c.player_id = p.id \
             WHERE c.id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
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
            fish.caught_by = row.get("caught_by");
            
            let owner_name: String = row.get("owner_name");
            Ok(Some((fish, owner_name)))
        } else {
            Ok(None)
        }
    }

    pub async fn count_fish_owned_by_player(&self, player_id: i64, fish_name: &str) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM catches WHERE player_id = ? AND LOWER(fish_name) = LOWER(?)")
            .bind(player_id)
            .bind(fish_name)
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    pub async fn update_player_gold(&self, player_id: i64, amount: i64) -> Result<i64, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("UPDATE players SET gold = MAX(0, gold + ?), max_gold_held = MAX(max_gold_held, MAX(0, gold + ?)) WHERE id = ?")
            .bind(amount)
            .bind(amount)
            .bind(player_id)
            .execute(&mut *tx)
            .await?;
        let new_gold: i64 = sqlx::query_scalar("SELECT gold FROM players WHERE id = ?")
            .bind(player_id)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(new_gold)
    }

    pub async fn record_coinflip_result(&self, player_id: i64, wager: i64, win: bool) -> Result<Player, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        if win {
            sqlx::query("UPDATE players SET \
                          gold = gold + ?, \
                          max_gold_held = MAX(max_gold_held, gold + ?), \
                          coinflip_wins = COALESCE(coinflip_wins, 0) + 1, \
                          coinflip_gold_won_total = COALESCE(coinflip_gold_won_total, 0) + ?, \
                          coinflip_biggest_win = MAX(COALESCE(coinflip_biggest_win, 0), ?), \
                          coinflip_current_win_streak = COALESCE(coinflip_current_win_streak, 0) + 1, \
                          coinflip_max_win_streak = MAX(COALESCE(coinflip_max_win_streak, 0), COALESCE(coinflip_current_win_streak, 0) + 1), \
                          coinflip_current_loss_streak = 0 \
                          WHERE id = ?")
                .bind(wager)
                .bind(wager)
                .bind(wager)
                .bind(wager)
                .bind(player_id)
                .execute(&mut *tx)
                .await?;
        } else {
            sqlx::query("UPDATE players SET \
                          gold = MAX(0, gold - ?), \
                          coinflip_losses = COALESCE(coinflip_losses, 0) + 1, \
                          coinflip_gold_lost_total = COALESCE(coinflip_gold_lost_total, 0) + ?, \
                          coinflip_biggest_loss = MAX(COALESCE(coinflip_biggest_loss, 0), ?), \
                          coinflip_current_loss_streak = COALESCE(coinflip_current_loss_streak, 0) + 1, \
                          coinflip_max_loss_streak = MAX(COALESCE(coinflip_max_loss_streak, 0), COALESCE(coinflip_current_loss_streak, 0) + 1), \
                          coinflip_current_win_streak = 0 \
                          WHERE id = ?")
                .bind(wager)
                .bind(wager)
                .bind(wager)
                .bind(player_id)
                .execute(&mut *tx)
                .await?;
        }
        let row = sqlx::query("SELECT p.*, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND is_junk = 1) as junk_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Banana%') as banana_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Gemme%') as gem_count \
            FROM players p WHERE p.id = ?")
            .bind(player_id)
            .fetch_one(&mut *tx)
            .await?;

        let player = Player {
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
            last_daily_reward_at: row.get::<Option<DateTime<Utc>>, _>("last_daily_reward_at"),
            consecutive_days: row.get("consecutive_days"),
            total_days: row.get("total_days"),
            coinflip_wins: row.get("coinflip_wins"),
            coinflip_losses: row.get("coinflip_losses"),
            coinflip_biggest_win: row.get("coinflip_biggest_win"),
            coinflip_biggest_loss: row.get("coinflip_biggest_loss"),
            coinflip_gold_won_total: row.get("coinflip_gold_won_total"),
            coinflip_gold_lost_total: row.get("coinflip_gold_lost_total"),
            coinflip_current_win_streak: row.get("coinflip_current_win_streak"),
            coinflip_current_loss_streak: row.get("coinflip_current_loss_streak"),
            coinflip_max_win_streak: row.get("coinflip_max_win_streak"),
            coinflip_max_loss_streak: row.get("coinflip_max_loss_streak"),
            gold_given_total: row.get("gold_given_total"),
            max_gold_held: row.get("max_gold_held"),
            language: row.get("language"),
        };

        tx.commit().await?;
        Ok(player)
    }

    pub async fn get_gambling_leaderboard(&self) -> Result<Vec<Player>, sqlx::Error> {
        let rows = sqlx::query("SELECT p.*, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND is_junk = 1) as junk_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Banana%') as banana_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Gemme%') as gem_count \
            FROM players p \
            WHERE (COALESCE(p.coinflip_wins, 0) + COALESCE(p.coinflip_losses, 0)) > 0 \
            ORDER BY (COALESCE(p.coinflip_gold_won_total, 0) - COALESCE(p.coinflip_gold_lost_total, 0)) DESC \
            LIMIT 5")
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
            last_daily_reward_at: row.get::<Option<DateTime<Utc>>, _>("last_daily_reward_at"),
            consecutive_days: row.get("consecutive_days"),
            total_days: row.get("total_days"),
            coinflip_wins: row.get("coinflip_wins"),
            coinflip_losses: row.get("coinflip_losses"),
            coinflip_biggest_win: row.get("coinflip_biggest_win"),
            coinflip_biggest_loss: row.get("coinflip_biggest_loss"),
            coinflip_gold_won_total: row.get("coinflip_gold_won_total"),
            coinflip_gold_lost_total: row.get("coinflip_gold_lost_total"),
            coinflip_current_win_streak: row.get("coinflip_current_win_streak"),
            coinflip_current_loss_streak: row.get("coinflip_current_loss_streak"),
            coinflip_max_win_streak: row.get("coinflip_max_win_streak"),
            coinflip_max_loss_streak: row.get("coinflip_max_loss_streak"),
            gold_given_total: row.get("gold_given_total"),
            max_gold_held: row.get("max_gold_held"),
            language: row.get("language"),
        }).collect();

        Ok(players)
    }


    pub async fn add_cooldown_penalty(&self, player_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE players SET last_fishing_time = DATETIME(COALESCE(last_fishing_time, CURRENT_TIMESTAMP), '+20 seconds'), gold = MAX(0, gold - 20) WHERE id = ?")
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn apply_extra_fail_penalty(&self, player_id: i64, gold_penalty: i64, cooldown_penalty_secs: i64) -> Result<(), sqlx::Error> {
        let query = format!(
            "UPDATE players SET last_fishing_time = DATETIME(COALESCE(last_fishing_time, CURRENT_TIMESTAMP), '+{} seconds'), gold = MAX(0, gold - {}) WHERE id = ?",
            cooldown_penalty_secs, gold_penalty
        );
        sqlx::query(&query)
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn record_stream_live_date(&self, date: chrono::NaiveDate) -> Result<(), sqlx::Error> {
        let date_str = date.to_string();
        sqlx::query("INSERT OR IGNORE INTO stream_live_dates (live_date) VALUES (?)")
            .bind(&date_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn count_stream_days_between(&self, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<i64, sqlx::Error> {
        let start_str = start.to_string();
        let end_str = end.to_string();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM stream_live_dates WHERE live_date > ? AND live_date < ?")
            .bind(&start_str)
            .bind(&end_str)
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    pub async fn reset_player(&self, username: &str) -> Result<(), sqlx::Error> {
        let username_lower = username.to_lowercase();
        let mut tx = self.pool.begin().await?;

        // Supprimer les poissons capturés
        sqlx::query("DELETE FROM catches WHERE player_id IN (SELECT id FROM players WHERE username = ?)")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        // Réinitialiser les stats du joueur (y compris les pièces d'or)
        sqlx::query("UPDATE players SET total_attempts = 0, successful_attempts = 0, failed_attempts = 0, last_fishing_time = NULL, level = 1, xp = 0, vip_until = NULL, gold = 0 WHERE username = ?")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        // Mettre à jour le statut du Roi des Bananes
        Self::check_and_update_banana_king_status(&mut tx, 0).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn record_museum_discovery(&self, tx: &mut sqlx::SqliteConnection, player_id: i64, fish: &Fish) -> Result<(), sqlx::Error> {
        let username: String = sqlx::query_scalar("SELECT username FROM players WHERE id = ?")
            .bind(player_id)
            .fetch_one(&mut *tx)
            .await?;

        let existing: Option<(i64, f64, f64, String, Option<String>, i32)> = sqlx::query_as(
            "SELECT id, max_size, max_weight, best_state, description, total_caught FROM museum_discoveries WHERE player_id = ? AND fish_name = ?"
        )
        .bind(player_id)
        .bind(&fish.name)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some((id, max_size, max_weight, best_state, description, total_caught)) = existing {
            let new_max_size = if fish.size > max_size { fish.size } else { max_size };
            let new_max_weight = if fish.weight > max_weight { fish.weight } else { max_weight };
            
            fn state_rank(s: &str) -> i32 {
                match s.to_lowercase().as_str() {
                    "badly damaged" => 1,
                    "damaged" => 2,
                    "worn" => 3,
                    "good" => 4,
                    "pristine" => 5,
                    _ => 0,
                }
            }
            
            let is_better = state_rank(&fish.state) > state_rank(&best_state);
            let new_best_state = if is_better {
                fish.state.clone()
            } else {
                best_state.clone()
            };
            
            let new_desc = if is_better {
                fish.description.clone()
            } else {
                description.unwrap_or_default()
            };

            sqlx::query("UPDATE museum_discoveries SET max_size = ?, max_weight = ?, best_state = ?, description = ?, total_caught = ? WHERE id = ?")
                .bind(new_max_size)
                .bind(new_max_weight)
                .bind(new_best_state)
                .bind(new_desc)
                .bind(total_caught + 1)
                .bind(id)
                .execute(&mut *tx)
                .await?;
        } else {
            sqlx::query("INSERT INTO museum_discoveries (player_id, username, fish_name, rarity, max_size, max_weight, best_state, description, total_caught) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1)")
                .bind(player_id)
                .bind(&username)
                .bind(&fish.name)
                .bind(serde_json::to_string(&fish.rarity).unwrap_or_default())
                .bind(fish.size)
                .bind(fish.weight)
                .bind(&fish.state)
                .bind(&fish.description)
                .execute(&mut *tx)
                .await?;
        }
        Ok(())
    }

    pub async fn get_player_museum(&self, player_id: i64) -> Result<Vec<MuseumDiscovery>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, player_id, username, fish_name, rarity, max_size, max_weight, best_state, description, total_caught, strftime('%Y-%m-%dT%H:%M:%SZ', unlocked_at) as unlocked_at FROM museum_discoveries WHERE player_id = ? ORDER BY unlocked_at DESC")
            .bind(player_id)
            .fetch_all(&self.pool)
            .await?;

        let discoveries = rows.into_iter().map(|row| {
            MuseumDiscovery {
                id: Some(row.get("id")),
                player_id: row.get("player_id"),
                username: row.get("username"),
                fish_name: row.get("fish_name"),
                rarity: row.get("rarity"),
                max_size: row.get("max_size"),
                max_weight: row.get("max_weight"),
                best_state: row.get("best_state"),
                description: row.get("description"),
                total_caught: row.get("total_caught"),
                unlocked_at: row.get("unlocked_at"),
            }
        }).collect();

        Ok(discoveries)
    }

    pub async fn is_museum_empty(&self) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM museum_discoveries")
            .fetch_one(&self.pool)
            .await?;
        Ok(count == 0)
    }

    pub async fn backfill_museum(&self) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        let rows = sqlx::query(
            "SELECT c.player_id, c.fish_name, c.rarity, c.size, c.weight, c.state, c.description, c.stream_title, c.caught_by 
             FROM catches c 
             WHERE c.is_junk = 0"
        )
        .fetch_all(&mut *tx)
        .await?;

        for row in rows {
            let player_id: i64 = row.get("player_id");
            let fish_name: String = row.get("fish_name");
            let rarity_str: String = row.get("rarity");
            let rarity: crate::config::Rarity = serde_json::from_str(&rarity_str).unwrap_or(crate::config::Rarity::Common);
            
            let fish = Fish {
                id: None,
                name: fish_name,
                rarity,
                size: row.get("size"),
                weight: row.get("weight"),
                state: row.get("state"),
                description: row.get("description"),
                stream_title: row.get("stream_title"),
                caught_at: None,
                is_junk: false,
                caught_by: row.get("caught_by"),
            };
            
            self.record_museum_discovery(&mut *tx, player_id, &fish).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn save_catch_only(&self, player_id: i64, fish: Fish, caught_by: Option<&str>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        sqlx::query("INSERT INTO catches (player_id, fish_name, rarity, size, weight, state, description, stream_title, is_junk, caught_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(player_id)
            .bind(&fish.name)
            .bind(serde_json::to_string(&fish.rarity).unwrap_or_default())
            .bind(fish.size)
            .bind(fish.weight)
            .bind(&fish.state)
            .bind(&fish.description)
            .bind(&fish.stream_title)
            .bind(fish.is_junk)
            .bind(caught_by)
            .execute(&mut *tx)
            .await?;

        self.record_museum_discovery(&mut *tx, player_id, &fish).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn save_attempt(&self, player: &Player, success: bool, fish: Option<Fish>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("UPDATE players SET total_attempts = total_attempts + 1, successful_attempts = successful_attempts + ?, failed_attempts = failed_attempts + ?, last_fishing_time = ?, level = ?, xp = ?, vip_until = ?, gold = MAX(0, gold - 10) WHERE id = ?")
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
            
            // Bananas are now always inserted as new rows (no unique skip)
            sqlx::query("INSERT INTO catches (player_id, fish_name, rarity, size, weight, state, description, stream_title, is_junk, caught_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(player.id)
                .bind(&f.name)
                .bind(serde_json::to_string(&f.rarity).unwrap_or_default())
                .bind(f.size)
                .bind(f.weight)
                .bind(&f.state)
                .bind(&f.description)
                .bind(&f.stream_title)
                .bind(f.is_junk)
                .bind(&player.username)
                .execute(&mut *tx)
                .await?;

            self.record_museum_discovery(&mut *tx, player.id.unwrap(), &f).await?;

            if is_banana {
                Self::check_and_update_banana_king_status(&mut tx, player.id.unwrap()).await?;
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
        // Find the owner of the latest catch of banana_name
        let row: Option<sqlx::sqlite::SqliteRow> = sqlx::query(
            "SELECT p.id, p.username FROM catches c JOIN players p ON c.player_id = p.id WHERE c.fish_name = ? ORDER BY c.id DESC LIMIT 1"
        )
        .bind(banana_name)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            use sqlx::Row;
            let old_player_id: i64 = r.get("id");
            let old_username: String = r.get("username");
            
            if old_player_id != current_player_id {
                Ok(Some(old_username))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn has_caught_banana(&self, player_id: i64, banana_name: &str) -> Result<bool, sqlx::Error> {
        let latest_owner: Option<i64> = sqlx::query_scalar(
            "SELECT player_id FROM catches WHERE fish_name = ? ORDER BY id DESC LIMIT 1"
        )
        .bind(banana_name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(latest_owner == Some(player_id))
    }

    pub async fn is_active_king(&self, player_id: i64) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM banana_kings_history WHERE player_id = ? AND dethroned_at IS NULL"
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(count > 0)
    }

    async fn check_and_update_banana_king_status(
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        _player_id: i64,
    ) -> Result<(), sqlx::Error> {
        // Find the owner of the latest catch of Pristine Banana 1
        let latest_b1_owner: Option<i64> = sqlx::query_scalar(
            "SELECT player_id FROM catches WHERE fish_name = 'Pristine Banana 1' ORDER BY id DESC LIMIT 1"
        )
        .fetch_optional(&mut **tx)
        .await?;

        // Find the owner of the latest catch of Pristine Banana 2
        let latest_b2_owner: Option<i64> = sqlx::query_scalar(
            "SELECT player_id FROM catches WHERE fish_name = 'Pristine Banana 2' ORDER BY id DESC LIMIT 1"
        )
        .fetch_optional(&mut **tx)
        .await?;

        match (latest_b1_owner, latest_b2_owner) {
            (Some(owner1), Some(owner2)) if owner1 == owner2 => {
                // One single player owns the latest catch of both bananas!
                let username: String = sqlx::query_scalar("SELECT username FROM players WHERE id = ?")
                    .bind(owner1)
                    .fetch_one(&mut **tx)
                    .await?;

                let is_already_king: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM banana_kings_history WHERE player_id = ? AND dethroned_at IS NULL"
                )
                .bind(owner1)
                .fetch_one(&mut **tx)
                .await?;

                if is_already_king == 0 {
                    // Dethrone any existing active King
                    sqlx::query("UPDATE banana_kings_history SET dethroned_at = CURRENT_TIMESTAMP WHERE dethroned_at IS NULL")
                        .execute(&mut **tx)
                        .await?;

                    // Insert new King record
                    sqlx::query("INSERT INTO banana_kings_history (player_id, username, dethroned_at) VALUES (?, ?, NULL)")
                        .bind(owner1)
                        .bind(&username)
                        .execute(&mut **tx)
                        .await?;
                }
            }
            _ => {
                // No player owns both latest bananas.
                // Dethrone any active King if one exists.
                sqlx::query("UPDATE banana_kings_history SET dethroned_at = CURRENT_TIMESTAMP WHERE dethroned_at IS NULL")
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn execute_simulation(
        &self,
        player_id: i64,
        username: &str,
        count: u32,
        use_english: bool,
    ) -> Result<(u32, u32, u32, i32), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Load current player state
        let r = sqlx::query("SELECT p.*, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND is_junk = 1) as junk_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Banana%') as banana_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Carte Postale%') as postcard_count, \
            (SELECT COUNT(*) FROM catches WHERE player_id = p.id AND fish_name LIKE '%Gemme%') as gem_count \
            FROM players p WHERE p.id = ?")
            .bind(player_id)
            .fetch_one(&mut *tx)
            .await?;

        let mut player = Player {
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
            last_daily_reward_at: r.get::<Option<DateTime<Utc>>, _>("last_daily_reward_at"),
            consecutive_days: r.get("consecutive_days"),
            total_days: r.get("total_days"),
            coinflip_wins: r.get("coinflip_wins"),
            coinflip_losses: r.get("coinflip_losses"),
            coinflip_biggest_win: r.get("coinflip_biggest_win"),
            coinflip_biggest_loss: r.get("coinflip_biggest_loss"),
            coinflip_gold_won_total: r.get("coinflip_gold_won_total"),
            coinflip_gold_lost_total: r.get("coinflip_gold_lost_total"),
            coinflip_current_win_streak: r.get("coinflip_current_win_streak"),
            coinflip_current_loss_streak: r.get("coinflip_current_loss_streak"),
            coinflip_max_win_streak: r.get("coinflip_max_win_streak"),
            coinflip_max_loss_streak: r.get("coinflip_max_loss_streak"),
            gold_given_total: r.get("gold_given_total"),
            max_gold_held: r.get("max_gold_held"),
            language: r.get("language"),
        };

        let mut success_count = 0;
        let mut junk_count = 0;
        let mut fail_count = 0;

        for _ in 0..count {
            let level_factor = (player.level as f64 - 1.0) / 199.0;
            let success_rate = 0.35 + (level_factor * 0.20);
            let junk_rate = 0.05;
            let roll = rand::random::<f64>();

            let (success, fish) = if roll < success_rate {
                success_count += 1;
                player.add_xp(25);
                (true, crate::game::generate_fish(use_english))
            } else if roll < success_rate + junk_rate {
                junk_count += 1;
                player.add_xp(5);
                (true, crate::game::generate_junk(use_english))
            } else {
                fail_count += 1;
                player.add_xp(5);
                (false, None)
            };

            // Update player in DB
            sqlx::query("UPDATE players SET total_attempts = total_attempts + 1, successful_attempts = successful_attempts + ?, failed_attempts = failed_attempts + ?, last_fishing_time = ?, level = ?, xp = ?, vip_until = ?, gold = MAX(0, gold - 10) WHERE id = ?")
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
                
                sqlx::query("INSERT INTO catches (player_id, fish_name, rarity, size, weight, state, description, stream_title, is_junk, caught_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                    .bind(player.id)
                    .bind(&f.name)
                    .bind(serde_json::to_string(&f.rarity).unwrap_or_default())
                    .bind(f.size)
                    .bind(f.weight)
                    .bind(&f.state)
                    .bind(&f.description)
                    .bind(&f.stream_title)
                    .bind(f.is_junk)
                    .bind(username)
                    .execute(&mut *tx)
                    .await?;

                self.record_museum_discovery(&mut *tx, player.id.unwrap(), &f).await?;
                
                if is_banana {
                    Self::check_and_update_banana_king_status(&mut tx, player.id.unwrap()).await?;
                }
            }
        }

        tx.commit().await?;
        Ok((success_count, junk_count, fail_count, player.level))
    }

    pub async fn execute_gold_sale(
        &self,
        seller_id: i64,
        catch_ids: &[i64],
        gold_earned: i64,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        for &id in catch_ids {
            let rows_affected = sqlx::query("DELETE FROM catches WHERE id = ? AND player_id = ?")
                .bind(id)
                .bind(seller_id)
                .execute(&mut *tx)
                .await?
                .rows_affected();
            if rows_affected == 0 {
                return Err(sqlx::Error::RowNotFound);
            }
        }

        sqlx::query("UPDATE players SET gold = gold + ?, max_gold_held = MAX(max_gold_held, gold + ?) WHERE id = ?")
            .bind(gold_earned)
            .bind(gold_earned)
            .bind(seller_id)
            .execute(&mut *tx)
            .await?;

        Self::check_and_update_banana_king_status(&mut tx, seller_id).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn execute_gold_transfer(
        &self,
        giver_id: i64,
        receiver_id: i64,
        amount: i64,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Déduction avec vérification atomique de solde suffisant
        let rows_affected = sqlx::query("UPDATE players SET gold = gold - ?, gold_given_total = gold_given_total + ? WHERE id = ? AND gold >= ?")
            .bind(amount)
            .bind(amount)
            .bind(giver_id)
            .bind(amount)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        // 2. Crédit au receveur
        sqlx::query("UPDATE players SET gold = gold + ?, max_gold_held = MAX(max_gold_held, gold + ?) WHERE id = ?")
            .bind(amount)
            .bind(amount)
            .bind(receiver_id)
            .execute(&mut *tx)
            .await?;

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

        // 2. Transfer gold (atomic deduction checking if buyer has enough gold)
        let gold_rows_affected = sqlx::query("UPDATE players SET gold = gold - ? WHERE id = ? AND gold >= ?")
            .bind(price)
            .bind(buyer_id)
            .bind(price)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        if gold_rows_affected == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query("UPDATE players SET gold = gold + ?, max_gold_held = MAX(max_gold_held, gold + ?) WHERE id = ?")
            .bind(price)
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
        sqlx::query("DROP INDEX IF EXISTS idx_museum_player_id").execute(&mut *tx).await?;
        sqlx::query("DROP TABLE IF EXISTS catches").execute(&mut *tx).await?;
        sqlx::query("DROP TABLE IF EXISTS banana_kings_history").execute(&mut *tx).await?;
        sqlx::query("DROP TABLE IF EXISTS museum_discoveries").execute(&mut *tx).await?;
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
                gold INTEGER DEFAULT 0,
                last_daily_reward_at DATETIME,
                consecutive_days INTEGER DEFAULT 0,
                total_days INTEGER DEFAULT 0,
                coinflip_wins INTEGER DEFAULT 0,
                coinflip_losses INTEGER DEFAULT 0,
                coinflip_biggest_win INTEGER DEFAULT 0,
                coinflip_biggest_loss INTEGER DEFAULT 0,
                coinflip_gold_won_total INTEGER DEFAULT 0,
                coinflip_gold_lost_total INTEGER DEFAULT 0,
                coinflip_current_win_streak INTEGER DEFAULT 0,
                coinflip_current_loss_streak INTEGER DEFAULT 0,
                coinflip_max_win_streak INTEGER DEFAULT 0,
                coinflip_max_loss_streak INTEGER DEFAULT 0,
                gold_given_total INTEGER DEFAULT 0,
                max_gold_held INTEGER DEFAULT 0,
                language TEXT DEFAULT NULL
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
                is_junk BOOLEAN DEFAULT 0,
                caught_by TEXT
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

        // Recréer la table museum_discoveries
        sqlx::query(
            "CREATE TABLE museum_discoveries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
                username TEXT NOT NULL,
                fish_name TEXT NOT NULL,
                rarity TEXT NOT NULL,
                max_size REAL NOT NULL,
                max_weight REAL DEFAULT 0,
                best_state TEXT NOT NULL,
                description TEXT,
                total_caught INTEGER DEFAULT 1,
                unlocked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(player_id, fish_name)
            )"
        ).execute(&mut *tx).await?;

        // Recréer les index
        sqlx::query("CREATE INDEX idx_catches_player_id ON catches(player_id);").execute(&mut *tx).await?;
        sqlx::query("CREATE INDEX idx_players_username ON players(username);").execute(&mut *tx).await?;
        sqlx::query("CREATE INDEX idx_museum_player_id ON museum_discoveries(player_id);").execute(&mut *tx).await?;

        tx.commit().await?;

        // Supprimer le fichier de sauvegarde s'il existe pour éviter une auto-restauration au redémarrage
        let _ = std::fs::remove_file("data/players_backup.json");

        Ok(())
    }

    pub async fn get_player_trophies(&self, _player_id: i64) -> Result<Vec<PlayerTrophy>, sqlx::Error> {
        Ok(Vec::new())
    }

    pub async fn reset_player_all(&self, username: &str) -> Result<(), sqlx::Error> {
        let username_lower = username.to_lowercase();
        let mut tx = self.pool.begin().await?;

        // 1. Supprimer les poissons capturés
        sqlx::query("DELETE FROM catches WHERE player_id IN (SELECT id FROM players WHERE username = ?)")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        // 2. Supprimer les découvertes du musée
        sqlx::query("DELETE FROM museum_discoveries WHERE player_id IN (SELECT id FROM players WHERE username = ?)")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        // 3. Réinitialiser les stats du joueur (y compris les pièces d'or)
        sqlx::query("UPDATE players SET total_attempts = 0, successful_attempts = 0, failed_attempts = 0, last_fishing_time = NULL, level = 1, xp = 0, vip_until = NULL, gold = 0 WHERE username = ?")
            .bind(&username_lower)
            .execute(&mut *tx)
            .await?;

        // Mettre à jour le statut du Roi des Bananes
        Self::check_and_update_banana_king_status(&mut tx, 0).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn update_player_language(&self, player_id: i64, language: Option<String>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE players SET language = ? WHERE id = ?")
            .bind(language)
            .bind(player_id)
            .execute(&self.pool)
            .await?;
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
                gold INTEGER DEFAULT 0,
                last_daily_reward_at DATETIME,
                consecutive_days INTEGER DEFAULT 0,
                total_days INTEGER DEFAULT 0,
                coinflip_wins INTEGER DEFAULT 0,
                coinflip_losses INTEGER DEFAULT 0,
                coinflip_biggest_win INTEGER DEFAULT 0,
                coinflip_biggest_loss INTEGER DEFAULT 0,
                coinflip_gold_won_total INTEGER DEFAULT 0,
                coinflip_gold_lost_total INTEGER DEFAULT 0,
                coinflip_current_win_streak INTEGER DEFAULT 0,
                coinflip_current_loss_streak INTEGER DEFAULT 0,
                coinflip_max_win_streak INTEGER DEFAULT 0,
                coinflip_max_loss_streak INTEGER DEFAULT 0,
                gold_given_total INTEGER DEFAULT 0,
                max_gold_held INTEGER DEFAULT 0,
                language TEXT DEFAULT NULL
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
                is_junk BOOLEAN DEFAULT 0,
                caught_by TEXT
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

        sqlx::query(
            "CREATE TABLE museum_discoveries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER REFERENCES players(id),
                username TEXT NOT NULL,
                fish_name TEXT NOT NULL,
                rarity TEXT NOT NULL,
                max_size REAL NOT NULL,
                max_weight REAL DEFAULT 0,
                best_state TEXT NOT NULL,
                description TEXT,
                total_caught INTEGER DEFAULT 1,
                unlocked_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE stream_live_dates (
                live_date TEXT PRIMARY KEY
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

        // Save B's Banana 1 catch!
        let b1_b = Fish::new("Pristine Banana 1".to_string(), Rarity::Divin, 21.0, 155.0, "pristine".to_string(), "Banana 1".to_string());
        repo.save_attempt(&p_b, true, Some(b1_b)).await.unwrap();

        // Player A should now be dethroned because Player B has the latest Banana 1 catch!
        let hist_2 = repo.get_banana_kings_history().await.unwrap();
        assert_eq!(hist_2.len(), 1);
        assert_eq!(hist_2[0].username, "player_a");
        assert!(hist_2[0].dethroned_at.is_some());

        // Verify Player A still has their original catches in database, but Player B has their new catch
        let catches_a_after = repo.get_player_catches(p_a.id.unwrap()).await.unwrap();
        assert!(catches_a_after.iter().any(|c| c.name == "Pristine Banana 1"));
        assert!(catches_a_after.iter().any(|c| c.name == "Pristine Banana 2"));

        let catches_b = repo.get_player_catches(p_b.id.unwrap()).await.unwrap();
        assert!(catches_b.iter().any(|c| c.name == "Pristine Banana 1"));

        // 4. Player B catches Pristine Banana 2 (Theft!)
        let stolen_from_2 = repo.check_and_execute_banana_theft(p_b.id.unwrap(), "Pristine Banana 2").await.unwrap();
        assert_eq!(stolen_from_2, Some("player_a".to_string()));

        // Player B saves their Pristine Banana 2 catch (Crowning Player B!)
        let b2_b = Fish::new("Pristine Banana 2".to_string(), Rarity::Divin, 22.0, 160.0, "pristine".to_string(), "Banana 2".to_string());
        repo.save_attempt(&p_b, true, Some(b2_b)).await.unwrap();

        // Verify Player B has both latest captures!
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

    #[tokio::test]
    async fn test_reset_player_gold() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        // Restore Player A with 100 gold
        let _p_id = repo.restore_player(&crate::db::PlayerBackup {
            username: "player_a".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(100),
        }).await.unwrap();

        // 1. Soft Reset Player A
        repo.reset_player("player_a").await.unwrap();
        let player_a = repo.get_player("player_a").await.unwrap().unwrap();
        assert_eq!(player_a.gold, 0);

        // 2. Restore Player B with 200 gold
        let _p_id_b = repo.restore_player(&crate::db::PlayerBackup {
            username: "player_b".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(200),
        }).await.unwrap();

        // Hard Reset Player B
        repo.reset_player_all("player_b").await.unwrap();
        let player_b = repo.get_player("player_b").await.unwrap().unwrap();
        assert_eq!(player_b.gold, 0);
    }

    #[tokio::test]
    async fn test_update_player_gold() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        // Restore Player with 100 gold
        let p_id = repo.restore_player(&crate::db::PlayerBackup {
            username: "player_gold_test".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(100),
        }).await.unwrap();

        // 1. Add 50 gold
        let new_gold = repo.update_player_gold(p_id, 50).await.unwrap();
        assert_eq!(new_gold, 150);
        let player = repo.get_player("player_gold_test").await.unwrap().unwrap();
        assert_eq!(player.gold, 150);

        // 2. Subtract 80 gold
        let new_gold = repo.update_player_gold(p_id, -80).await.unwrap();
        assert_eq!(new_gold, 70);
        let player = repo.get_player("player_gold_test").await.unwrap().unwrap();
        assert_eq!(player.gold, 70);

        // 3. Try to subtract more than current (e.g. -100 gold), check it clamps at 0
        let new_gold = repo.update_player_gold(p_id, -100).await.unwrap();
        assert_eq!(new_gold, 0);
        let player = repo.get_player("player_gold_test").await.unwrap().unwrap();
        assert_eq!(player.gold, 0);
    }

    #[tokio::test]
    async fn test_add_cooldown_penalty() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        let p_id = repo.restore_player(&crate::db::PlayerBackup {
            username: "player_penalty_test".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(100),
        }).await.unwrap();

        let initial_player = repo.get_player("player_penalty_test").await.unwrap().unwrap();
        assert_eq!(initial_player.gold, 100);

        // Apply penalty (adds 20s and deducts 20 gold)
        repo.add_cooldown_penalty(p_id).await.unwrap();

        let updated_player = repo.get_player("player_penalty_test").await.unwrap().unwrap();
        assert_eq!(updated_player.gold, 80);
        assert!(updated_player.last_fishing_time.is_some());

        // Apply another penalty when player has less than 20 gold
        let _ = repo.update_player_gold(p_id, -70).await.unwrap(); // gold set to 10
        repo.add_cooldown_penalty(p_id).await.unwrap();
        let final_player = repo.get_player("player_penalty_test").await.unwrap().unwrap();
        assert_eq!(final_player.gold, 0); // clamped at 0
    }

    #[tokio::test]
    async fn test_execute_gold_transfer() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        let p_id_a = repo.restore_player(&crate::db::PlayerBackup {
            username: "player_a".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(100),
        }).await.unwrap();

        let p_id_b = repo.restore_player(&crate::db::PlayerBackup {
            username: "player_b".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(50),
        }).await.unwrap();

        // 1. Transfert valide de 40 gold de A vers B
        repo.execute_gold_transfer(p_id_a, p_id_b, 40).await.unwrap();
        let player_a = repo.get_player("player_a").await.unwrap().unwrap();
        let player_b = repo.get_player("player_b").await.unwrap().unwrap();
        assert_eq!(player_a.gold, 60);
        assert_eq!(player_b.gold, 90);

        // 2. Transfert invalide de 70 gold de A vers B (solde insuffisant, car A n'a plus que 60 gold)
        let res = repo.execute_gold_transfer(p_id_a, p_id_b, 70).await;
        assert!(res.is_err());
        let player_a_after = repo.get_player("player_a").await.unwrap().unwrap();
        let player_b_after = repo.get_player("player_b").await.unwrap().unwrap();
        assert_eq!(player_a_after.gold, 60); // Inchangé
        assert_eq!(player_b_after.gold, 90); // Inchangé
    }

    #[tokio::test]
    async fn test_record_coinflip_result() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        // Restore Player A with 100 gold
        let p_id_a = repo.restore_player(&crate::db::PlayerBackup {
            username: "gambler_a".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(100),
        }).await.unwrap();

        // Restore Player B with 100 gold
        let p_id_b = repo.restore_player(&crate::db::PlayerBackup {
            username: "gambler_b".to_string(),
            total_attempts: 10,
            successful_attempts: 5,
            failed_attempts: 5,
            level: 3,
            xp: 250,
            vip_until: None,
            gold: Some(100),
        }).await.unwrap();

        // 1. Player A wins a coinflip of 50 gold
        let p_res = repo.record_coinflip_result(p_id_a, 50, true).await.unwrap();
        assert_eq!(p_res.gold, 150);
        assert_eq!(p_res.coinflip_current_win_streak, 1);
        assert_eq!(p_res.coinflip_max_win_streak, 1);
        assert_eq!(p_res.coinflip_current_loss_streak, 0);

        let p_a = repo.get_player("gambler_a").await.unwrap().unwrap();
        assert_eq!(p_a.coinflip_wins, 1);
        assert_eq!(p_a.coinflip_losses, 0);
        assert_eq!(p_a.coinflip_gold_won_total, 50);
        assert_eq!(p_a.coinflip_gold_lost_total, 0);
        assert_eq!(p_a.coinflip_biggest_win, 50);
        assert_eq!(p_a.coinflip_current_win_streak, 1);
        assert_eq!(p_a.coinflip_max_win_streak, 1);

        // 2. Player A wins another coinflip of 100 gold
        let p_res2 = repo.record_coinflip_result(p_id_a, 100, true).await.unwrap();
        assert_eq!(p_res2.coinflip_current_win_streak, 2);
        assert_eq!(p_res2.coinflip_max_win_streak, 2);

        let p_a = repo.get_player("gambler_a").await.unwrap().unwrap();
        assert_eq!(p_a.coinflip_wins, 2);
        assert_eq!(p_a.coinflip_gold_won_total, 150);
        assert_eq!(p_a.coinflip_biggest_win, 100);
        assert_eq!(p_a.coinflip_current_win_streak, 2);
        assert_eq!(p_a.coinflip_max_win_streak, 2);

        // 3. Player B loses a coinflip of 40 gold
        let p_res_b = repo.record_coinflip_result(p_id_b, 40, false).await.unwrap();
        assert_eq!(p_res_b.gold, 60);
        assert_eq!(p_res_b.coinflip_current_loss_streak, 1);
        assert_eq!(p_res_b.coinflip_max_loss_streak, 1);
        assert_eq!(p_res_b.coinflip_current_win_streak, 0);

        let p_b = repo.get_player("gambler_b").await.unwrap().unwrap();
        assert_eq!(p_b.coinflip_wins, 0);
        assert_eq!(p_b.coinflip_losses, 1);
        assert_eq!(p_b.coinflip_gold_won_total, 0);
        assert_eq!(p_b.coinflip_gold_lost_total, 40);
        assert_eq!(p_b.coinflip_biggest_loss, 40);
        assert_eq!(p_b.coinflip_current_loss_streak, 1);
        assert_eq!(p_b.coinflip_max_loss_streak, 1);
        assert_eq!(p_b.coinflip_current_win_streak, 0);

        // 4. Verify Leaderboard
        let lb = repo.get_gambling_leaderboard().await.unwrap();
        assert_eq!(lb.len(), 2);
        // Player A has net +150, Player B has net -40. A should be #1.
        assert_eq!(lb[0].username, "gambler_a");
        assert_eq!(lb[1].username, "gambler_b");
    }

    #[tokio::test]
    async fn test_stream_live_dates() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        let d1 = chrono::NaiveDate::from_ymd_opt(2026, 5, 25).unwrap(); // Monday
        let d2 = chrono::NaiveDate::from_ymd_opt(2026, 5, 26).unwrap(); // Tuesday
        let d3 = chrono::NaiveDate::from_ymd_opt(2026, 5, 27).unwrap(); // Wednesday

        // Initially no stream days
        let count = repo.count_stream_days_between(d1, d3).await.unwrap();
        assert_eq!(count, 0);

        // Record a stream day on Tuesday
        repo.record_stream_live_date(d2).await.unwrap();

        // Should find 1 stream day between Monday and Wednesday
        let count = repo.count_stream_days_between(d1, d3).await.unwrap();
        assert_eq!(count, 1);

        // Monday to Tuesday (exclusive) should have 0 stream days
        let count_d1_d2 = repo.count_stream_days_between(d1, d2).await.unwrap();
        assert_eq!(count_d1_d2, 0);

        // Tuesday to Wednesday (exclusive) should have 0 stream days
        let count_d2_d3 = repo.count_stream_days_between(d2, d3).await.unwrap();
        assert_eq!(count_d2_d3, 0);
    }

    #[tokio::test]
    async fn test_gold_sale_vulnerability_fix() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        // 1. Create a player and save a catch
        let p_id = repo.restore_player(&crate::db::PlayerBackup {
            username: "seller".to_string(),
            total_attempts: 1,
            successful_attempts: 1,
            failed_attempts: 0,
            level: 1,
            xp: 0,
            vip_until: None,
            gold: Some(20),
        }).await.unwrap();

        let fish = Fish::new("Daurade".to_string(), Rarity::Common, 10.0, 1.0, "worn".to_string(), "Fish desc".to_string());
        repo.save_attempt(&repo.get_player("seller").await.unwrap().unwrap(), true, Some(fish)).await.unwrap();

        let catches = repo.get_player_catches(p_id).await.unwrap();
        assert_eq!(catches.len(), 1);
        let catch_id = catches[0].id.unwrap();

        // 2. First sale succeeds
        let res_first = repo.execute_gold_sale(p_id, &[catch_id], 50).await;
        assert!(res_first.is_ok());

        let player_after = repo.get_player("seller").await.unwrap().unwrap();
        assert_eq!(player_after.gold, 60); // 10 + 50 = 60 po

        let catches_after = repo.get_player_catches(p_id).await.unwrap();
        assert_eq!(catches_after.len(), 0);

        // 3. Second sale with the same (already deleted) catch MUST fail (preventing duplication!)
        let res_second = repo.execute_gold_sale(p_id, &[catch_id], 50).await;
        assert!(res_second.is_err()); // MUST be Err since the catch is deleted

        // Gold must remain unchanged
        let player_final = repo.get_player("seller").await.unwrap().unwrap();
        assert_eq!(player_final.gold, 60);
    }

    #[tokio::test]
    async fn test_direct_trade_race_condition_fix() {
        let pool = setup_db().await;
        let repo = Repository::new(pool);

        // 1. Create seller and buyer
        let seller_id = repo.restore_player(&crate::db::PlayerBackup {
            username: "seller".to_string(),
            total_attempts: 1,
            successful_attempts: 1,
            failed_attempts: 0,
            level: 1,
            xp: 0,
            vip_until: None,
            gold: Some(0),
        }).await.unwrap();

        let buyer_id = repo.restore_player(&crate::db::PlayerBackup {
            username: "buyer".to_string(),
            total_attempts: 0,
            successful_attempts: 0,
            failed_attempts: 0,
            level: 1,
            xp: 0,
            vip_until: None,
            gold: Some(50), // Buyer has only 50 gold!
        }).await.unwrap();

        // 2. Save a catch to seller
        let fish = Fish::new("Turbot".to_string(), Rarity::Rare, 15.0, 2.0, "good".to_string(), "Fish desc".to_string());
        repo.save_attempt(&repo.get_player("seller").await.unwrap().unwrap(), true, Some(fish)).await.unwrap();

        let catches = repo.get_player_catches(seller_id).await.unwrap();
        assert_eq!(catches.len(), 1);
        let catch_id = catches[0].id.unwrap();

        // 3. Direct trade price is 100 gold. Since buyer has only 50 gold, trade must fail!
        let trade_res = repo.execute_direct_trade(catch_id, seller_id, buyer_id, 100).await;
        assert!(trade_res.is_err()); // Must fail due to atomic gold constraint

        // 4. Verify balances remain unchanged
        let buyer_after = repo.get_player("buyer").await.unwrap().unwrap();
        assert_eq!(buyer_after.gold, 50); // Gold did not go negative

        let seller_after = repo.get_player("seller").await.unwrap().unwrap();
        assert_eq!(seller_after.gold, 0);

        // Verify catch ownership did not transfer
        let catches_seller = repo.get_player_catches(seller_id).await.unwrap();
        assert_eq!(catches_seller.len(), 1);
        assert_eq!(catches_seller[0].id.unwrap(), catch_id);

        let catches_buyer = repo.get_player_catches(buyer_id).await.unwrap();
        assert_eq!(catches_buyer.len(), 0);
    }
}
