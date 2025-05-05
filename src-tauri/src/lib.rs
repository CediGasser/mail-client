use tauri::async_runtime::Mutex;

use config::AccountConfig;
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

struct AppState {
    imap_session: Option<Session>,
    auth: auth::GmailOAuth2,
}

impl AppState {
    async fn get_imap_session(&mut self) -> error::Result<&mut Session> {
        let auth = &self.auth;

        if self.imap_session.is_none() {
            println!("Creating new IMAP session");

            let imap_session = email::get_imap_session(auth)?;
            self.imap_session = Some(imap_session);
        } else {
            println!("Reusing existing IMAP session");
        }

        Ok(self.imap_session.as_mut().unwrap())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    builder
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::config_add_account,
            commands::config_remove_account,
            commands::login_with_google,
            commands::get_mail_content,
            commands::get_envelopes,
            commands::send_email,
            commands::get_mailboxes,
            commands::mark_flagged,
            commands::unmark_flagged,
            commands::mark_seen,
            commands::unmark_seen,
            commands::mark_deleted,
            commands::unmark_deleted,
            commands::mark_draft,
            commands::unmark_draft,
            commands::mark_answered,
            commands::unmark_answered,
        ])
        .setup(|app| {
            let config_path = app
                .path()
                .config_dir()
                .unwrap()
                .join(constants::CONFIG_FILE_NAME);
            let config = AccountConfig::load(config_path).expect("Failed to load account config");

            app.manage(Mutex::new(config));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
