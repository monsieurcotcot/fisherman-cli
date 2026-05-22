mod config;
mod models;
mod game;
mod db;
mod auth;
mod api;
mod bot;
mod tasks;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::SecureTCPTransport;
use sqlx::sqlite::SqlitePoolOptions;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use axum::{
    routing::get,
    Router,
    response::Html,
    http::{HeaderValue, header::{CONTENT_SECURITY_POLICY, X_FRAME_OPTIONS, STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS}},
};
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::services::ServeFile;

use crate::db::{Repository, PlayerBackup};
use crate::game::{generate_fish, generate_junk};
use crate::auth::{AuthManager, MyError};

pub type TwitchClient = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

pub struct AppState {
    pub repo: Arc<Repository>,
    pub auth: Arc<AuthManager>,
    pub twitch_client: RwLock<Option<TwitchClient>>,
    pub channel: String,
    pub pending_resets: RwLock<HashMap<String, DateTime<Utc>>>,
    pub pending_resets_all: RwLock<HashMap<String, DateTime<Utc>>>,
    pub pending_purges: RwLock<HashMap<String, DateTime<Utc>>>,
    pub bot_abort_handle: RwLock<Option<tokio::task::JoinHandle<()>>>,
    pub rate_limiter: RwLock<HashMap<String, (u32, Option<DateTime<Utc>>)>>,
}

use bot::start_bot;

#[tokio::main]
async fn main() -> Result<(), MyError> {
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

    let catches_info: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as("PRAGMA table_info(catches)").fetch_all(&pool).await.unwrap_or_default();
    let has_id = catches_info.iter().any(|c| c.1 == "id");
    
    if !catches_info.is_empty() && !has_id {
        tracing::warn!("🚨 [Migration] Ancien schéma détecté ! Sauvegarde et Wipe automatique...");
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
            vip_until DATETIME,
            profile_image_url TEXT
        )"
    ).execute(&pool).await?;

    let _ = sqlx::query("ALTER TABLE players ADD COLUMN profile_image_url TEXT").execute(&pool).await;

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

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS banana_kings_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            player_id INTEGER REFERENCES players(id),
            username TEXT NOT NULL,
            crowned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            dethroned_at DATETIME
        )"
    ).execute(&pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS player_trophies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
            username TEXT NOT NULL,
            season TEXT NOT NULL,
            trophy_tier TEXT NOT NULL,
            level INTEGER DEFAULT 1,
            unlocked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(player_id, season)
        )"
    ).execute(&pool).await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_catches_player_id ON catches(player_id);").execute(&pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_players_username ON players(username);").execute(&pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_trophies_player_id ON player_trophies(player_id);").execute(&pool).await?;

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
        pending_resets_all: RwLock::new(HashMap::new()),
        pending_purges: RwLock::new(HashMap::new()),
        bot_abort_handle: RwLock::new(None),
        rate_limiter: RwLock::new(HashMap::new()),
    });

    tasks::start_vip_cleanup_task(Arc::clone(&state));

    if let Ok(0) = repo.count_players().await {
        if let Ok(data) = tokio::fs::read_to_string("data/players_backup.json").await {
            if let Ok(backups) = serde_json::from_str::<Vec<PlayerBackup>>(&data) {
                tracing::info!("⚠️ [Restore] Base de données vide ! Tentative de restauration de {} joueurs...", backups.len());
                for backup in backups {
                    let username = backup.username.clone();
                    let player_id = repo.restore_player(&backup).await?;
                    let mut success_count = 0;
                    let mut fail_count = 0;

                    for _ in 0..backup.total_attempts {
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
                    let _ = repo.update_player_stats_after_restore(player_id, success_count, fail_count).await;
                    tracing::info!("✅ [Restore] {} restauré ({} tentatives simulées).", username, backup.total_attempts);
                }
            }
        }
    }

    tasks::start_backup_task(Arc::clone(&state));

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
        .route("/admin-cotcot", get(api::admin_panel))
        .route("/auth/login", get(api::login_redirect))
        .route("/auth/callback", get(api::auth_callback))
        .route("/api/stats/{username}", get(api::get_player_stats))
        .route("/api/leaderboard", get(api::get_leaderboard))
        .route("/api/fish_data", get(api::get_fish_data_api))
        .route("/api/banana_kings", get(api::get_banana_kings))
        .fallback_service(ServeFile::new("static/index.html"))
        .layer(CorsLayer::permissive())
        .layer(SetResponseHeaderLayer::if_not_present(CONTENT_SECURITY_POLICY, HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https://*.jtvnw.net https://*.twitch.tv; connect-src 'self';")))
        .layer(SetResponseHeaderLayer::if_not_present(X_FRAME_OPTIONS, HeaderValue::from_static("DENY")))
        .layer(SetResponseHeaderLayer::if_not_present(STRICT_TRANSPORT_SECURITY, HeaderValue::from_static("max-age=31536000; includeSubDomains")))
        .layer(SetResponseHeaderLayer::if_not_present(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff")))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}
