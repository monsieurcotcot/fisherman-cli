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
    if params.get("token") == Some(&secret) {
        Html(match tokio::fs::read_to_string("static/admin.html").await { Ok(h) => h, Err(_) => "Erreur chargement admin.html".to_string() }).into_response()
    } else {
        (axum::http::StatusCode::FORBIDDEN, "Accès refusé").into_response()
    }
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

            Json(serde_json::json!({
                "username": p.username,
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
                "coinflip_gold_lost_total": p.coinflip_gold_lost_total
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
