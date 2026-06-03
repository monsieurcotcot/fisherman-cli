use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::AppState;
use crate::db::PlayerBackup;

pub fn start_vip_cleanup_task(state: Arc<AppState>) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(60)).await;
            if let Ok(expired_vips) = state.repo.get_expired_vips().await {
                for player in expired_vips {
                    tracing::info!("⏳ [VIP] Expiration pour @{}", player.username);
                    if let Some(tokens) = state.auth.load_tokens() {
                        if let Ok(broadcaster_id) = state.auth.get_user_id(&tokens.access_token, &state.channel).await {
                            if let Ok(user_id) = state.auth.get_user_id(&tokens.access_token, &player.username).await {
                                match state.auth.remove_vip(&tokens.access_token, &broadcaster_id, &user_id).await {
                                    Ok(_) => {
                                        let _ = state.repo.remove_vip_status(player.id.unwrap()).await;
                                        tracing::info!("✅ [VIP] Grade retire pour @{}", player.username);
                                    },
                                    Err(e) => tracing::error!("❌ [VIP] Erreur retrait Helix pour @{} : {}", player.username, e),
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

pub fn start_backup_task(state: Arc<AppState>) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(300)).await;
            if let Ok(players) = state.repo.get_all_players().await {
                let backups: Vec<PlayerBackup> = players.into_iter().map(|p| PlayerBackup {
                    username: p.username,
                    total_attempts: p.total_attempts,
                    successful_attempts: p.successful_attempts,
                    failed_attempts: p.failed_attempts,
                    level: p.level,
                    xp: p.xp,
                    vip_until: p.vip_until,
                    gold: Some(p.gold),
                    eco_notoriety: Some(p.eco_notoriety),
                }).collect();
                
                if let Ok(json) = serde_json::to_string_pretty(&backups) {
                    if let Ok(_) = tokio::fs::write("data/players_backup.json", json).await {
                        tracing::info!("[Backup] {} joueurs sauvegardés dans data/players_backup.json", backups.len());
                    }
                }
            }
        }
    });
}

pub fn start_stream_monitor_task(state: Arc<AppState>) {
    tokio::spawn(async move {
        loop {
            // Check every 5 minutes (300 seconds)
            sleep(Duration::from_secs(300)).await;
            let _ = crate::bot::is_stream_online(&state).await;
        }
    });
}
