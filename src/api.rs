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
    let now = Utc::now();
    
    let mut entry = state.rate_limiter.entry(ip.to_string()).or_insert((0, None));
    
    // 1. Verifier si l'IP est actuellement bannie
    if let Some(ban_time) = entry.value().1 {
        if now.signed_duration_since(ban_time).num_minutes() < 15 {
            return false; // Toujours banni
        } else {
            entry.value_mut().1 = None; // Fin du ban
            entry.value_mut().0 = 0;
        }
    }
    
    // 2. Incremeter les tentatives
    entry.value_mut().0 += 1;
    
    // 3. Bannir si trop de tentatives (ex: 5 essais infructueux)
    if entry.value().0 > 10 {
        entry.value_mut().1 = Some(now);
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

    let html = match tokio::fs::read_to_string("static/admin.html").await {
        Ok(h) => h,
        Err(_) => "Erreur chargement admin.html".to_string()
    };

    let mut response = Html(html).into_response();
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("no-store, no-cache, must-revalidate, max-age=0"),
    );
    response.headers_mut().insert(
        axum::http::header::PRAGMA,
        axum::http::HeaderValue::from_static("no-cache"),
    );
    response
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
            let player_id = match p.id {
                Some(id) => id,
                None => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Player ID missing").into_response(),
            };
            
            let catches = state.repo.get_player_catches(player_id).await.unwrap_or_default();
            let trophies = state.repo.get_player_trophies(player_id).await.unwrap_or_default();
            let museum = state.repo.get_player_museum(player_id).await.unwrap_or_default();
            
            let mut profile_image_url = p.profile_image_url.clone();
            if profile_image_url.is_none() {
                let mut fetched = false;
                
                // 1. Essayer avec les jetons du bot
                if let Some(tokens) = state.auth.load_tokens() {
                    if let Ok(img) = state.auth.get_user_profile_image(&tokens.access_token, &p.username).await {
                        let _ = state.repo.update_player_profile_image(player_id, &img).await;
                        profile_image_url = Some(img);
                        fetched = true;
                    }
                }
                
                // 2. Repli sur les jetons du streamer si échec/absent
                if !fetched {
                    if let Some(tokens) = state.auth.load_streamer_tokens() {
                        if let Ok(img) = state.auth.get_user_profile_image(&tokens.access_token, &p.username).await {
                            let _ = state.repo.update_player_profile_image(player_id, &img).await;
                            profile_image_url = Some(img);
                        }
                    }
                }
            }
            let is_banana_king = state.repo.is_active_king(player_id).await.unwrap_or(false);
            let has_banana_1 = state.repo.has_caught_banana(player_id, "Pristine Banana 1").await.unwrap_or(false);
            let has_banana_2 = state.repo.has_caught_banana(player_id, "Pristine Banana 2").await.unwrap_or(false);

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
                "is_first_millionaire": p.is_first_millionaire,
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
                "max_gold_held": p.max_gold_held,
                "eco_notoriety": p.eco_notoriety,
                "scrap_metal": p.scrap_metal,
                "total_sold_scrap_metal": p.total_sold_scrap_metal,
                "language": p.language
            })).into_response()
        },
        _ => Json(serde_json::json!({"error": "Player not found"})).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct LangQuery {
    pub lang: Option<String>,
}

pub async fn get_fish_data_api(Query(params): Query<LangQuery>) -> impl IntoResponse {
    let use_english = params.lang.as_deref() == Some("en");
    let data = if use_english {
        crate::config::get_game_data_en().fish_data.clone()
    } else {
        crate::config::get_game_data_fr().fish_data.clone()
    };
    Json(data).into_response()
}

pub async fn get_junk_data_api(Query(params): Query<LangQuery>) -> impl IntoResponse {
    let use_english = params.lang.as_deref() == Some("en");
    let data = if use_english {
        crate::config::get_game_data_en().junk_data.clone()
    } else {
        crate::config::get_game_data_fr().junk_data.clone()
    };
    Json(data).into_response()
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

pub async fn get_eco_champions(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.repo.get_eco_champions_history().await {
        Ok(history) => Json(serde_json::json!({ "history": history })).into_response(),
        Err(_) => Json(serde_json::json!({ "error": "Error fetching eco champions history" })).into_response()
    }
}

pub async fn get_global_museum(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.repo.get_global_museum().await {
        Ok(discoveries) => Json(serde_json::json!({ "museum": discoveries })).into_response(),
        Err(_) => Json(serde_json::json!({ "error": "Error fetching global museum" })).into_response()
    }
}

pub async fn get_top_eco(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.repo.get_top_eco_player().await {
        Ok(Some(player)) => Json(serde_json::json!({
            "username": player.username,
            "eco_notoriety": player.eco_notoriety,
            "level": player.level
        })).into_response(),
        Ok(None) => Json(serde_json::json!({ "error": "No players found" })).into_response(),
        Err(_) => Json(serde_json::json!({ "error": "Error" })).into_response()
    }
}

pub async fn get_top_banana(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.repo.get_active_banana_king_details().await {
        Ok(Some(player)) => Json(serde_json::json!({
            "username": player.username,
            "level": player.level
        })).into_response(),
        Ok(None) => Json(serde_json::json!({ "error": "No active Banana King" })).into_response(),
        Err(_) => Json(serde_json::json!({ "error": "Error" })).into_response()
    }
}

pub async fn trigger_maintenance(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let channel = state.channel.clone();
    let client_opt = {
        let lock = state.twitch_client.read().await;
        lock.clone()
    };
    if let Some(client) = client_opt {
        tracing::info!("⚠️ Notification de maintenance forcée envoyée à #{}", channel);
        let msg = "⚠️ Le bot est en cours de redémarrage/mise à jour. Merci de patienter avant de relancer vos commandes ! ⏳".to_string();
        let _ = client.say(channel.to_lowercase(), msg).await;
        (axum::http::StatusCode::OK, "✅ Message de maintenance envoyé avec succès sur Twitch !").into_response()
    } else {
        (axum::http::StatusCode::SERVICE_UNAVAILABLE, "❌ Le client Twitch n'est pas initialisé.").into_response()
    }
}



use std::sync::OnceLock;
use dashmap::DashMap;
use chrono::{DateTime, Duration};

#[derive(Clone, Debug)]
pub struct FailedLoginState {
    pub attempt_count: u32,
    pub banned_until: Option<DateTime<Utc>>,
    pub is_permabanned: bool,
}

static FAILED_LOGINS: OnceLock<DashMap<String, FailedLoginState>> = OnceLock::new();
static VALID_SESSIONS: OnceLock<DashMap<String, DateTime<Utc>>> = OnceLock::new();

fn get_failed_logins() -> &'static DashMap<String, FailedLoginState> {
    FAILED_LOGINS.get_or_init(DashMap::new)
}

fn get_valid_sessions() -> &'static DashMap<String, DateTime<Utc>> {
    VALID_SESSIONS.get_or_init(DashMap::new)
}

pub fn check_login_ban(ip: &str) -> Result<(), String> {
    let logins = get_failed_logins();
    if let Some(state) = logins.get(ip) {
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
    Ok(())
}

pub fn record_failed_login(ip: &str) -> String {
    let logins = get_failed_logins();
    let mut entry = logins.entry(ip.to_string()).or_insert(FailedLoginState {
        attempt_count: 0,
        banned_until: None,
        is_permabanned: false,
    });
    
    let state = entry.value_mut();
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
    get_failed_logins().remove(ip);
}

pub fn generate_session_token() -> String {
    let token = uuid::Uuid::new_v4().to_string();
    get_valid_sessions().insert(token.clone(), Utc::now() + Duration::hours(2));
    token
}

pub fn is_session_valid(token: &str) -> bool {
    let sessions = get_valid_sessions();
    if let Some(expiry) = sessions.get(token) {
        return Utc::now() < *expiry.value();
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
        ("cf_disappear_messages", "en") => "data/cf_disappear_messages_en.json",
        ("cf_disappear_messages", _) => "data/cf_disappear_messages.json",
        ("cf_edge_messages", "en") => "data/cf_edge_messages_en.json",
        ("cf_edge_messages", _) => "data/cf_edge_messages.json",
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
            "cf_disappear_messages" => "data/cf_disappear_messages.json",
            "cf_edge_messages" => "data/cf_edge_messages.json",
            _ => return (axum::http::StatusCode::BAD_REQUEST, "Fichier inconnu").into_response(),
        }
    };

    match tokio::fs::read_to_string(actual_path).await {
        Ok(content) => (axum::http::StatusCode::OK, content).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Erreur de lecture: {}", e)).into_response(),
    }
}

async fn sync_lang_parameters(file_type: &str, saved_lang: &str, saved_content: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (target_path, src_val) = match saved_lang {
        "en" => {
            let target = match file_type {
                "fish_data" => "data/fish_data.json",
                "junk_data" => "data/junk_data.json",
                "fail_messages" => "data/fail_messages.json",
                _ => return Ok(()),
            };
            (target, serde_json::from_str::<serde_json::Value>(saved_content)?)
        }
        _ => {
            let target = match file_type {
                "fish_data" => "data/fish_data_en.json",
                "junk_data" => "data/junk_data_en.json",
                "fail_messages" => "data/fail_messages_en.json",
                _ => return Ok(()),
            };
            (target, serde_json::from_str::<serde_json::Value>(saved_content)?)
        }
    };

    if !std::path::Path::new(target_path).exists() {
        tokio::fs::write(target_path, saved_content).await?;
        return Ok(());
    }

    let target_content = tokio::fs::read_to_string(target_path).await?;
    let mut target_val: serde_json::Value = serde_json::from_str(&target_content)?;

    match file_type {
        "fish_data" | "junk_data" => {
            if let (Some(fr_obj), Some(en_obj)) = (src_val.as_object(), target_val.as_object_mut()) {
                for (rarity, fr_list_val) in fr_obj {
                    if let Some(fr_list) = fr_list_val.as_array() {
                        let mut new_en_list = Vec::new();
                        let empty_vec = Vec::new();
                        let old_en_list = en_obj.get(rarity)
                            .and_then(|v| v.as_array())
                            .unwrap_or(&empty_vec);

                        for fr_item in fr_list {
                            let fr_id = fr_item.get("id").and_then(|v| v.as_i64());
                            let old_en_item = if let Some(target_id) = fr_id {
                                old_en_list.iter().find(|item| {
                                    item.get("id").and_then(|v| v.as_i64()) == Some(target_id)
                                })
                            } else {
                                None
                            };

                            let mut new_en_item = fr_item.clone();
                            if let Some(new_en_obj) = new_en_item.as_object_mut() {
                                if let Some(old_item) = old_en_item {
                                    if let Some(en_item_obj) = old_item.as_object() {
                                        for text_field in &["name", "description", "descriptions", "fun_fact", "location", "preferred_time", "preferred_season"] {
                                            if let Some(val) = en_item_obj.get(*text_field) {
                                                new_en_obj.insert(text_field.to_string(), val.clone());
                                            }
                                        }
                                    }
                                }
                            }
                            new_en_list.push(new_en_item);
                        }
                        en_obj.insert(rarity.clone(), serde_json::Value::Array(new_en_list));
                    }
                }
            }
        }
        "fail_messages" => {
            if let (Some(fr_arr), Some(en_arr)) = (src_val.as_array(), target_val.as_array_mut()) {
                let mut new_en_arr = Vec::new();
                for (i, fr_item) in fr_arr.iter().enumerate() {
                    let old_en_item = en_arr.get(i);
                    let mut new_en_item = fr_item.clone();
                    
                    if let Some(fr_obj) = fr_item.as_object() {
                        let gold_p = fr_obj.get("gold_penalty").cloned();
                        let cooldown_p = fr_obj.get("cooldown_penalty").cloned();
                        
                        let text_val = if let Some(old_item) = old_en_item {
                            if let Some(old_obj) = old_item.as_object() {
                                old_obj.get("text").cloned()
                            } else {
                                Some(old_item.clone())
                            }
                        } else {
                            fr_obj.get("text").cloned()
                        };

                        if let Some(t) = text_val {
                            let mut map = serde_json::Map::new();
                            map.insert("text".to_string(), t);
                            if let Some(gp) = gold_p { map.insert("gold_penalty".to_string(), gp); }
                            if let Some(cp) = cooldown_p { map.insert("cooldown_penalty".to_string(), cp); }
                            new_en_item = serde_json::Value::Object(map);
                        }
                    } else {
                        if let Some(old_item) = old_en_item {
                            if let Some(old_obj) = old_item.as_object() {
                                if let Some(t) = old_obj.get("text") {
                                    new_en_item = t.clone();
                                }
                            } else {
                                new_en_item = old_item.clone();
                            }
                        }
                    }
                    new_en_arr.push(new_en_item);
                }
                target_val = serde_json::Value::Array(new_en_arr);
            }
        }
        _ => {}
    }

    let updated_target_content = serde_json::to_string_pretty(&target_val)?;
    tokio::fs::write(target_path, updated_target_content).await?;
    Ok(())
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
            if let Err(err) = serde_json::from_str::<Vec<crate::config::FailMessageEntry>>(&payload.content) {
                return (axum::http::StatusCode::BAD_REQUEST, format!("Format JSON invalide (doit être un tableau de messages d'échecs valides) : {}", err)).into_response();
            }
        }
        "fish_data" | "junk_data" => {
            if let Err(err) = serde_json::from_str::<HashMap<crate::config::Rarity, Vec<crate::config::FishData>>>(&payload.content) {
                return (axum::http::StatusCode::BAD_REQUEST, format!("Format JSON invalide (structure du catalogue de poissons incorrecte) : {}", err)).into_response();
            }
        }
        "cf_disappear_messages" | "cf_edge_messages" => {
            if let Err(err) = serde_json::from_str::<Vec<String>>(&payload.content) {
                return (axum::http::StatusCode::BAD_REQUEST, format!("Format JSON invalide (doit être un tableau de chaînes) : {}", err)).into_response();
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
        ("cf_disappear_messages", "en") => "data/cf_disappear_messages_en.json",
        ("cf_disappear_messages", _) => "data/cf_disappear_messages.json",
        ("cf_edge_messages", "en") => "data/cf_edge_messages_en.json",
        ("cf_edge_messages", _) => "data/cf_edge_messages.json",
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

    // Synchroniser automatiquement les paramètres de jeu avec l'autre fichier de langue
    if let Err(sync_err) = sync_lang_parameters(&payload.file, &payload.lang, &payload.content).await {
        tracing::error!("Failed to sync language file parameters during save: {}", sync_err);
    }

    // 5. Trigger static hot-reload of game configuration!
    if let Err(reload_err) = crate::config::reload_all_game_data() {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Fichier enregistré mais échec du rechargement dynamique: {}", reload_err)).into_response();
    }

    (axum::http::StatusCode::OK, "Fichier sauvegardé et rechargé avec succès").into_response()
}

use axum::response::sse::{Event, KeepAlive, Sse};
use axum::extract::Multipart;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;

pub async fn obs_alerts(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let required_token = std::env::var("OBS_TOKEN")
        .or_else(|_| std::env::var("ADMIN_TOKEN"))
        .unwrap_or_else(|_| "change-me".to_string());

    if params.get("token") != Some(&required_token) {
        return (axum::http::StatusCode::FORBIDDEN, "Non autorisé").into_response();
    }

    match tokio::fs::read_to_string("static/obs.html").await {
        Ok(html) => Html(html).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Erreur chargement obs.html").into_response(),
    }
}

pub async fn obs_stream(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let required_token = std::env::var("OBS_TOKEN")
        .or_else(|_| std::env::var("ADMIN_TOKEN"))
        .unwrap_or_else(|_| "change-me".to_string());

    if params.get("token") != Some(&required_token) {
        return (axum::http::StatusCode::FORBIDDEN, "Non autorisé").into_response();
    }

    let rx = state.obs_broadcaster.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|res| match res {
            Ok(username) => {
                let event = Event::default().data(serde_json::json!({
                    "type": "banana",
                    "username": username,
                }).to_string());
                Some(Ok::<Event, std::convert::Infallible>(event))
            }
            Err(_) => None, // Ignore lags
        });

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

pub async fn list_sounds(
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or_default();
    let token = auth_header.trim_start_matches("Bearer ").trim();
    if !is_session_valid(token) {
        return (axum::http::StatusCode::UNAUTHORIZED, "Non autorisé").into_response();
    }

    let mut files = Vec::new();
    if let Ok(mut entries) = tokio::fs::read_dir("static/sounds").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".mp3") {
                    files.push(name.to_string());
                }
            }
        }
    }
    files.sort();
    Json(files).into_response()
}

pub async fn upload_sound(
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or_default();
    let token = auth_header.trim_start_matches("Bearer ").trim();
    if !is_session_valid(token) {
        return (axum::http::StatusCode::UNAUTHORIZED, "Non autorisé").into_response();
    }

    let target_username = params.get("username").cloned();
    let mut file_saved = false;
    let mut error_msg = String::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or_default().to_string();
        let file_name = field.file_name().unwrap_or_default().to_string();

        if name == "file" && file_name.ends_with(".mp3") {
            let base_name = if let Some(ref user) = target_username {
                user.clone()
            } else {
                file_name.strip_suffix(".mp3").unwrap_or(&file_name).to_string()
            };

            // Limit file name to alphanumeric/lowercase characters for safety
            let sanitized_base: String = base_name.chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                .collect();

            if sanitized_base.is_empty() {
                error_msg = "Nom de fichier ou d'utilisateur invalide".to_string();
                break;
            }

            let sanitized_name = format!("{}.mp3", sanitized_base.to_lowercase());

            if let Ok(data) = field.bytes().await {
                // Limit file size (5MB)
                if data.len() > 5 * 1024 * 1024 {
                    error_msg = "Fichier trop lourd (maximum 5 Mo)".to_string();
                    break;
                }

                let path = format!("static/sounds/{}", sanitized_name);
                if let Err(e) = tokio::fs::write(&path, data).await {
                    error_msg = format!("Échec de l'écriture sur le disque : {}", e);
                    break;
                }
                file_saved = true;
            } else {
                error_msg = "Échec de lecture des données du fichier".to_string();
                break;
            }
        }
    }

    if file_saved {
        (axum::http::StatusCode::OK, "Fichier MP3 téléchargé avec succès").into_response()
    } else {
        if error_msg.is_empty() {
            error_msg = "Aucun fichier valide trouvé".to_string();
        }
        (axum::http::StatusCode::BAD_REQUEST, error_msg).into_response()
    }
}

pub async fn list_players(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or_default();
    let token = auth_header.trim_start_matches("Bearer ").trim();
    if !is_session_valid(token) {
        return (axum::http::StatusCode::UNAUTHORIZED, "Non autorisé").into_response();
    }

    match state.repo.get_all_players().await {
        Ok(mut players) => {
            // Sort by total_attempts descending
            players.sort_by(|a, b| b.total_attempts.cmp(&a.total_attempts));
            
            let result: Vec<serde_json::Value> = players.into_iter().map(|p| {
                serde_json::json!({
                    "username": p.username,
                    "total_attempts": p.total_attempts
                })
            }).collect();
            
            Json(result).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get all players: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to get players").into_response()
        }
    }
}

