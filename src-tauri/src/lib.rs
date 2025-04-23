use email::Session;

mod auth;
mod auth_store;
mod commands;
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
            commands::get_user,
            commands::login_with_google,
            commands::get_mail_content,
            commands::get_envelopes,
            commands::send_email,
            commands::get_mailboxes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
