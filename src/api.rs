use axum::{
    extract::{Path, State, Query, ConnectInfo},
    Json,
    response::{Redirect, IntoResponse, Html},
    http::HeaderMap,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::net::SocketAddr;
use chrono::Utc;

use crate::{AppState, start_bot};

pub async fn check_rate_limit(state: &AppState, ip: &str) -> bool {
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

pub async fn admin_panel(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
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

    Html(match tokio::fs::read_to_string("static/admin.html").await {
        Ok(h) => h,
        Err(_) => "Erreur chargement admin.html".to_string()
    }).into_response()
}

pub async fn login_redirect(
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

    let secret = std::env::var("ADMIN_TOKEN").unwrap_or_else(|_| "change-me".to_string());
    if params.get("token") != Some(&secret) { return (axum::http::StatusCode::FORBIDDEN, "Acces refuse").into_response(); }
    let is_streamer = params.get("type").map(|t| t == "streamer").unwrap_or(false);
    Redirect::temporary(&state.auth.get_auth_url(is_streamer)).into_response()
}

#[derive(serde::Deserialize)]
pub struct AuthQuery { pub code: String, pub state: String }

pub async fn auth_callback(State(app_state): State<Arc<AppState>>, Query(query): Query<AuthQuery>) -> impl IntoResponse {
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

pub async fn get_player_stats(headers: HeaderMap, ConnectInfo(addr): ConnectInfo<SocketAddr>, Path(username): Path<String>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client_ip = headers.get("CF-Connecting-IP").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| addr.ip().to_string());
    tracing::info!("[Web] Recherche de {} par l'IP : {}", username, client_ip);
    let u_low = username.to_lowercase();
    match state.repo.get_player(&u_low).await {
        Ok(Some(p)) => {
            let catches = state.repo.get_player_catches(p.id.unwrap()).await.unwrap_or_default();
            let trophies = state.repo.get_player_trophies(p.id.unwrap()).await.unwrap_or_default();
            let museum = state.repo.get_player_museum(p.id.unwrap()).await.unwrap_or_default();
            
            let mut profile_image_url = p.profile_image_url.clone();
            if profile_image_url.is_none() {
                let mut fetched = false;
                
                // 1. Essayer avec les jetons du bot
                if let Some(tokens) = state.auth.load_tokens() {
                    if let Ok(img) = state.auth.get_user_profile_image(&tokens.access_token, &p.username).await {
                        let _ = state.repo.update_player_profile_image(p.id.unwrap(), &img).await;
                        profile_image_url = Some(img);
                        fetched = true;
                    }
                }
                
                // 2. Repli sur les jetons du streamer si échec/absent
                if !fetched {
                    if let Some(tokens) = state.auth.load_streamer_tokens() {
                        if let Ok(img) = state.auth.get_user_profile_image(&tokens.access_token, &p.username).await {
                            let _ = state.repo.update_player_profile_image(p.id.unwrap(), &img).await;
                            profile_image_url = Some(img);
                        }
                    }
                }
            }
            let is_banana_king = state.repo.is_active_king(p.id.unwrap()).await.unwrap_or(false);
            let has_banana_1 = state.repo.has_caught_banana(p.id.unwrap(), "Pristine Banana 1").await.unwrap_or(false);
            let has_banana_2 = state.repo.has_caught_banana(p.id.unwrap(), "Pristine Banana 2").await.unwrap_or(false);

            Json(serde_json::json!({
                "username": p.username,
                "is_banana_king": is_banana_king,
                "has_banana_1": has_banana_1,
                "has_banana_2": has_banana_2,
                "total": p.total_attempts,
                "success": p.successful_attempts,
                "failed": p.failed_attempts,
                "can_fish": p.can_fish(),
                "level": p.level,
                "xp": p.xp,
                "xp_next": p.xp_for_next_level(),
                "is_vip": p.is_vip(),
                "catches": catches,
                "trophies": trophies,
                "museum": museum,
                "profile_image_url": profile_image_url,
                "junk": p.junk_count,
                "banana": p.banana_count,
                "postcard": p.postcard_count,
                "gem": p.gem_count,
                "gold": p.gold,
                "coinflip_wins": p.coinflip_wins,
                "coinflip_losses": p.coinflip_losses,
                "coinflip_biggest_win": p.coinflip_biggest_win,
                "coinflip_biggest_loss": p.coinflip_biggest_loss,
                "coinflip_gold_won_total": p.coinflip_gold_won_total,
                "coinflip_gold_lost_total": p.coinflip_gold_lost_total,
                "coinflip_current_win_streak": p.coinflip_current_win_streak,
                "coinflip_current_loss_streak": p.coinflip_current_loss_streak,
                "coinflip_max_win_streak": p.coinflip_max_win_streak,
                "coinflip_max_loss_streak": p.coinflip_max_loss_streak,
                "gold_given_total": p.gold_given_total,
                "max_gold_held": p.max_gold_held
            })).into_response()
        },
        _ => Json(serde_json::json!({"error": "Player not found"})).into_response(),
    }
}

pub async fn get_fish_data_api() -> impl IntoResponse {
    Json(crate::config::get_fish_data()).into_response()
}

pub async fn get_junk_data_api() -> impl IntoResponse {
    Json(crate::config::get_junk_data()).into_response()
}

pub async fn get_leaderboard(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.repo.get_leaderboard().await {
        Ok(players) => Json(serde_json::json!({"top": players.iter().map(|p| serde_json::json!({"username": p.username, "success": p.successful_attempts, "level": p.level, "junk": p.junk_count, "banana": p.banana_count, "postcard": p.postcard_count, "gem": p.gem_count, "gold": p.gold})).collect::<Vec<_>>()})).into_response(),
        Err(_) => Json(serde_json::json!({"error": "Error"})).into_response()
    }
}

pub async fn get_banana_kings(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.repo.get_banana_kings_history().await {
        Ok(history) => Json(serde_json::json!({ "history": history })).into_response(),
        Err(_) => Json(serde_json::json!({ "error": "Error fetching banana kings history" })).into_response()
    }
}

use std::sync::RwLock as StdRwLock;
use std::collections::HashMap as StdHashMap;
use chrono::{DateTime, Duration};

#[derive(Clone, Debug)]
pub struct FailedLoginState {
    pub attempt_count: u32,
    pub banned_until: Option<DateTime<Utc>>,
    pub is_permabanned: bool,
}

static FAILED_LOGINS: StdRwLock<Option<StdHashMap<String, FailedLoginState>>> = StdRwLock::new(None);
static VALID_SESSIONS: StdRwLock<Option<StdHashMap<String, DateTime<Utc>>>> = StdRwLock::new(None);

pub fn check_login_ban(ip: &str) -> Result<(), String> {
    let logins = FAILED_LOGINS.read().unwrap();
    if let Some(map) = &*logins {
        if let Some(state) = map.get(ip) {
            if state.is_permabanned {
                return Err("Banni de façon permanente".to_string());
            }
            if let Some(until) = state.banned_until {
                let now = Utc::now();
                if now < until {
                    let diff = until.signed_duration_since(now).num_seconds();
                    return Err(format!("Accès bloqué. Réessayez dans {} secondes", diff));
                }
            }
        }
    }
    Ok(())
}

pub fn record_failed_login(ip: &str) -> String {
    let mut logins = FAILED_LOGINS.write().unwrap();
    let map = logins.get_or_insert_with(StdHashMap::new);
    let state = map.entry(ip.to_string()).or_insert(FailedLoginState {
        attempt_count: 0,
        banned_until: None,
        is_permabanned: false,
    });

    state.attempt_count += 1;
    
    match state.attempt_count {
        1 | 2 => {
            format!("Mot de passe incorrect (Tentative {}/3 avant blocage)", state.attempt_count)
        }
        3 => {
            state.banned_until = Some(Utc::now() + Duration::seconds(30));
            "Mot de passe incorrect. IP bloquée pour 30 secondes.".to_string()
        }
        4 => {
            state.banned_until = Some(Utc::now() + Duration::minutes(1));
            "Mot de passe incorrect. IP bloquée pour 1 minute.".to_string()
        }
        5 => {
            state.banned_until = Some(Utc::now() + Duration::minutes(5));
            "Mot de passe incorrect. IP bloquée pour 5 minutes.".to_string()
        }
        6 => {
            state.banned_until = Some(Utc::now() + Duration::hours(1));
            "Mot de passe incorrect. IP bloquée pour 1 heure.".to_string()
        }
        _ => {
            state.is_permabanned = true;
            "Mot de passe incorrect. IP BANNIE DE FAÇON PERMANENTE !".to_string()
        }
    }
}

pub fn record_successful_login(ip: &str) {
    let mut logins = FAILED_LOGINS.write().unwrap();
    if let Some(map) = &mut *logins {
        map.remove(ip);
    }
}

pub fn generate_session_token() -> String {
    let token = uuid::Uuid::new_v4().to_string();
    let mut sessions = VALID_SESSIONS.write().unwrap();
    let map = sessions.get_or_insert_with(StdHashMap::new);
    map.insert(token.clone(), Utc::now() + Duration::hours(2));
    token
}

pub fn is_session_valid(token: &str) -> bool {
    let sessions = VALID_SESSIONS.read().unwrap();
    if let Some(map) = &*sessions {
        if let Some(expiry) = map.get(token) {
            return Utc::now() < *expiry;
        }
    }
    false
}

#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>,
    pub error: Option<String>,
}

pub async fn admin_login(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let client_ip = headers
        .get("CF-Connecting-IP")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| addr.ip().to_string());

    // 1. Check if IP is banned
    if let Err(ban_msg) = check_login_ban(&client_ip) {
        return Json(LoginResponse {
            success: false,
            token: None,
            error: Some(ban_msg),
        }).into_response();
    }

    let required_pwd = std::env::var("ADMIN_PASSWORD")
        .or_else(|_| std::env::var("ADMIN_TOKEN"))
        .unwrap_or_else(|_| "admin123".to_string());

    if payload.password == required_pwd {
        record_successful_login(&client_ip);
        let token = generate_session_token();
        Json(LoginResponse {
            success: true,
            token: Some(token),
            error: None,
        }).into_response()
    } else {
        let err_msg = record_failed_login(&client_ip);
        Json(LoginResponse {
            success: false,
            token: None,
            error: Some(err_msg),
        }).into_response()
    }
}

#[derive(serde::Deserialize)]
pub struct GetJsonQuery {
    pub file: String, // "fail_messages", "fish_data", "junk_data"
    pub lang: String, // "fr" or "en"
}

pub async fn get_admin_json(
    headers: HeaderMap,
    Query(params): Query<GetJsonQuery>,
) -> impl IntoResponse {
    // 1. Authenticate session token
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or_default();
    let token = auth_header.trim_start_matches("Bearer ").trim();
    if !is_session_valid(token) {
        return (axum::http::StatusCode::UNAUTHORIZED, "Non autorisé").into_response();
    }

    // 2. Select file path
    let path = match (params.file.as_str(), params.lang.as_str()) {
        ("fail_messages", "en") => "data/fail_messages_en.json",
        ("fail_messages", _) => "data/fail_messages.json",
        ("fish_data", "en") => "data/fish_data_en.json",
        ("fish_data", _) => "data/fish_data.json",
        ("junk_data", "en") => "data/junk_data_en.json",
        ("junk_data", _) => "data/junk_data.json",
        _ => "data/fish_data.json",
    };

    let actual_path = if std::path::Path::new(path).exists() {
        path
    } else {
        // Fallback to non-en file if English file doesn't exist
        match params.file.as_str() {
            "fail_messages" => "data/fail_messages.json",
            "fish_data" => "data/fish_data.json",
            "junk_data" => "data/junk_data.json",
            _ => return (axum::http::StatusCode::BAD_REQUEST, "Fichier inconnu").into_response(),
        }
    };

    match tokio::fs::read_to_string(actual_path).await {
        Ok(content) => (axum::http::StatusCode::OK, content).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur de lecture: {}", e)).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct SaveJsonPayload {
    pub file: String, // "fail_messages", "fish_data", "junk_data"
    pub lang: String, // "fr" or "en"
    pub content: String,
}

pub async fn save_admin_json(
    headers: HeaderMap,
    Json(payload): Json<SaveJsonPayload>,
) -> impl IntoResponse {
    // 1. Authenticate session token
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or_default();
    let token = auth_header.trim_start_matches("Bearer ").trim();
    if !is_session_valid(token) {
        return (axum::http::StatusCode::UNAUTHORIZED, "Non autorisé").into_response();
    }

    // 2. Validate JSON syntax and structure
    match payload.file.as_str() {
        "fail_messages" => {
            if serde_json::from_str::<Vec<String>>(&payload.content).is_err() {
                return (axum::http::StatusCode::BAD_REQUEST, "Format JSON invalide (doit être un tableau de chaînes de caractères)").into_response();
            }
        }
        "fish_data" | "junk_data" => {
            if serde_json::from_str::<HashMap<crate::config::Rarity, Vec<crate::config::FishData>>>(&payload.content).is_err() {
                return (axum::http::StatusCode::BAD_REQUEST, "Format JSON invalide (structure du catalogue de poissons incorrecte)").into_response();
            }
        }
        _ => return (axum::http::StatusCode::BAD_REQUEST, "Fichier inconnu").into_response(),
    }

    // 3. File path selection
    let path = match (payload.file.as_str(), payload.lang.as_str()) {
        ("fail_messages", "en") => "data/fail_messages_en.json",
        ("fail_messages", _) => "data/fail_messages.json",
        ("fish_data", "en") => "data/fish_data_en.json",
        ("fish_data", _) => "data/fish_data.json",
        ("junk_data", "en") => "data/junk_data_en.json",
        ("junk_data", _) => "data/junk_data.json",
        _ => "data/fish_data.json",
    };

    // Ensure parent directory exists
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }

    // 4. Save to disk
    if let Err(e) = tokio::fs::write(path, &payload.content).await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Échec de l'écriture: {}", e)).into_response();
    }

    // 5. Trigger static hot-reload of game configuration!
    if let Err(reload_err) = crate::config::reload_all_game_data() {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Fichier enregistré mais échec du rechargement dynamique: {}", reload_err)).into_response();
    }

    (axum::http::StatusCode::OK, "Fichier sauvegardé et rechargé avec succès").into_response()
}
