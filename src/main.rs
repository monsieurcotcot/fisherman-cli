mod config;
mod models;
mod game;
mod db;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;
use sqlx::sqlite::SqlitePoolOptions;
use dotenvy::dotenv;
use std::env;
use crate::db::Repository;
use crate::game::generate_fish;
use rand::seq::SliceRandom;
use crate::config::get_fail_attempt_reasons;

use axum::{
    routing::get,
    extract::{Path, State},
    Json,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create tables (players and catches)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            total_attempts INTEGER DEFAULT 0,
            successful_attempts INTEGER DEFAULT 0,
            failed_attempts INTEGER DEFAULT 0,
            last_fishing_time DATETIME,
            level INTEGER DEFAULT 1,
            xp INTEGER DEFAULT 0
        )"
    ).execute(&pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS catches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            player_id INTEGER REFERENCES players(id),
            fish_name TEXT NOT NULL,
            rarity TEXT NOT NULL,
            size REAL NOT NULL,
            state TEXT NOT NULL,
            description TEXT,
            caught_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(&pool).await?;

    let repo = Arc::new(Repository::new(pool.clone()));

    // --- TWITCH BOT SETUP ---
    let twitch_username = env::var("TWITCH_USERNAME").expect("TWITCH_USERNAME must be set");
    let twitch_oauth_token = env::var("TWITCH_OAUTH_TOKEN").expect("TWITCH_OAUTH_TOKEN must be set");
    let twitch_channel = env::var("TWITCH_CHANNEL").expect("TWITCH_CHANNEL must be set");

    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);

    let repo_bot = Arc::clone(&repo);
    let client_clone = client.clone();
    let bot_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    let text = msg.message_text.trim();
                    let username = msg.sender.name.clone();
                    
                    tracing::info!("[Chat] {} : {}", username, text);

                    if text.starts_with("!fish") {
                        match repo_bot.get_or_create_player(&username).await {
                            Ok(mut player) => {
                                if player.can_fish(60) {
                                    // Difficulté croissante : le taux de réussite baisse légèrement avec le niveau
                                    let success_rate = 0.45 - (player.level as f64 * 0.001);
                                    let success = rand::random::<f64>() < success_rate;
                                    
                                    if success {
                                        if let Some(fish) = generate_fish() {
                                            tracing::info!("[Peche] @{} a attrape un {} ({} cm)", username, fish.name, fish.size);
                                            
                                            // Gain d'XP en cas de succès (+25 XP)
                                            let leveled_up = player.add_xp(25);
                                            
                                            let mut response = format!("🐟 @{} a pêché un(e) {} ({} cm) ! {}", username, fish.name, fish.size, fish.description);
                                            if leveled_up {
                                                response.push_str(&format!(" ✨ LEVEL UP ! Tu es maintenant niveau {} !", player.level));
                                            }
                                            
                                            client_clone.say(msg.channel_login.clone(), response).await.unwrap();
                                            if let Err(e) = repo_bot.save_attempt(&player, true, Some(fish)).await {
                                                tracing::error!("[Erreur DB] Impossible de sauvegarder la capture : {}", e);
                                            }
                                        }
                                    } else {
                                        let reasons = get_fail_attempt_reasons();
                                        let reason = reasons.choose(&mut rand::thread_rng()).unwrap_or(&"Pas de chance !");
                                        tracing::info!("[Peche] @{} a echoue sa prise.", username);
                                        
                                        // Gain d'XP minimal en cas d'échec (+5 XP)
                                        let leveled_up = player.add_xp(5);
                                        
                                        let mut response = reason.replace("#viewer_name#", &username);
                                        if leveled_up {
                                            response.push_str(&format!(" ✨ LEVEL UP ! Tu es maintenant niveau {} !", player.level));
                                        }
                                        
                                        client_clone.say(msg.channel_login.clone(), response).await.unwrap();
                                        if let Err(e) = repo_bot.save_attempt(&player, false, None).await {
                                            tracing::error!("[Erreur DB] Impossible de sauvegarder l'echec : {}", e);
                                        }
                                    }
                                } else {
                                    tracing::debug!("[Cooldown] @{} a tente de pecher trop tot.", username);
                                    client_clone.say(msg.channel_login.clone(), format!("⏳ @{}, attends un peu ! (Cooldown: 60s)", username)).await.unwrap();
                                }
                            }
                            Err(e) => tracing::error!("[Erreur DB] Impossible de recuperer le joueur {} : {}", username, e),
                        }
                    } else if text.starts_with("!stats") {
                        tracing::info!("[Stats] Demande pour @{}", username);
                        match repo_bot.get_or_create_player(&username).await {
                            Ok(player) => {
                                let response = format!(
                                    "📊 @{} : Niveau {} (XP: {}/{}) | Succès: {} | Stats complètes : http://localhost:3000/player/{}",
                                    username, player.level, player.xp, player.xp_for_next_level(), player.successful_attempts, username
                                );
                                client_clone.say(msg.channel_login.clone(), response).await.unwrap();
                            }
                            Err(e) => tracing::error!("[Erreur DB] {}", e),
                        }
                    } else if text.starts_with("!top") {
                        tracing::info!("[Top] Demande de leaderboard dans le chat");
                        match repo_bot.get_leaderboard().await {
                            Ok(players) => {
                                let mut response = "🏆 Classement des pêcheurs : ".to_string();
                                let top_list: Vec<String> = players.iter().take(5).enumerate()
                                    .map(|(i, p)| format!("#{}. {} ({} 🐟)", i + 1, p.username, p.successful_attempts))
                                    .collect();
                                response.push_str(&top_list.join(" | "));
                                client_clone.say(msg.channel_login.clone(), response).await.unwrap();
                            }
                            Err(e) => tracing::error!("[Erreur DB] Impossible de charger le top : {}", e),
                        }
                    }
                }
                _ => {}
            }
        }
    });

    client.join(twitch_channel.clone())?;
    tracing::info!("[Twitch] Bot connecte a #{}", twitch_channel);

    // --- WEB SERVER SETUP ---
    let app = Router::new()
        .route("/api/stats/:username", get(get_player_stats))
        .route("/api/leaderboard", get(get_leaderboard))
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(Arc::clone(&repo));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("[Web] Serveur API en ligne sur {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Run everything !
    tokio::select! {
        _ = axum::serve(listener, app) => {},
        _ = bot_handle => {},
    }

    Ok(())
}

async fn get_leaderboard(
    State(repo): State<Arc<Repository>>,
) -> Json<serde_json::Value> {
    match repo.get_leaderboard().await {
        Ok(players) => Json(serde_json::json!({
            "top": players.iter().map(|p| serde_json::json!({
                "username": p.username,
                "success": p.successful_attempts,
            })).collect::<Vec<_>>()
        })),
        Err(e) => {
            tracing::error!("[API Error] Impossible de recuperer le leaderboard : {}", e);
            Json(serde_json::json!({"error": "Failed to load leaderboard"}))
        }
    }
}

async fn get_player_stats(
    Path(username): Path<String>,
    State(repo): State<Arc<Repository>>,
) -> Json<serde_json::Value> {
    match repo.get_or_create_player(&username).await {
        Ok(player) => Json(serde_json::json!({
            "username": player.username,
            "total": player.total_attempts,
            "success": player.successful_attempts,
            "failed": player.failed_attempts,
            "can_fish": player.can_fish(30),
        })),
        Err(_) => Json(serde_json::json!({"error": "Player not found"})),
    }
}
