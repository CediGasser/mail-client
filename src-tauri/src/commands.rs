use tauri::Manager;

use crate::auth::{init_google_oauth_flow, GmailOAuth2};
use crate::error::Result;

#[tauri::command]
pub fn get_mail_config(domain: &str) -> String {
    format!("The configuration for {} should now be loaded.", domain)
}

#[tauri::command]
pub async fn login_with_google(handle: tauri::AppHandle) -> Result<()> {
    init_google_oauth_flow(handle).await
}

#[tauri::command]
pub async fn get_gmail_oauth(handle: tauri::AppHandle) -> Result<GmailOAuth2> {
    let auth = handle.state::<GmailOAuth2>();

    Ok(auth.inner().clone())
}
