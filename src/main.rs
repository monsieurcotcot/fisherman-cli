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

#[derive(Debug, Clone)]
pub struct PendingSale {
    pub player_id: i64,
    pub catch_ids: Vec<i64>,
    pub catch_names: Vec<String>,
    pub gold_earned: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum PendingTrade {
    Direct {
        seller_id: i64,
        seller_username: String,
        buyer_username: String, // lowercase
        catch_id: i64,
        catch_name: String,
        price: i64,
        created_at: DateTime<Utc>,
    },
    Barter {
        player_a_id: i64,
        player_a_username: String, // lowercase
        catch_a_id: i64,
        catch_a_name: String,
        player_b_username: String, // lowercase
        catch_b_id: Option<i64>, // filled in step 2
        catch_b_name: Option<String>,
        step: u8,
        player_a_accepted: bool,
        player_b_accepted: bool,
        last_activity: DateTime<Utc>,
    },
}

pub struct AppState {
    pub repo: Arc<Repository>,
    pub auth: Arc<AuthManager>,
    pub twitch_client: RwLock<Option<TwitchClient>>,
    pub channel: String,
    pub use_english: bool,
    pub pending_resets: RwLock<HashMap<String, DateTime<Utc>>>,
    pub pending_resets_all: RwLock<HashMap<String, DateTime<Utc>>>,
    pub pending_purges: RwLock<HashMap<String, DateTime<Utc>>>,
    pub bot_abort_handle: RwLock<Option<tokio::task::JoinHandle<()>>>,
    pub rate_limiter: dashmap::DashMap<String, (u32, Option<DateTime<Utc>>)>,
    pub pending_sales: RwLock<HashMap<String, PendingSale>>,
    pub pending_trades: RwLock<Vec<PendingTrade>>,
    pub daily_reward_cache: dashmap::DashMap<String, chrono::NaiveDate>,
    pub offline_attempts: RwLock<HashMap<String, u32>>,
    pub offline_bypassed: RwLock<std::collections::HashSet<String>>,
    pub stream_live_cache: RwLock<Option<(bool, DateTime<Utc>)>>,
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
                gold: Some(p.gold),
                eco_notoriety: Some(p.eco_notoriety),
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

    // Exécution des migrations SQLx standardisées
    sqlx::migrate!().run(&pool).await?;

    let repo = Arc::new(Repository::new(pool.clone()));
    
    // Remplissage automatique du Musée si celui-ci est vide au démarrage
    match repo.is_museum_empty().await {
        Ok(true) => {
            tracing::info!("🔮 [Museum] Le Musée est vide ! Démarrage du backfill des découvertes historiques...");
            if let Err(e) = repo.backfill_museum().await {
                tracing::error!("❌ [Museum] Échec du backfill : {}", e);
            } else {
                tracing::info!("✨ [Museum] Backfill des découvertes historiques terminé avec succès !");
            }
        }
        Ok(false) => {
            tracing::info!("✨ [Museum] Le Musée contient déjà des découvertes.");
        }
        Err(e) => {
            tracing::error!("❌ [Museum] Impossible de vérifier l'état du Musée : {}", e);
        }
    }

    // Correction et couronnement automatique des Rois Bananes actifs au démarrage
    let active_king: Option<(i64, String)> = sqlx::query_as(
        "SELECT c1.player_id, p.username \
         FROM catches c1 \
         JOIN catches c2 ON c1.player_id = c2.player_id \
         JOIN players p ON c1.player_id = p.id \
         WHERE c1.id = (SELECT id FROM catches WHERE fish_name = 'Pristine Banana 1' ORDER BY id DESC LIMIT 1) \
           AND c2.id = (SELECT id FROM catches WHERE fish_name = 'Pristine Banana 2' ORDER BY id DESC LIMIT 1)"
    )
    .fetch_optional(&pool)
    .await
    .unwrap_or_default();

    if let Some((p_id, u_name)) = active_king {
        let has_active: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM banana_kings_history WHERE dethroned_at IS NULL")
            .fetch_one(&pool)
            .await
            .unwrap_or_default();

        if has_active == 0 {
            tracing::info!("🍌 [Banana King] Aucun roi actif trouvé mais @{} possède les deux bananes. Couronnement automatique !", u_name);
            let _ = sqlx::query("INSERT INTO banana_kings_history (player_id, username, dethroned_at) VALUES (?, ?, NULL)")
                .bind(p_id)
                .bind(&u_name)
                .execute(&pool)
                .await;
        } else {
            let current_active_id: Option<i64> = sqlx::query_scalar("SELECT player_id FROM banana_kings_history WHERE dethroned_at IS NULL")
                .fetch_optional(&pool)
                .await
                .unwrap_or_default();

            if current_active_id != Some(p_id) {
                tracing::info!("🍌 [Banana King] Rectification de la couronne : @{} prend son titre légitime !", u_name);
                let mut tx = pool.begin().await.unwrap();
                let _ = sqlx::query("UPDATE banana_kings_history SET dethroned_at = CURRENT_TIMESTAMP WHERE dethroned_at IS NULL")
                    .execute(&mut *tx)
                    .await;
                let _ = sqlx::query("INSERT INTO banana_kings_history (player_id, username, dethroned_at) VALUES (?, ?, NULL)")
                    .bind(p_id)
                    .bind(&u_name)
                    .execute(&mut *tx)
                    .await;
                let _ = tx.commit().await;
            }
        }
    } else {
        // Dethrone any active King since no single player currently holds both latest bananas
        let has_active: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM banana_kings_history WHERE dethroned_at IS NULL")
            .fetch_one(&pool)
            .await
            .unwrap_or_default();
        if has_active > 0 {
            tracing::info!("🍌 [Banana King] Aucun joueur ne possède les deux dernières bananes. Détrônement de l'ancien Roi.");
            let _ = sqlx::query("UPDATE banana_kings_history SET dethroned_at = CURRENT_TIMESTAMP WHERE dethroned_at IS NULL")
                .execute(&pool)
                .await;
        }
    }

    // Correction et couronnement automatique du Champion Écolo actif au démarrage
    if let Ok(mut eco_tx) = pool.begin().await {
        let _ = repo.update_eco_champion_status_direct(&mut eco_tx).await;
        let _ = eco_tx.commit().await;
    }

    let client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID must be set");
    let client_secret = env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET must be set");
    let channel = env::var("TWITCH_CHANNEL").expect("TWITCH_CHANNEL must be set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
    let use_english = env::var("USE_ENGLISH").map(|s| s.to_lowercase() == "true").unwrap_or(false);

    let auth_manager = Arc::new(AuthManager::new(client_id, client_secret, redirect_uri));
    
    let state = Arc::new(AppState {
        repo: Arc::clone(&repo),
        auth: Arc::clone(&auth_manager),
        twitch_client: RwLock::new(None),
        channel: channel.clone(),
        use_english,
        pending_resets: RwLock::new(HashMap::new()),
        pending_resets_all: RwLock::new(HashMap::new()),
        pending_purges: RwLock::new(HashMap::new()),
        bot_abort_handle: RwLock::new(None),
        rate_limiter: dashmap::DashMap::new(),
        pending_sales: RwLock::new(HashMap::new()),
        pending_trades: RwLock::new(Vec::new()),
        daily_reward_cache: dashmap::DashMap::new(),
        offline_attempts: RwLock::new(HashMap::new()),
        offline_bypassed: RwLock::new(std::collections::HashSet::new()),
        stream_live_cache: RwLock::new(None),
    });

    tasks::start_vip_cleanup_task(Arc::clone(&state));
    tasks::start_stream_monitor_task(Arc::clone(&state));

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
                            if let Some(fish) = generate_fish(use_english) {
                                let _ = repo.save_catch_only(player_id, fish, Some(&username)).await;
                                success_count += 1;
                            }
                        } else if r < (success_chance + junk_chance) {
                            if let Some(junk) = generate_junk(use_english) {
                                let _ = repo.save_catch_only(player_id, junk, Some(&username)).await;
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
        let is_valid = auth_manager.validate_token(&tokens.access_token).await;
        
        if tokens.expires_at < Utc::now() || !is_valid {
            if !is_valid {
                tracing::warn!("⚠️ [Auth] Le token de session du Bot est invalide à distance. Lancement d'un rafraîchissement forcé...");
            }
            match auth_manager.refresh_tokens(&tokens.refresh_token).await {
                Ok(new_tokens) => {
                    let _ = auth_manager.save_tokens(&new_tokens);
                    tokens = new_tokens;
                    tracing::info!("✅ [Auth] Token du Bot rafraîchi avec succès.");
                }
                Err(e) => {
                    tracing::error!("Failed to refresh bot tokens: {}", e);
                }
            }
        }
        start_bot(state.clone(), tokens.access_token).await;
    }

    let app = Router::new()
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .route("/", get(|| async { Html(match tokio::fs::read_to_string("static/index.html").await { Ok(h) => h, Err(_) => "Erreur chargement index.html".to_string() }) }))
        .route("/player/{username}", get(|| async { Html(match tokio::fs::read_to_string("static/index.html").await { Ok(h) => h, Err(_) => "Erreur chargement index.html".to_string() }) }))
        .route("/admin-cotcot", get(api::admin_panel))
        .route("/auth/login", get(api::login_redirect))
        .route("/auth/callback", get(api::auth_callback))
        .route("/api/stats/{username}", get(api::get_player_stats))
        .route("/api/leaderboard", get(api::get_leaderboard))
        .route("/api/fish_data", get(api::get_fish_data_api))
        .route("/api/junk_data", get(api::get_junk_data_api))
        .route("/api/banana_kings", get(api::get_banana_kings))
        .route("/api/eco_champions", get(api::get_eco_champions))
        .route("/api/global_museum", get(api::get_global_museum))
        .route("/api/top_eco", get(api::get_top_eco))
        .route("/api/top_banana", get(api::get_top_banana))
        .route("/api/admin/login", axum::routing::post(api::admin_login))
        .route("/api/admin/json", get(api::get_admin_json).post(api::save_admin_json))
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
