// This module interacts with the keyring to store and retrieve authentication tokens.
use crate::{auth::GmailOAuth2, error::Result};
use chrono::DateTime;
use keyring::Entry;

fn key(email: &str, suffix: &str) -> String {
    format!("mailclient:{}:{}", email, suffix)
}

pub fn store_token(auth: GmailOAuth2) -> Result<()> {
    Entry::new("mailclient", &key(auth.user(), "access_token"))?
        .set_password(auth.access_token())?;
    Entry::new("mailclient", &key(auth.user(), "expires_at"))?
        .set_password(auth.expires_at().timestamp().to_string().as_str())?;
    Entry::new("mailclient", &key(auth.user(), "refresh_token"))?
        .set_password(auth.refresh_token())?;
    Ok(())
}

pub fn load_token_for(user: &str) -> Option<GmailOAuth2> {
    let refresh = Entry::new("mailclient", &key(user, "refresh_token"))
        .ok()?
        .get_password()
        .ok()?;
    let access_token = Entry::new("mailclient", &key(user, "access_token"))
        .ok()
        .and_then(|e| e.get_password().ok())
        .expect("Access token not found");
    let expires_at = Entry::new("mailclient", &key(user, "expires_at"))
        .ok()
        .and_then(|e| e.get_password().ok())
        .and_then(|s| s.parse::<i64>().ok())
        .expect("Expires at not found");

    let gmail_oauth2 = GmailOAuth2 {
        user: user.to_string(),
        access_token: access_token.into(),
        expires_at: DateTime::from_timestamp(expires_at, 0).unwrap(),
        refresh_token: refresh.into(),
    };

    Some(gmail_oauth2)
}
