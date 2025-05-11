// This module interacts with the keyring to store and retrieve authentication tokens.
use crate::{auth, error::Result};
use chrono::{DateTime, Utc};
use keyring::Entry;
use lettre::transport::smtp::authentication::Credentials;
use oauth2::{RefreshToken, TokenResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct OAuthCredentials {
    access_token: String,
    expires_at: DateTime<Utc>,
    refresh_token: String,

    pub user: String,
    entry: Entry,
}

impl OAuthCredentials {
    pub fn new(
        access_token: String,
        expires_at: DateTime<Utc>,
        refresh_token: String,
        user: String,
    ) -> Self {
        let entry = Entry::new("mailclient", &user).unwrap();
        OAuthCredentials {
            access_token,
            expires_at,
            refresh_token,
            user,
            entry,
        }
    }
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub async fn refresh(&mut self) -> Result<()> {
        if !self.is_expired() {
            println!("Reusing existing access token");
            return Ok(());
        }

        println!("Refreshing access token");
        let http_client = reqwest::Client::new();
        let oauth_client = auth::create_client();
        let token = oauth_client
            .exchange_refresh_token(&RefreshToken::new(self.refresh_token.clone()))
            .request_async(&http_client)
            .await?;

        let new_access_token = token.access_token().secret().to_string();
        let expires_in = token.expires_in().unwrap_or_default();

        let new_expires_at = Utc::now() + expires_in;
        let new_refresh_token = token
            .refresh_token()
            .map(|rt| rt.secret().to_string())
            .unwrap_or_else(|| self.refresh_token.clone());

        self.access_token = new_access_token;
        self.expires_at = new_expires_at;
        self.refresh_token = new_refresh_token;

        self.persist()?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializedOAuthCredentials {
    access_token: String,
    expires_at: DateTime<Utc>,
    refresh_token: String,
}

impl PersistedCredentials for OAuthCredentials {
    fn load(key: &str) -> Option<Self> {
        let entry = Entry::new("mailclient", key).ok()?;

        let json_string = entry.get_password().ok()?;

        let credentials = serde_json::from_str::<SerializedOAuthCredentials>(&json_string).ok()?;
        let credentials = OAuthCredentials {
            access_token: credentials.access_token,
            expires_at: credentials.expires_at,
            refresh_token: credentials.refresh_token,
            user: key.to_string(),
            entry: entry,
        };

        Some(credentials)
    }

    fn persist(&self) -> Result<()> {
        let serializable_credentials = SerializedOAuthCredentials {
            access_token: self.access_token.clone(),
            expires_at: self.expires_at,
            refresh_token: self.refresh_token.clone(),
        };
        let json = serde_json::to_string(&serializable_credentials)?;
        self.entry.set_password(&json)?;
        Ok(())
    }

    fn delete(&self) -> Result<()> {
        self.entry.delete_credential()?;
        Ok(())
    }
}

impl imap::Authenticator for &OAuthCredentials {
    type Response = String;

    fn process(&self, _data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

impl Into<Credentials> for &OAuthCredentials {
    fn into(self) -> Credentials {
        Credentials::new(self.user.clone(), self.access_token.clone())
    }
}

pub trait PersistedCredentials {
    fn load(key: &str) -> Option<Self>
    where
        Self: Sized;
    fn persist(&self) -> Result<()>;
    fn delete(&self) -> Result<()>;
}
