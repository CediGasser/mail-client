// This module interacts with the keyring to store and retrieve authentication tokens.
use crate::{auth::GmailOAuth2, error::Result};
use keyring::Entry;

fn key(email: &str, suffix: &str) -> String {
    format!("mailclient:{}:{}", email, suffix)
}

pub fn store_token(auth: GmailOAuth2) -> Result<()> {
    let json = serde_json::to_string(&auth)?;
    Entry::new("mailclient", &key(auth.user(), "tokens"))?.set_password(&json)?;
    Ok(())
}

pub fn load_token_for(email: &str) -> Option<GmailOAuth2> {
    let refresh = Entry::new("mailclient", &key(email, "tokens"))
        .ok()?
        .get_password()
        .ok()?;

    let auth = serde_json::from_str::<GmailOAuth2>(&refresh).ok()?;

    Some(auth)
}
