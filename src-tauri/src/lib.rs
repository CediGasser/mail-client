use std::collections::HashMap;

use auth_store::PersistedCredentials;
use tauri::async_runtime::Mutex;

use config::Config;
use email::Session;
use tauri::Manager;

mod auth;
mod auth_store;
mod commands;
mod config;
mod constants;
mod email;
mod error;
mod util;

// Global states:
// Mutex<AppState> - to manage the state of the accounts, including their credentials and IMAP sessions.
// Mutex<OAuthState> - to manage the OAuth Flow state, including pkce and csrf tokens
// Mutex<config::Config> - to manage the account configuration, including the list of accounts and their settings.

struct AccountState {
    credentials: auth_store::OAuthCredentials,
    imap_session: Option<Session>,
}

struct AppState {
    accounts: HashMap<String, AccountState>,
}

impl AccountState {
    async fn get_imap_session(&mut self) -> error::Result<&mut Session> {
        if self.imap_session.is_some() {
            // Test if the session is still valid
            match self.imap_session.as_mut().unwrap().noop() {
                Ok(_) => {
                    return Ok(self.imap_session.as_mut().unwrap());
                }
                Err(_) => {
                    println!("IMAP session invalid, creating a new one");
                    self.credentials.refresh().await?;
                    self.imap_session = email::get_imap_session(&self.credentials).ok();
                }
            }
            return Ok(self.imap_session.as_mut().unwrap());
        }

        if self.imap_session.is_none() {
            println!("No IMAP session found, creating a new one");

            self.credentials.refresh().await?;
            let imap_session = email::get_imap_session(&self.credentials)?;
            self.imap_session = Some(imap_session);
        }

        Ok(self.imap_session.as_mut().unwrap())
    }
}

impl AppState {
    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    fn get_account(&mut self, email: &str) -> Option<&mut AccountState> {
        if !self.accounts.contains_key(email) {
            let credentials = auth_store::OAuthCredentials::load(email)?;
            let account_state = AccountState {
                credentials,
                imap_session: None,
            };
            self.accounts.insert(email.to_string(), account_state);
        }

        Some(self.accounts.get_mut(email).unwrap())
    }

    pub fn set_account(
        &mut self,
        email: String,
        credentials: auth_store::OAuthCredentials,
    ) -> error::Result<()> {
        let account_state = AccountState {
            credentials,
            imap_session: None,
        };
        self.accounts.insert(email, account_state);
        Ok(())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    let app_state = AppState::new();

    builder
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(app_state))
        .setup(|app| {
            let config_path = app
                .path()
                .config_dir()
                .unwrap()
                .join(constants::CONFIG_FILE_NAME);
            println!("Config path: {:?}", config_path);
            let config = Config::load(config_path).expect("Failed to load account config");
            app.manage(Mutex::new(config));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::config_add_account,
            commands::config_remove_account,
            commands::login_with_google,
            commands::get_message,
            commands::get_envelopes,
            commands::send_email,
            commands::get_mailboxes,
            commands::remove_flags,
            commands::add_flags,
            commands::delete_message,
            commands::archive_message,
            commands::save_draft,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
