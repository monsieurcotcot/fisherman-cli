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
}

impl AuthManager {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            token_path: "data/tokens.json".to_string(),
        }
    }

    pub fn get_auth_url(&self) -> String {
        let mut url = Url::parse("https://id.twitch.tv/oauth2/authorize").unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", "chat:read chat:edit channel:manage:vips");
        url.to_string()
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
            Ok(r) => r.status().is_success() || r.status().as_u16() == 422, // 422 = deja VIP
            Err(_) => false,
        }
    }

    pub async fn remove_vip(&self, broadcaster_id: &str, user_id: &str, access_token: &str) -> bool {
        let client = Client::new();
        let url = format!("https://api.twitch.tv/helix/channels/vips?broadcaster_id={}&user_id={}", broadcaster_id, user_id);
        
        let res = client.delete(url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await;

        match res {
            Ok(r) => r.status().is_success(),
            Err(_) => false,
        }
    }

    pub async fn get_user_id(&self, username: &str, access_token: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("https://api.twitch.tv/helix/users?login={}", username);
        
        let res = client.get(url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .ok()?
            .json::<serde_json::Value>()
            .await
            .ok()?;

        res["data"][0]["id"].as_str().map(|s| s.to_string())
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

        let tokens = TwitchTokens {
            access_token: res.access_token,
            refresh_token: res.refresh_token,
            expires_at: Utc::now() + Duration::seconds(res.expires_in),
        };

        self.save_tokens(&tokens)?;
        Ok(tokens)
    }

    pub async fn refresh_tokens(&self, refresh_token: &str) -> Result<TwitchTokens, MyError> {
        let client = Client::new();
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];

        let res = client.post("https://id.twitch.tv/oauth2/token")
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        let tokens = TwitchTokens {
            access_token: res.access_token,
            refresh_token: res.refresh_token,
            expires_at: Utc::now() + Duration::seconds(res.expires_in),
        };

        self.save_tokens(&tokens)?;
        Ok(tokens)
    }

    pub async fn get_stream_info(&self, channel: &str, access_token: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("https://api.twitch.tv/helix/streams?user_login={}", channel);
        
        let res = client.get(url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .ok()?
            .json::<serde_json::Value>()
            .await
            .ok()?;

        // On extrait le titre du premier stream trouvé (si en ligne)
        res["data"][0]["title"].as_str().map(|s| s.to_string())
    }

    pub fn load_tokens(&self) -> Option<TwitchTokens> {
        if !Path::new(&self.token_path).exists() {
            return None;
        }
        let data = fs::read_to_string(&self.token_path).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn save_tokens(&self, tokens: &TwitchTokens) -> Result<(), MyError> {
        let data = serde_json::to_string_pretty(tokens)?;
        fs::write(&self.token_path, data)?;
        Ok(())
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}
