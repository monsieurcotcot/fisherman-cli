mod config;
mod models;
mod game;
mod db;
mod auth;

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
use crate::auth::AuthManager;
use rand::seq::SliceRandom;
use crate::config::get_fail_attempt_reasons;

use axum::{
    routing::get,
    extract::{Path, State, Query},
    Json,
    Router,
    response::Redirect,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use std::sync::Arc;
use tokio::sync::RwLock;

struct AppState {
    repo: Arc<Repository>,
    auth: Arc<AuthManager>,
    twitch_client: RwLock<Option<TwitchIRCClient<TCPTransport, StaticLoginCredentials>>>,
    channel: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create tables
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
    
    let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID must be set");
    let client_secret = env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET must be set");
    let channel = env::var("TWITCH_CHANNEL").expect("TWITCH_CHANNEL must be set");
    let redirect_uri = env::var("REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string());

    let auth_manager = Arc::new(AuthManager::new(client_id, client_secret, redirect_uri));
    
    let state = Arc::new(AppState {
        repo: Arc::clone(&repo),
        auth: Arc::clone(&auth_manager),
        twitch_client: RwLock::new(None),
        channel: channel.clone(),
    });

    // Tentative de chargement des tokens existants
    if let Some(tokens) = auth_manager.load_tokens() {
        tracing::info!("Tokens trouves, tentative de connexion...");
        start_bot(Arc::clone(&state), tokens.access_token).await;
    } else {
        tracing::warn!("Aucun token trouve. Veuillez vous authentifier sur http://localhost:3000/auth");
    }

    // --- WEB SERVER SETUP ---
    let app = Router::new()
        .route("/api/stats/:username", get(get_player_stats))
        .route("/api/leaderboard", get(get_leaderboard))
        .route("/auth", get(login_redirect))
        .route("/auth/callback", get(auth_callback))
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(Arc::clone(&state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("[Web] Serveur API en ligne sur {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn start_bot(state: Arc<AppState>, access_token: String) {
    let credentials = StaticLoginCredentials::new("bot".to_string(), Some(access_token));
    let config = ClientConfig::new_simple(credentials);
    let (mut incoming_messages, client) = TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);

    let mut client_lock = state.twitch_client.write().await;
    *client_lock = Some(client.clone());
    drop(client_lock);

    let state_clone = Arc::clone(&state);
    let channel_name = state.channel.clone();

    tokio::spawn(async move {
        client.join(channel_name.clone()).unwrap();
        tracing::info!("[Twitch] Bot connecte a #{}", channel_name);

        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                let text = msg.message_text.trim();
                let username = msg.sender.name.clone();
                tracing::info!("[Chat] {} : {}", username, text);

                if text.starts_with("!fish") {
                    let mut player = state_clone.repo.get_or_create_player(&username).await.unwrap();
                    if player.can_fish(60) {
                        let success_rate = 0.45 - (player.level as f64 * 0.001);
                        if rand::random::<f64>() < success_rate {
                            if let Some(fish) = generate_fish() {
                                let leveled_up = player.add_xp(25);
                                let mut response = format!("🐟 @{} a pêché un(e) {} ({} cm) ! {}", username, fish.name, fish.size, fish.description);
                                if leveled_up { response.push_str(&format!(" ✨ LEVEL UP ! Tu es maintenant niveau {} !", player.level)); }
                                client.say(msg.channel_login.clone(), response).await.unwrap();
                                state_clone.repo.save_attempt(&player, true, Some(fish)).await.unwrap();
                            }
                        } else {
                            let reasons = get_fail_attempt_reasons();
                            let reason = reasons.choose(&mut rand::thread_rng()).unwrap_or(&"Pas de chance !");
                            let leveled_up = player.add_xp(5);
                            let mut response = reason.replace("#viewer_name#", &username);
                            if leveled_up { response.push_str(&format!(" ✨ LEVEL UP ! Tu es maintenant niveau {} !", player.level)); }
                            client.say(msg.channel_login.clone(), response).await.unwrap();
                            state_clone.repo.save_attempt(&player, false, None).await.unwrap();
                        }
                    } else {
                        client.say(msg.channel_login.clone(), format!("⏳ @{}, attends un peu ! (Cooldown: 60s)", username)).await.unwrap();
                    }
                } else if text.starts_with("!stats") {
                    let player = state_clone.repo.get_or_create_player(&username).await.unwrap();
                    let response = format!("📊 @{} : Niveau {} (XP: {}/{}) | Stats : http://localhost:3000/player/{}", username, player.level, player.xp, player.xp_for_next_level(), username);
                    client.say(msg.channel_login.clone(), response).await.unwrap();
                } else if text.starts_with("!top") {
                    if let Ok(players) = state_clone.repo.get_leaderboard().await {
                        let mut response = "🏆 Top Pêcheurs : ".to_string();
                        let top_list: Vec<String> = players.iter().take(5).enumerate().map(|(i, p)| format!("#{}. {} (Niv. {})", i + 1, p.username, p.level)).collect();
                        response.push_str(&top_list.join(" | "));
                        client.say(msg.channel_login.clone(), response).await.unwrap();
                    }
                }
            }
        }
    });
}

// --- HANDLERS WEB ---

async fn login_redirect(State(state): State<Arc<AppState>>) -> Redirect {
    Redirect::temporary(&state.auth.get_auth_url())
}

#[derive(serde::Deserialize)]
struct AuthQuery { code: String }

async fn auth_callback(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AuthQuery>,
) -> String {
    match state.auth.exchange_code(&query.code).await {
        Ok(tokens) => {
            start_bot(Arc::clone(&state), tokens.access_token).await;
            "Authentification reussie ! Le bot est maintenant connecte au chat. Vous pouvez fermer cette page.".to_string()
        }
        Err(e) => format!("Erreur d'authentification : {}", e),
    }
}

async fn get_player_stats(
    Path(username): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match state.repo.get_or_create_player(&username).await {
        Ok(player) => Json(serde_json::json!({
            "username": player.username,
            "total": player.total_attempts,
            "success": player.successful_attempts,
            "failed": player.failed_attempts,
            "can_fish": player.can_fish(60),
            "level": player.level,
            "xp": player.xp,
            "xp_next": player.xp_for_next_level(),
        })),
        Err(_) => Json(serde_json::json!({"error": "Player not found"})),
    }
}

async fn get_leaderboard(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match state.repo.get_leaderboard().await {
        Ok(players) => Json(serde_json::json!({
            "top": players.iter().map(|p| serde_json::json!({
                "username": p.username,
                "success": p.successful_attempts,
                "level": p.level,
            })).collect::<Vec<_>>()
        })),
        Err(e) => {
            tracing::error!("[API Error] Impossible de recuperer le leaderboard : {}", e);
            Json(serde_json::json!({"error": "Failed to load leaderboard"}))
        }
    }
}
