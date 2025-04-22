use crate::auth::init_google_oauth_flow;
use crate::constants::GOOGLE_SMTP_HOST;
use crate::email::{self, Envelope, Mailbox};
use crate::error::{Error, ErrorKind, Result};
use crate::AppState;
use lettre::transport::smtp::authentication::Mechanism;
use lettre::{message::header::ContentType, Message, SmtpTransport, Transport};
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[tauri::command]
pub async fn login_with_google(handle: tauri::AppHandle) -> Result<()> {
    init_google_oauth_flow(handle).await
}

#[tauri::command]
pub async fn get_user() -> Result<String> {
    let user = "".to_string();

    Ok(user)
}

#[tauri::command]
pub async fn get_mailboxes(handle: tauri::AppHandle) -> Result<Vec<Mailbox>> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let imap_session = app_state.get_imap_session().await?;

    let mailboxes = email::get_mailboxes(imap_session).unwrap();
    return Ok(mailboxes);
}

#[tauri::command]
pub async fn get_envelopes(handle: tauri::AppHandle, mailbox: &str) -> Result<Vec<Envelope>> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let imap_session = app_state.get_imap_session().await?;

    let envelopes = email::get_envelopes(imap_session, mailbox)?;
    return Ok(envelopes);
}

#[tauri::command]
pub async fn get_mail_content(handle: tauri::AppHandle, mailbox: &str, uid: u32) -> Result<String> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let imap_session = app_state.get_imap_session().await?;

    let html = email::get_mail_content(imap_session, mailbox, uid)?;
    return Ok(html);
}

#[tauri::command]
pub async fn send_email(
    handle: tauri::AppHandle,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<String> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let app_state = app_state_mutex.lock().await;
    let auth = &app_state.auth;
    let domain = GOOGLE_SMTP_HOST;

    let email = Message::builder()
        .from(auth.user().parse().expect("Failed to parse sender email"))
        .to(to
            .parse()
            .expect(format!("Failed to parse recipient email: {}", to).as_str()))
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .map_err(|e| Error::new(ErrorKind::Generic(e.to_string()), "Failed to create email"))?;

    let mailer = SmtpTransport::relay(domain)
        .expect("Failed to create SMTP transport")
        .authentication(vec![Mechanism::Xoauth2])
        .credentials(auth.into())
        .build();

    let result = mailer.send(&email)?;
    Ok(result.code().to_string())
}
