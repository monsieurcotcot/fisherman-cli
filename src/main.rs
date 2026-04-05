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

use crate::db::Repository;
use crate::game::generate_fish;
use crate::auth::{AuthManager, MyError};
use crate::config::get_fail_attempt_reasons;

use std::collections::HashMap;
use chrono::DateTime;
use chrono::Utc;

type TwitchClient = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

struct AppState {
    repo: Arc<Repository>,
    auth: Arc<AuthManager>,
    twitch_client: RwLock<Option<TwitchClient>>,
    channel: String,
    pending_resets: RwLock<HashMap<String, DateTime<Utc>>>,
    bot_abort_handle: RwLock<Option<tokio::task::JoinHandle<()>>>,
}

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
            caught_at DATETIME DEFAULT CURRENT_TIMESTAMP
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
    });

    if let Some(tokens) = auth_manager.load_tokens() {
        start_bot(state.clone(), tokens.access_token).await;
    }

    let app = Router::new()
        .route("/", get(|| async { Html(include_str!("../static/index.html")) }))
        .route("/player/{username}", get(|| async { Html(include_str!("../static/index.html")) }))
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
            if let ServerMessage::Privmsg(msg) = message {
                let text = msg.message_text.trim().to_lowercase();
                let username = msg.sender.name.to_lowercase();
                tracing::info!("[Chat] {} : {}", username, text);
                
                if text == "!fish help" || text == "!pêche help" || text == "!peche help" {
                    let _ = client.say(msg.channel_login.clone(), "📖 !fish | !pêche | !fish stats | !fish top".to_string()).await;
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
                            let list: Vec<String> = players.iter().take(5).enumerate().map(|(i, p)| format!("#{}. {} (Niv. {})", i + 1, p.username, p.level)).collect();
                            let _ = client_msg.say(channel_login, format!("🏆 Top Pêcheurs : {}", list.join(" | "))).await;
                        }
                    });
                } else if text == "!fish reset" || text == "!peche reset" || text == "!pêche reset" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        state_task.pending_resets.write().await.insert(username.clone(), Utc::now());
                        let _ = client_msg.say(channel_login, format!("⚠️ @{}, tape !fish yes pour reset.", username)).await;
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
                } else if text == "!fish" || text == "!peche" || text == "!pêche" || (text == "!fish testvip" && (username == "monsieurcotcot" || username == "ze_fisherman" || username == "ze_tester")) {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let is_test = text == "!fish testvip";
                    
                    tokio::spawn(async move {
                        if let Ok(mut player) = state_task.repo.get_or_create_player(&username).await {
                            if player.can_fish() || is_test {
                                let rate = if is_test { 1.0 } else { match player.level { 1..=25 => 0.35, 26..=50 => 0.40, 51..=75 => 0.45, 76..=100 => 0.50, 101..=125 => 0.53, 126..=150 => 0.55, 151..=175 => 0.57, 176..=199 => 0.59, 200 => 0.60, _ => 0.35 } };
                                if rand::random::<f64>() < rate {
                                    let mut fish = if is_test { crate::models::Fish::new("Gemme VIP (TEST)".to_string(), crate::config::Rarity::Legendary, 1.0, 100.0, "pristine".to_string(), "Gemme de test.".to_string()) } 
                                                   else { match generate_fish() { Some(f) => f, None => return } };
                                    
                                    let leveled_up = player.add_xp(25);
                                    if fish.name == "Gemme VIP" || is_test {
                                        let mins = if is_test { 1 } else { match fish.state.as_str() { "badly damaged" => 10, "damaged" => 20, "worn" => 30, "good" => 40, "pristine" => 120, _ => 10 } };
                                        player.vip_until = Some(Utc::now() + chrono::Duration::minutes(mins));
                                        let auth_vip = Arc::clone(&state_task.auth);
                                        let ch_vip = channel_login.clone();
                                        let u_vip = username.clone();
                                        let cl_vip = client_msg.clone();
                                        tokio::spawn(async move {
                                            if let Some(t) = auth_vip.load_streamer_tokens() {
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
                                        let d = if is_test { "1 MIN" } else { match fish.state.as_str() { "pristine" => "2H", "good" => "40 MIN", "worn" => "30 MIN", "damaged" => "20 MIN", _ => "10 MIN" } };
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
                                } else {
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

async fn admin_panel(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let secret = env::var("ADMIN_TOKEN").unwrap_or_else(|_| "change-me".to_string());
    if params.get("token") == Some(&secret) { Html(include_str!("../static/admin.html")).into_response() }
    else { (axum::http::StatusCode::FORBIDDEN, "Acces refuse").into_response() }
}

async fn login_redirect(Query(params): Query<HashMap<String, String>>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
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
    let _ip = headers.get("CF-Connecting-IP").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| addr.ip().to_string());
    let u_low = username.to_lowercase();
    match state.repo.get_or_create_player(&u_low).await {
        Ok(p) => {
            let catches = state.repo.get_player_catches(p.id.unwrap()).await.unwrap_or_default();
            Json(serde_json::json!({ "username": p.username, "total": p.total_attempts, "success": p.successful_attempts, "failed": p.failed_attempts, "can_fish": p.can_fish(), "level": p.level, "xp": p.xp, "xp_next": p.xp_for_next_level(), "is_vip": p.is_vip(), "catches": catches })).into_response()
        },
        Err(_) => Json(serde_json::json!({"error": "Not found"})).into_response(),
    }
}

async fn get_leaderboard(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.repo.get_leaderboard().await {
        Ok(players) => Json(serde_json::json!({"top": players.iter().map(|p| serde_json::json!({"username": p.username, "success": p.successful_attempts, "level": p.level})).collect::<Vec<_>>()})).into_response(),
        Err(_) => Json(serde_json::json!({"error": "Error"})).into_response()
    }
}
