use crate::auth::init_google_oauth_flow;
use crate::config::{Account, Config};
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
pub async fn get_config(handle: tauri::AppHandle) -> Result<Vec<Account>> {
    let account_config_mutex = handle
        .try_state::<Mutex<Config>>()
        .expect("Failed to get config state");
    let account_config = account_config_mutex.lock().await;

    let config = account_config.accounts().to_vec();

    Ok(config)
}

#[tauri::command]
pub async fn config_add_account(handle: tauri::AppHandle, email: &str) -> Result<()> {
    let account_config_mutex = handle.state::<Mutex<Config>>();
    let mut account_config = account_config_mutex.lock().await;

    account_config
        .add_account(email.to_string())
        .expect("Failed to add account");

    Ok(())
}

#[tauri::command]
pub async fn config_remove_account(handle: tauri::AppHandle, email: &str) -> Result<()> {
    let account_config_mutex = handle.state::<Mutex<Config>>();
    let mut account_config = account_config_mutex.lock().await;

    account_config
        .remove_account(email)
        .expect("Failed to remove account");

    Ok(())
}

#[tauri::command]
pub async fn get_mailboxes(handle: tauri::AppHandle, email: &str) -> Result<Vec<Mailbox>> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let account = app_state
        .get_account(email)
        .ok_or(Error::from("Account not found"))?;
    let imap_session = account.get_imap_session().await?;

    let mailboxes = email::get_mailboxes(imap_session).unwrap();
    return Ok(mailboxes);
}

#[tauri::command]
pub async fn get_envelopes(
    handle: tauri::AppHandle,
    email: &str,
    mailbox: &str,
) -> Result<Vec<Envelope>> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let account = app_state
        .get_account(email)
        .ok_or(Error::from("Account not found"))?;
    let imap_session = account.get_imap_session().await?;

    let envelopes = email::get_envelopes(imap_session, mailbox)?;
    return Ok(envelopes);
}

#[tauri::command]
pub async fn get_message(
    handle: tauri::AppHandle,
    email: &str,
    mailbox: &str,
    uid: u32,
) -> Result<email::Message> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let account = app_state
        .get_account(email)
        .ok_or(Error::from("Account not found"))?;
    let imap_session = account.get_imap_session().await?;

    let html = email::get_message(imap_session, mailbox, uid)?;
    return Ok(html);
}

#[tauri::command]
pub async fn add_flags(
    handle: tauri::AppHandle,
    email: &str,
    mailbox: &str,
    uid: u32,
    flags: Vec<&str>,
) -> Result<()> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let account = app_state
        .get_account(email)
        .ok_or(Error::from("Account not found"))?;
    let imap_session = account.get_imap_session().await?;

    email::add_flags(imap_session, mailbox, uid, flags)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_flags(
    handle: tauri::AppHandle,
    email: &str,
    mailbox: &str,
    uid: u32,
    flags: Vec<&str>,
) -> Result<()> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let account = app_state
        .get_account(email)
        .ok_or(Error::from("Account not found"))?;
    let imap_session = account.get_imap_session().await?;

    email::remove_flags(imap_session, mailbox, uid, flags)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_message(
    handle: tauri::AppHandle,
    email: &str,
    mailbox: &str,
    uid: u32,
) -> Result<()> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let account = app_state
        .get_account(email)
        .ok_or(Error::from("Account not found"))?;
    let imap_session = account.get_imap_session().await?;

    let mailboxes = email::get_mailboxes(imap_session).unwrap();
    let trash = mailboxes
        .iter()
        .find(|m| m.attributes.contains(&"\\Trash".to_string()));

    // Move to trash mailbox if it exists, otherwise add \Deleted flag
    if let Some(trash) = trash {
        if trash.name != mailbox {
            email::move_mail(imap_session, mailbox, uid, &trash.name)?;
            return Ok(());
        }
    }

    email::add_flags(imap_session, mailbox, uid, vec!["\\Deleted"])?;
    Ok(())
}

#[tauri::command]
pub async fn archive_message(
    handle: tauri::AppHandle,
    email: &str,
    mailbox: &str,
    uid: u32,
) -> Result<()> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let account = app_state
        .get_account(email)
        .ok_or(Error::from("Account not found"))?;
    let imap_session = account.get_imap_session().await?;

    let mailboxes = email::get_mailboxes(imap_session).unwrap();
    let archive = mailboxes
        .iter()
        .find(|m| m.attributes.contains(&"\\All".to_string()));

    // Move to archive mailbox if it exists, otherwise add \Deleted flag
    if let Some(archive) = archive {
        email::move_mail(imap_session, mailbox, uid, &archive.name)?;
    } else {
        return Err(Error::from("Archive mailbox not found"))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn send_email(
    handle: tauri::AppHandle,
    email: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<String> {
    let app_state_mutex = handle.state::<Mutex<AppState>>();
    let mut app_state = app_state_mutex.lock().await;
    let account = app_state
        .get_account(email)
        .ok_or(Error::from("Account not found"))?;
    let auth = &account.credentials;
    let domain = GOOGLE_SMTP_HOST;

    let message = Message::builder()
        .from(
            email
                .parse()
                .expect(format!("Failed to parse sender email: {}", email).as_str()),
        )
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

    let result = mailer.send(&message)?;
    Ok(result.code().to_string())
}
