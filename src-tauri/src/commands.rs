use crate::auth::{init_google_oauth_flow, GmailOAuth2};
use crate::error::{Error, ErrorKind, Result};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::{message::header::ContentType, Message, SmtpTransport, Transport};
use mail_parser::MessageParser;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;
use tauri::Manager;

#[tauri::command]
pub async fn login_with_google(handle: tauri::AppHandle) -> Result<()> {
    init_google_oauth_flow(handle).await
}

#[tauri::command]
pub async fn get_gmail_oauth(handle: tauri::AppHandle) -> Result<GmailOAuth2> {
    let auth = handle.state::<GmailOAuth2>();

    Ok(auth.inner().clone())
}

#[tauri::command]
pub async fn get_mail_content(handle: tauri::AppHandle, uid: u32) -> Result<String> {
    let auth = handle.state::<GmailOAuth2>();

    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .authenticate("XOAUTH2", auth.inner())
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.uid_fetch(uid.to_string(), "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Err(Error::new(
            ErrorKind::Generic("No messages found".to_string()),
            "No messages found",
        ));
    };

    // extract the message's body
    let body = message
        .body()
        .ok_or(Error::from("No body found".to_string()))?;
    let body = MessageParser::default()
        .parse(body)
        .ok_or(Error::from("Could not parse email message".to_string()))?;
    let body = body
        .body_html(0)
        .ok_or(Error::from(
            "Could not get text content of root part".to_string(),
        ))?
        .to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(body)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Envelope {
    uid: u32,
    date: String,
    subject: String,
    from: String,
    read: bool,
    starred: bool,
}

#[tauri::command]
pub fn get_envelopes(handle: tauri::AppHandle) -> Result<Vec<Envelope>> {
    let auth = handle.state::<GmailOAuth2>();

    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .authenticate("XOAUTH2", auth.inner())
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.uid_fetch("1:*", "ALL")?;
    let mut envelopes = Vec::new();
    for message in messages.iter() {
        if let Some(envelope) = message.envelope() {
            let envelope_data = Envelope {
                uid: message.uid.unwrap_or_default(),
                date: envelope
                    .date
                    .map(|s| from_utf8(s).unwrap())
                    .unwrap_or("(no subject)")
                    .to_string(),
                subject: envelope
                    .subject
                    .map(|s| from_utf8(s).unwrap())
                    .unwrap_or("(no subject)")
                    .to_string(),
                from: envelope
                    .from
                    .as_ref()
                    .map(|f| {
                        f.iter()
                            .map(|a| {
                                from_utf8(a.name.unwrap_or_default())
                                    .unwrap_or_default()
                                    .to_string()
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default()
                    .join(", "),
                read: message.flags().contains(&imap::types::Flag::Seen),
                starred: message.flags().contains(&imap::types::Flag::Flagged),
            };
            envelopes.push(envelope_data);
        }
    }

    // Logout
    imap_session.logout()?;
    Ok(envelopes)
}

#[tauri::command]
pub fn send_email(handle: tauri::AppHandle, to: &str, subject: &str, body: &str) -> Result<String> {
    let auth = handle.state::<GmailOAuth2>();
    let domain = "smtp.gmail.com";

    let email = Message::builder()
        .from(
            auth.inner()
                .user
                .parse()
                .expect("Failed to parse sender email"),
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
        .credentials(auth.inner().clone().into())
        .build();

    let result = mailer.send(&email)?;
    Ok(result.code().to_string())
}
