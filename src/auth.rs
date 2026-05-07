use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc, Duration};
use reqwest::Client;
use url::Url;

pub type MyError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitchTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

pub struct AuthManager {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    token_path: String,
    streamer_token_path: String,
}

impl AuthManager {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            token_path: "data/tokens.json".to_string(),
            streamer_token_path: "data/streamer_tokens.json".to_string(),
        }
    }

    pub fn load_tokens(&self) -> Option<TwitchTokens> {
        let data = fs::read_to_string(&self.token_path).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn load_streamer_tokens(&self) -> Option<TwitchTokens> {
        let data = fs::read_to_string(&self.streamer_token_path).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn save_tokens(&self, tokens: &TwitchTokens) -> Result<(), MyError> {
        let data = serde_json::to_string_pretty(tokens)?;
        fs::write(&self.token_path, data)?;
        Ok(())
    }

    pub fn save_streamer_tokens(&self, tokens: &TwitchTokens) -> Result<(), MyError> {
        let data = serde_json::to_string_pretty(tokens)?;
        fs::write(&self.streamer_token_path, data)?;
        Ok(())
    }

    pub fn get_auth_url(&self, is_streamer: bool) -> String {
        let state = if is_streamer { "streamer" } else { "bot" };
        let scope = "chat:read chat:edit channel:manage:vips";
        
        // Construction manuelle pour éviter l'encodage des slashs/deux-points qui bloque Twitch
        format!(
            "https://id.twitch.tv/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.client_id,
            self.redirect_uri,
            scope.replace(" ", "+"),
            state
        )
    }

    pub async fn add_vip(&self, broadcaster_id: &str, user_id: &str, access_token: &str) -> bool {
        let client = Client::new();
        let url = format!("https://api.twitch.tv/helix/channels/vips?broadcaster_id={}&user_id={}", broadcaster_id, user_id);
        
        let res = client.post(url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await;

        match res {
            Ok(r) => {
                if r.status().is_success() || r.status().as_u16() == 422 {
                    tracing::info!("[API] VIP ajoute avec succes (ou deja present).");
                    true
                } else {
                    tracing::error!("[API] Erreur add_vip : Status {} - {}", r.status(), r.text().await.unwrap_or_default());
                    false
                }
            },
            Err(e) => {
                tracing::error!("[API] Erreur reseau add_vip : {}", e);
                false
            }
        }
    }

    pub async fn remove_vip(&self, access_token: &str, broadcaster_id: &str, user_id: &str) -> Result<(), MyError> {
        let client = Client::new();
        let url = format!("https://api.twitch.tv/helix/channels/vips?broadcaster_id={}&user_id={}", broadcaster_id, user_id);
        
        let res = client.delete(url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if res.status().is_success() || res.status().as_u16() == 422 || res.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(format!("Twitch API Error: {}", res.status()).into())
        }
    }

    pub async fn get_user_id(&self, access_token: &str, username: &str) -> Result<String, MyError> {
        let client = Client::new();
        let url = format!("https://api.twitch.tv/helix/users?login={}", username);
        
        let res = client.get(url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        res["data"][0]["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "User not found".into())
    }

    pub async fn get_stream_info(&self, channel_login: &str, access_token: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("https://api.twitch.tv/helix/streams?user_login={}", channel_login);
        
        let res = client.get(url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .ok()?
            .json::<serde_json::Value>()
            .await
            .ok()?;

        res["data"][0]["title"].as_str().map(|s| s.to_string())
    }

    pub async fn exchange_code(&self, code: &str) -> Result<TwitchTokens, MyError> {
        let client = Client::new();
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];

        let res = client.post("https://id.twitch.tv/oauth2/token")
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(TwitchTokens {
            access_token: res.access_token,
            refresh_token: res.refresh_token,
            expires_at: Utc::now() + Duration::seconds(res.expires_in),
        })
    }

    pub async fn refresh_tokens(&self, refresh_token: &str) -> Result<TwitchTokens, MyError> {
        let client = Client::new();
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];

        let res = client.post("https://id.twitch.tv/oauth2/token")
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(TwitchTokens {
            access_token: res.access_token,
            refresh_token: res.refresh_token,
            expires_at: Utc::now() + Duration::seconds(res.expires_in),
        })
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}
