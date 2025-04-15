use crate::auth::{init_google_oauth_flow, GmailOAuth2};
use crate::error::{Error, ErrorKind, Result};
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
pub async fn get_mail_content(handle: tauri::AppHandle) -> Result<String> {
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
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Err(Error::new(
            ErrorKind::Generic("No messages found".to_string()),
            "No messages found",
        ));
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)?.to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(body)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Envelope {
    date: String,
    subject: String,
    from: String,
    to: String,
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
                to: envelope
                    .to
                    .as_ref()
                    .map(|t| {
                        t.iter()
                            .map(|a| {
                                from_utf8(a.name.unwrap_or_default())
                                    .unwrap_or_default()
                                    .to_string()
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default()
                    .join(", "),
            };
            envelopes.push(envelope_data);
        }
    }

    // Logout
    imap_session.logout()?;
    Ok(envelopes)
}
