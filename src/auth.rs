use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc, Duration};
use reqwest::Client;
use url::Url;

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
            .append_pair("scope", "chat:read chat:edit");
        url.to_string()
    }

    pub async fn exchange_code(&self, code: &str) -> Result<TwitchTokens, Box<dyn std::error::Error>> {
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

    pub async fn refresh_tokens(&self, refresh_token: &str) -> Result<TwitchTokens, Box<dyn std::error::Error>> {
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

    pub fn load_tokens(&self) -> Option<TwitchTokens> {
        if !Path::new(&self.token_path).exists() {
            return None;
        }
        let data = fs::read_to_string(&self.token_path).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn save_tokens(&self, tokens: &TwitchTokens) -> Result<(), Box<dyn std::error::Error>> {
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
