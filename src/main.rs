mod config;
mod models;
mod game;
mod db;
mod auth;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;
use twitch_irc::SecureTCPTransport;
use sqlx::sqlite::SqlitePoolOptions;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;
use rand::seq::SliceRandom;

use axum::{
    routing::get,
    extract::{Path, State, Query, ConnectInfo},
    Json,
    Router,
    response::{Redirect, IntoResponse, Html},
    http::{HeaderValue, HeaderMap, header::{CONTENT_SECURITY_POLICY, X_FRAME_OPTIONS, STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS}},
};
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::services::ServeFile;

use crate::db::{Repository, PlayerBackup};
use crate::game::{generate_fish, generate_junk};
use crate::auth::{AuthManager, MyError};
use crate::config::get_fail_attempt_reasons;

use std::collections::HashMap;
use chrono::DateTime;
use chrono::Utc;
use rand::Rng;

type TwitchClient = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

struct AppState {
    repo: Arc<Repository>,
    auth: Arc<AuthManager>,
    twitch_client: RwLock<Option<TwitchClient>>,
    channel: String,
    pending_resets: RwLock<HashMap<String, DateTime<Utc>>>,
    bot_abort_handle: RwLock<Option<tokio::task::JoinHandle<()>>>,
    // IP -> (Nombre de tentatives, Moment du dernier ban)
    rate_limiter: RwLock<HashMap<String, (u32, Option<DateTime<Utc>>)>>,
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    // Forcer la recompilation pour le changement de domaine
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    let _ = sqlx::query("PRAGMA journal_mode=WAL;").execute(&pool).await;
    let _ = sqlx::query("PRAGMA busy_timeout=5000;").execute(&pool).await;

    // --- LOGIQUE DE MIGRATION AUTOMATIQUE (AUTO-WIPE & SIMULATE) ---
    let catches_info: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as("PRAGMA table_info(catches)").fetch_all(&pool).await.unwrap_or_default();
    let has_id = catches_info.iter().any(|c| c.1 == "id");
    
    // Si la table catches existe mais n'a pas d'ID, ou si on veut forcer une migration
    if !catches_info.is_empty() && !has_id {
        tracing::warn!("🚨 [Migration] Ancien schéma détecté ! Sauvegarde et Wipe automatique...");
        
        // 1. On tente un backup de sécurité des joueurs actuels avant de wiper
        let temp_repo = Repository::new(pool.clone());
        if let Ok(players) = temp_repo.get_all_players().await {
            let backups: Vec<PlayerBackup> = players.into_iter().map(|p| PlayerBackup {
                username: p.username,
                total_attempts: p.total_attempts,
                successful_attempts: p.successful_attempts,
                failed_attempts: p.failed_attempts,
                level: p.level,
                xp: p.xp,
                vip_until: p.vip_until,
            }).collect();
            if let Ok(json) = serde_json::to_string_pretty(&backups) {
                let _ = tokio::fs::write("data/players_backup.json", json).await;
                tracing::info!("✅ [Migration] Backup de sécurité créé.");
            }
        }

        // 2. On wipe les tables
        let _ = sqlx::query("DROP TABLE IF EXISTS catches").execute(&pool).await;
        let _ = sqlx::query("DROP TABLE IF EXISTS players").execute(&pool).await;
        tracing::info!("🧹 [Migration] Tables supprimées.");
    }

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            total_attempts INTEGER DEFAULT 0,
            successful_attempts INTEGER DEFAULT 0,
            failed_attempts INTEGER DEFAULT 0,
            last_fishing_time DATETIME,
            level INTEGER DEFAULT 1,
            xp INTEGER DEFAULT 0,
            vip_until DATETIME
        )"
    ).execute(&pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS catches (
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
    ).execute(&pool).await?;

    let repo = Arc::new(Repository::new(pool.clone()));
    
    let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID must be set");
    let client_secret = env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET must be set");
    let channel = env::var("TWITCH_CHANNEL").expect("TWITCH_CHANNEL must be set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

    let auth_manager = Arc::new(AuthManager::new(client_id, client_secret, redirect_uri));
    
    let state = Arc::new(AppState {
        repo: Arc::clone(&repo),
        auth: Arc::clone(&auth_manager),
        twitch_client: RwLock::new(None),
        channel: channel.clone(),
        pending_resets: RwLock::new(HashMap::new()),
        bot_abort_handle: RwLock::new(None),
        rate_limiter: RwLock::new(HashMap::new()),
    });

    // --- LOGIQUE DE RESTAURATION ---
    if let Ok(0) = repo.count_players().await {
        if let Ok(data) = tokio::fs::read_to_string("data/players_backup.json").await {
            if let Ok(backups) = serde_json::from_str::<Vec<PlayerBackup>>(&data) {
                tracing::info!("⚠️ [Restore] Base de données vide ! Tentative de restauration de {} joueurs...", backups.len());
                for backup in backups {
                    let username = backup.username.clone();
                    let player_id = repo.restore_player(&backup).await?;
                    let mut success_count = 0;
                    let mut fail_count = 0;

                    // Simulation des tentatives globales pour reconstruire l'inventaire
                    for _ in 0..backup.total_attempts {
                        // Chances dynamiques basées sur le niveau restauré
                        let success_chance = 0.35 + (backup.level as f64 - 1.0) * (0.20 / 199.0);
                        let junk_chance = 0.05;
                        
                        let r: f64 = rand::random::<f64>();
                        if r < success_chance {
                            if let Some(fish) = generate_fish() {
                                let _ = repo.save_catch_only(player_id, fish).await;
                                success_count += 1;
                            }
                        } else if r < (success_chance + junk_chance) {
                            if let Some(junk) = generate_junk() {
                                let _ = repo.save_catch_only(player_id, junk).await;
                                success_count += 1;
                            }
                        } else {
                            fail_count += 1;
                        }
                    }
                    // Mise à jour finale des compteurs pour matcher l'inventaire simulé
                    let _ = repo.update_player_stats_after_restore(player_id, success_count, fail_count).await;
                    tracing::info!("✅ [Restore] {} restauré ({} tentatives simulées).", username, backup.total_attempts);
                }
            }
        }
    }

    // --- TACHE DE BACKUP PERIODIQUE ---
    let repo_backup = Arc::clone(&repo);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await; // Toutes les 5 minutes
            if let Ok(players) = repo_backup.get_all_players().await {
                let backups: Vec<PlayerBackup> = players.into_iter().map(|p| PlayerBackup {
                    username: p.username,
                    total_attempts: p.total_attempts,
                    successful_attempts: p.successful_attempts,
                    failed_attempts: p.failed_attempts,
                    level: p.level,
                    xp: p.xp,
                    vip_until: p.vip_until,
                }).collect();
                
                if let Ok(json) = serde_json::to_string_pretty(&backups) {
                    if let Ok(_) = tokio::fs::write("data/players_backup.json", json).await {
                        tracing::info!("[Backup] {} joueurs sauvegardés dans data/players_backup.json", backups.len());
                    }
                }
            }
        }
    });

    if let Some(mut tokens) = auth_manager.load_tokens() {
        if tokens.expires_at < Utc::now() {
            match auth_manager.refresh_tokens(&tokens.refresh_token).await {
                Ok(new_tokens) => {
                    let _ = auth_manager.save_tokens(&new_tokens);
                    tokens = new_tokens;
                }
                Err(e) => {
                    tracing::error!("Failed to refresh bot tokens: {}", e);
                }
            }
        }
        start_bot(state.clone(), tokens.access_token).await;
    }

    let app = Router::new()
        .route("/", get(|| async { Html(match tokio::fs::read_to_string("static/index.html").await { Ok(h) => h, Err(_) => "Erreur chargement index.html".to_string() }) }))
        .route("/player/{username}", get(|| async { Html(match tokio::fs::read_to_string("static/index.html").await { Ok(h) => h, Err(_) => "Erreur chargement index.html".to_string() }) }))
        .route("/admin-cotcot", get(admin_panel))
        .route("/auth/login", get(login_redirect))
        .route("/auth/callback", get(auth_callback))
        .route("/api/stats/{username}", get(get_player_stats))
        .route("/api/leaderboard", get(get_leaderboard))
        .fallback_service(ServeFile::new("static/index.html"))
        .layer(CorsLayer::permissive())
        .layer(SetResponseHeaderLayer::if_not_present(CONTENT_SECURITY_POLICY, HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self';")))
        .layer(SetResponseHeaderLayer::if_not_present(X_FRAME_OPTIONS, HeaderValue::from_static("DENY")))
        .layer(SetResponseHeaderLayer::if_not_present(STRICT_TRANSPORT_SECURITY, HeaderValue::from_static("max-age=31536000; includeSubDomains")))
        .layer(SetResponseHeaderLayer::if_not_present(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff")))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}

async fn start_bot(state: Arc<AppState>, access_token: String) {
    let mut abort_lock = state.bot_abort_handle.write().await;
    if let Some(handle) = abort_lock.take() { handle.abort(); }

    let credentials = StaticLoginCredentials::new("bot".to_string(), Some(access_token));
    let config = ClientConfig::new_simple(credentials);
    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let mut client_lock = state.twitch_client.write().await;
    *client_lock = Some(client.clone());
    drop(client_lock);

    let state_clone = Arc::clone(&state);
    let channel_name = state.channel.clone();

    let handle = tokio::spawn(async move {
        let _ = client.join(channel_name.to_lowercase());
        let channel_pulse = channel_name.to_lowercase();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                tracing::info!("[Pulse] #{}", channel_pulse);
            }
        });

        while let Some(message) = incoming_messages.recv().await {
            // ... (rest of function unchanged)
            if let ServerMessage::Privmsg(msg) = message {
                let text = msg.message_text.trim().to_lowercase();
                let username = msg.sender.name.to_lowercase();
                tracing::info!("[Chat] {} : {}", username, text);
                
                if text == "!fish help" || text == "!pêche help" || text == "!peche help" {
                    let mut help_msg = "📖 !fish | !pêche | !fish stats | !fish top | !fish reset".to_string();
                    if username == "monsieurcotcot" {
                        help_msg.push_str(" | 🛠️ Admin: !admin backup | !admin restore | !fish reset <pseudo> | !fish simulate <pseudo> <n>");
                    }
                    let _ = client.say(msg.channel_login.clone(), help_msg).await;
                } else if text == "!fish stats" || text == "!fish stat" || text == "!peche stats" || text == "!pêche stats" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let base_url = env::var("REDIRECT_URI").unwrap_or_default().replace("/auth/callback", "");
                    tokio::spawn(async move {
                        if let Ok(p) = state_task.repo.get_or_create_player(&username).await {
                            let _ = client_msg.say(channel_login, format!("📊 @{} : Niv. {} (XP: {}/{}) | Stats : {}/player/{}", username, p.level, p.xp, p.xp_for_next_level(), base_url, username)).await;
                        }
                    });
                } else if text == "!fish top" || text == "!peche top" || text == "!pêche top" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(players) = state_task.repo.get_leaderboard().await {
                            let list: Vec<String> = players.iter().take(5).enumerate().map(|(i, p)| format!("#{}. {} (Niv. {} | {} 🐟 | {} 🗑️ | {} 🍌 | {} 📜)", i + 1, p.username, p.level, p.successful_attempts, p.junk_count, p.banana_count, p.postcard_count)).collect();
                            let _ = client_msg.say(channel_login, format!("🏆 Top Pêcheurs : {}", list.join(" | "))).await;
                        }
                    });
                } else if text.starts_with("!fish reset") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let args: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

                    tokio::spawn(async move {
                        // Cas Admin : !fish reset <pseudo>
                        if args.len() >= 3 && username == "monsieurcotcot" {
                            let target = args[2].to_lowercase();
                            if let Ok(_) = state_task.repo.reset_player(&target).await {
                                let _ = client_msg.say(channel_login, format!("♻️ @{}, l'inventaire de @{} a été réinitialisé par l'administrateur.", username, target)).await;
                            }
                        } else {
                            // Cas classique : Reset de soi-meme avec confirmation
                            state_task.pending_resets.write().await.insert(username.clone(), Utc::now());
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, tape !fish yes pour confirmer ton propre reset.", username)).await;
                        }
                    });
                } else if text == "!fish yes" || text == "!peche yes" || text == "!pêche yes" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        let mut pending = state_task.pending_resets.write().await;
                        if let Some(time) = pending.get(&username) {
                            if Utc::now().signed_duration_since(*time).num_minutes() < 2 {
                                if let Ok(_) = state_task.repo.reset_player(&username).await {
                                    let _ = client_msg.say(channel_login, format!("♻️ @{}, reset réussi !", username)).await;
                                }
                                pending.remove(&username);
                            }
                        }
                    });
                } else if text.starts_with("!fish simulate ") && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let args: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

                    if args.len() >= 3 {
                        let target_user = args[2].to_lowercase();
                        let count = if args.len() >= 4 { args[3].parse::<u32>().unwrap_or(10) } else { 10 };

                        tokio::spawn(async move {
                            tracing::info!("[Admin] Simulation de {} lancers pour {}", count, target_user);
                            if let Ok(mut player) = state_task.repo.get_or_create_player(&target_user).await {
                                let mut success_count = 0;
                                let mut junk_count = 0;
                                let mut fail_count = 0;

                                for _ in 0..count {
                                    let level_factor = (player.level as f64 - 1.0) / 199.0;
                                    let success_rate = 0.35 + (level_factor * 0.20);
                                    let junk_rate = 0.05;
                                    let roll = rand::random::<f64>();

                                    if roll < success_rate {
                                        if let Some(fish) = generate_fish() {
                                            success_count += 1;
                                            player.add_xp(25);
                                            let _ = state_task.repo.save_attempt(&player, true, Some(fish)).await;
                                        }
                                    } else if roll < success_rate + junk_rate {
                                        if let Some(junk) = generate_junk() {
                                            junk_count += 1;
                                            player.add_xp(5);
                                            let _ = state_task.repo.save_attempt(&player, false, Some(junk)).await;
                                        }
                                    } else {
                                        fail_count += 1;
                                        player.add_xp(5);
                                        let _ = state_task.repo.save_attempt(&player, false, None).await;
                                    }
                                }
                                let _ = client_msg.say(channel_login, format!("✅ Simulation terminée pour @{} : {} poissons, {} déchets, {} échecs. Niv. {}", target_user, success_count, junk_count, fail_count, player.level)).await;
                            }
                        });
                    }
                } else if text == "!admin backup" && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(players) = state_task.repo.get_all_players().await {
                            let backups: Vec<PlayerBackup> = players.into_iter().map(|p| PlayerBackup {
                                username: p.username,
                                total_attempts: p.total_attempts,
                                successful_attempts: p.successful_attempts,
                                failed_attempts: p.failed_attempts,
                                level: p.level,
                                xp: p.xp,
                                vip_until: p.vip_until,
                            }).collect();
                            
                            if let Ok(json) = serde_json::to_string_pretty(&backups) {
                                if let Ok(_) = tokio::fs::write("data/players_backup.json", json).await {
                                    let _ = client_msg.say(channel_login, format!("💾 [Admin] Backup forcé : {} joueurs sauvegardés.", backups.len())).await;
                                }
                            }
                        }
                    });
                } else if text == "!admin restore" && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(data) = tokio::fs::read_to_string("data/players_backup.json").await {
                            if let Ok(backups) = serde_json::from_str::<Vec<PlayerBackup>>(&data) {
                                let _ = client_msg.say(channel_login.clone(), format!("⚠️ [Admin] Restauration forcée de {} joueurs en cours...", backups.len())).await;
                                for backup in backups {
                                    if let Ok(player_id) = state_task.repo.restore_player(&backup).await {
                                        let mut success_count = 0;
                                        let mut fail_count = 0;
                                        for _ in 0..backup.total_attempts {
                                            let success_chance = 0.35 + (backup.level as f64 - 1.0) * (0.20 / 199.0);
                                            let junk_chance = 0.05;
                                            let r: f64 = rand::random::<f64>();
                                            if r < success_chance {
                                                if let Some(fish) = generate_fish() {
                                                    let _ = state_task.repo.save_catch_only(player_id, fish).await;
                                                    success_count += 1;
                                                }
                                            } else if r < (success_chance + junk_chance) {
                                                if let Some(junk) = generate_junk() {
                                                    let _ = state_task.repo.save_catch_only(player_id, junk).await;
                                                    success_count += 1;
                                                }
                                            } else {
                                                fail_count += 1;
                                            }
                                        }
                                        let _ = state_task.repo.update_player_stats_after_restore(player_id, success_count, fail_count).await;
                                    }
                                }
                                let _ = client_msg.say(channel_login, "✅ [Admin] Restauration terminée.".to_string()).await;
                            }
                        }
                    });
                } else if text == "!fish" || text == "!peche" || text == "!pêche" || (text == "!fish testvip" && (username == "monsieurcotcot" || username == "ze_fisherman" || username == "ze_tester")) {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let is_test = text == "!fish testvip";

                    tokio::spawn(async move {
                        if let Ok(mut player) = state_task.repo.get_or_create_player(&username).await {
                            let is_admin = username == "monsieurcotcot";
                            if player.can_fish() || is_test || is_admin {
                                // Calcul des taux basés sur le niveau (1 à 200)
                                let level_factor = (player.level as f64 - 1.0) / 199.0;
                                let success_rate = 0.35 + (level_factor * 0.20);
                                let junk_rate = 0.05;
                                let roll = rand::random::<f64>();

                                if is_test || roll < success_rate {
                                    let mut fish = if is_test { crate::models::Fish::new("Gemme VIP (TEST)".to_string(), crate::config::Rarity::Legendary, 1.0, 100.0, "pristine".to_string(), "Gemme de test.".to_string()) } 
                                                   else { match generate_fish() { Some(f) => f, None => return } };

                                    let leveled_up = player.add_xp(25);
                                    if fish.name == "Gemme VIP" || is_test {
                                        let mins = if is_test { 1 } else { match fish.state.as_str() { "badly damaged" => 20, "damaged" => 40, "worn" => 60, "good" => 80, "pristine" => 240, _ => 20 } };
                                        player.vip_until = Some(Utc::now() + chrono::Duration::minutes(mins));
                                        let auth_vip = Arc::clone(&state_task.auth);
                                        let ch_vip = channel_login.clone();
                                        let u_vip = username.clone();
                                        let cl_vip = client_msg.clone();
                                        tokio::spawn(async move {
                                            if let Some(mut t) = auth_vip.load_streamer_tokens() {
                                                if t.expires_at < Utc::now() {
                                                    if let Ok(new_t) = auth_vip.refresh_tokens(&t.refresh_token).await {
                                                        let _ = auth_vip.save_streamer_tokens(&new_t);
                                                        t = new_t;
                                                    }
                                                }
                                                if let (Some(b), Some(u)) = (auth_vip.get_user_id(&ch_vip, &t.access_token).await, auth_vip.get_user_id(&u_vip, &t.access_token).await) {
                                                    let _ = auth_vip.add_vip(&b, &u, &t.access_token).await;
                                                    tokio::time::sleep(tokio::time::Duration::from_secs(mins as u64 * 60)).await;
                                                    if auth_vip.remove_vip(&b, &u, &t.access_token).await {
                                                        let _ = cl_vip.say(ch_vip, format!("🔔 @{}, ton grade VIP a expiré. Merci !", u_vip)).await;
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    let mut resp = format!("🐟 @{} a pêché un(e) {} ({} cm) ! {}", username, fish.name, fish.size, fish.description);
                                    if fish.name == "Gemme VIP" || is_test { 
                                        let d = if is_test { "1 MIN" } else { match fish.state.as_str() { "pristine" => "4H", "good" => "80 MIN", "worn" => "60 MIN", "damaged" => "40 MIN", _ => "20 MIN" } };
                                        resp.push_str(&format!(" 🌟 TU ES VIP PENDANT {} ! 🌟", d)); 
                                    }
                                    if leveled_up { resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level)); }
                                    let _ = client_msg.say(channel_login.clone(), resp).await;

                                    let state_bg = state_task.clone();
                                    let ch_bg = channel_login.clone();
                                    tokio::spawn(async move {
                                        if let Some(t) = state_bg.auth.load_tokens() { fish.stream_title = state_bg.auth.get_stream_info(&ch_bg, &t.access_token).await; }
                                        let _ = state_bg.repo.save_attempt(&player, true, Some(fish)).await;
                                    });
                                } else if roll < success_rate + junk_rate {
                                    // DÉCHET
                                    if let Some(mut junk) = generate_junk() {
                                        let leveled_up = player.add_xp(5); // Les déchets donnent un peu d'XP
                                        let mut resp = format!("🗑️ @{} a remonté un déchet : {} ! {}", username, junk.name, junk.description);
                                        if junk.rarity != crate::config::Rarity::Common { resp.push_str(&format!(" (Rareté: {:?})", junk.rarity)); }

                                        if leveled_up { resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level)); }
                                        let _ = client_msg.say(channel_login.clone(), resp).await;

                                        let state_bg = state_task.clone();
                                        let ch_bg = channel_login.clone();
                                        tokio::spawn(async move {
                                            if let Some(t) = state_bg.auth.load_tokens() { junk.stream_title = state_bg.auth.get_stream_info(&ch_bg, &t.access_token).await; }
                                            let _ = state_bg.repo.save_attempt(&player, false, Some(junk)).await;
                                        });
                                    }
                                } else {                                    // ÉCHEC TOTAL
                                    let reasons = get_fail_attempt_reasons();
                                    let reason = reasons.choose(&mut rand::thread_rng()).unwrap_or(&"Pas de chance !").replace("#viewer_name#", &username);
                                    let leveled_up = player.add_xp(5);
                                    let mut resp = reason;
                                    if leveled_up { resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level)); }
                                    let _ = client_msg.say(channel_login, resp).await;
                                    let _ = state_task.repo.save_attempt(&player, false, None).await;
                                }
                            } else {
                                let rem = player.get_remaining_cooldown();
                                if let Some(id) = player.id { let _ = state_task.repo.add_cooldown_penalty(id).await; }
                                let _ = client_msg.say(channel_login, format!("⏳ @{}, attends encore {}s ! (+5s penalty) ⚠️", username, rem + 5)).await;
                            }
                        }
                    });
                }
            }
        }
    });
    *abort_lock = Some(handle);
}

async fn check_rate_limit(state: &AppState, ip: &str) -> bool {
    let mut limiter = state.rate_limiter.write().await;
    let now = Utc::now();
    
    let entry = limiter.entry(ip.to_string()).or_insert((0, None));
    
    // 1. Verifier si l'IP est actuellement bannie
    if let Some(ban_time) = entry.1 {
        if now.signed_duration_since(ban_time).num_minutes() < 15 {
            return false; // Toujours banni
        } else {
            entry.1 = None; // Fin du ban
            entry.0 = 0;
        }
    }
    
    // 2. Incremeter les tentatives
    entry.0 += 1;
    
    // 3. Bannir si trop de tentatives (ex: 5 essais infructueux)
    if entry.0 > 10 {
        entry.1 = Some(now);
        tracing::warn!("[SECURITY] IP BANNIE (15 min) suite a un spam/bruteforce : {}", ip);
        return false;
    }
    
    true
}

async fn admin_panel(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let client_ip = headers
        .get("CF-Connecting-IP")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| addr.ip().to_string());
    
    if !check_rate_limit(&state, &client_ip).await {
        return (axum::http::StatusCode::TOO_MANY_REQUESTS, "Accès temporairement bloqué pour spam").into_response();
    }

    let secret = env::var("ADMIN_TOKEN").unwrap_or_else(|_| "change-me".to_string());
    if params.get("token") == Some(&secret) {
        Html(match tokio::fs::read_to_string("static/admin.html").await { Ok(h) => h, Err(_) => "Erreur chargement admin.html".to_string() }).into_response()
    } else {
        (axum::http::StatusCode::FORBIDDEN, "Accès refusé").into_response()
    }
}

async fn login_redirect(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let client_ip = headers
        .get("CF-Connecting-IP")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| addr.ip().to_string());

    if !check_rate_limit(&state, &client_ip).await {
        return (axum::http::StatusCode::TOO_MANY_REQUESTS, "Accès temporairement bloqué pour spam").into_response();
    }

    let secret = env::var("ADMIN_TOKEN").unwrap_or_else(|_| "change-me".to_string());
    if params.get("token") != Some(&secret) { return (axum::http::StatusCode::FORBIDDEN, "Acces refuse").into_response(); }
    let is_streamer = params.get("type").map(|t| t == "streamer").unwrap_or(false);
    Redirect::temporary(&state.auth.get_auth_url(is_streamer)).into_response()
}

#[derive(serde::Deserialize)]
struct AuthQuery { code: String, state: String }

async fn auth_callback(State(app_state): State<Arc<AppState>>, Query(query): Query<AuthQuery>) -> impl IntoResponse {
    let code = query.code.clone();
    let is_streamer = query.state == "streamer";
    let state_clone = Arc::clone(&app_state);
    match app_state.auth.exchange_code(&code).await {
        Ok(t) => {
            if is_streamer { let _ = app_state.auth.save_streamer_tokens(&t); Html("<h1>Authentification Streameur reussie !</h1>").into_response() }
            else { let _ = app_state.auth.save_tokens(&t); start_bot(state_clone, t.access_token).await; Html("<h1>Authentification Bot reussie !</h1>").into_response() }
        },
        Err(e) => Html(format!("<h1>Erreur</h1><p>{}</p>", e)).into_response(),
    }
}

async fn get_player_stats(headers: HeaderMap, ConnectInfo(addr): ConnectInfo<SocketAddr>, Path(username): Path<String>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client_ip = headers.get("CF-Connecting-IP").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| addr.ip().to_string());
    tracing::info!("[Web] Recherche de {} par l'IP : {}", username, client_ip);
    let u_low = username.to_lowercase();
    match state.repo.get_player(&u_low).await {
        Ok(Some(p)) => {
            let catches = state.repo.get_player_catches(p.id.unwrap()).await.unwrap_or_default();
            Json(serde_json::json!({ "username": p.username, "total": p.total_attempts, "success": p.successful_attempts, "failed": p.failed_attempts, "can_fish": p.can_fish(), "level": p.level, "xp": p.xp, "xp_next": p.xp_for_next_level(), "is_vip": p.is_vip(), "catches": catches })).into_response()
        },
        _ => Json(serde_json::json!({"error": "Player not found"})).into_response(),
    }
}

async fn get_leaderboard(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.repo.get_leaderboard().await {
        Ok(players) => Json(serde_json::json!({"top": players.iter().map(|p| serde_json::json!({"username": p.username, "success": p.successful_attempts, "level": p.level, "junk": p.junk_count, "banana": p.banana_count, "postcard": p.postcard_count})).collect::<Vec<_>>()})).into_response(),
        Err(_) => Json(serde_json::json!({"error": "Error"})).into_response()
    }
}
