mod auth;
mod commands;
mod constants;
mod error;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    builder
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::login_with_google,
            commands::get_gmail_oauth,
            commands::get_mail_content,
            commands::get_envelopes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
