use crate::error::{Error, Result};
use std::net::TcpStream;
use utf7_imap::decode_utf7_imap;

use imap::types::{Flag, NameAttribute};
use imap::Authenticator;
use mail_parser::MessageParser;
use native_tls::TlsStream;
use serde::{Deserialize, Serialize};

use crate::constants::{GOOGLE_IMAP_HOST, GOOGLE_IMAP_PORT};

pub type Session = imap::Session<TlsStream<TcpStream>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Envelope {
    uid: Option<u32>,
    date: Option<String>,
    from: Option<String>,
    subject: Option<String>,
    read: bool,
    starred: bool,
    mailbox_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mailbox {
    pub name: String,
    pub display_name: String,
    pub delimiter: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    body: String,
    uid: Option<u32>,
    date: Option<String>,
    from: Option<String>,
    subject: Option<String>,
    read: bool,
    starred: bool,
    mailbox_name: String,
}

/// Get the list of mailboxes
///
/// # Arguments
/// * `session` - The IMAP session
/// # Returns
/// * `Result<Vec<String>>` - The list of mailboxes
///
pub fn get_mailboxes(session: &mut Session) -> Result<Vec<Mailbox>> {
    let responses = session.list(None, Some("*"))?;

    let mailbox_names = responses
        .iter()
        .map(|mailbox| {
            let name = mailbox.name().to_string();
            let display_name = decode_utf7_imap(mailbox.name().to_string());
            let delimiter = mailbox.delimiter();
            let attributes = mailbox
                .attributes()
                .iter()
                .map(|a| match a {
                    NameAttribute::Marked => "Marked".to_string(),
                    NameAttribute::Unmarked => "Unmarked".to_string(),
                    NameAttribute::NoInferiors => "NoInferiors".to_string(),
                    NameAttribute::NoSelect => "NoSelect".to_string(),
                    NameAttribute::Custom(s) => s.to_string(),
                })
                .collect();

            Mailbox {
                name: name,
                display_name: display_name,
                delimiter: delimiter.unwrap_or_default().to_string(),
                attributes: attributes,
            }
        })
        .collect::<Vec<Mailbox>>();

    Ok(mailbox_names)
}

/// Get the list of envelopes
///
/// # Arguments
/// * `session` - The IMAP session
/// * `mailbox` - The mailbox to select
/// # Returns
/// * `Result<Vec<Envelope>>` - The list of envelopes
///
pub fn get_envelopes(session: &mut Session, mailbox: &str) -> Result<Vec<Envelope>> {
    session.select(mailbox)?;

    let responses = session.fetch("1:*", "(UID FLAGS RFC822.HEADER)")?;

    let parser = &MessageParser::new();

    let envelopes = responses
        .iter()
        .filter_map(|fetch| {
            let header_bytes = fetch.header()?;
            let message = parser.parse(header_bytes)?;
            let uid = fetch.uid;
            let date = message.date().map(|d| d.to_string());
            let from = message
                .from()
                .map(|f| f.first().unwrap().name().map(|n| n.to_string()))
                .flatten();
            let subject = message.subject().map(|s| s.to_string());
            let read = fetch.flags().contains(&Flag::Seen);
            let starred = fetch.flags().contains(&Flag::Flagged);
            let mailbox_name = mailbox.to_string();

            let envelope = Envelope {
                uid: uid,
                date,
                from,
                subject,
                read,
                starred,
                mailbox_name,
            };
            Some(envelope)
        })
        .collect::<Vec<Envelope>>();

    Ok(envelopes)
}

/// Get the content of a mail
///
/// # Arguments
/// * `session` - The IMAP session
/// * `mailbox` - The mailbox to select
/// * `uid` - The UID of the mail
/// # Returns
/// * `Result<String>` - The content of the mail as HTML
///
pub fn get_message(session: &mut Session, mailbox: &str, uid: u32) -> Result<Message> {
    session.select(mailbox)?;

    let response = session.uid_fetch(uid.to_string(), "(UID FLAGS RFC822)")?;

    let parser = &MessageParser::new();

    let response = response
        .first()
        .ok_or(Error::from("Could not get mail content".to_string()))?;

    let message_bytes = response
        .body()
        .ok_or(Error::from("Message is missing body"))?;

    let message = parser
        .parse(message_bytes)
        .ok_or(Error::from("Could not parse message"))?;

    let uid = response.uid;
    let date = message.date().map(|d| d.to_string());
    let from = message
        .from()
        .map(|f| f.first().unwrap().name().map(|n| n.to_string()))
        .flatten();
    let subject = message.subject().map(|s| s.to_string());
    let read = response.flags().contains(&Flag::Seen);
    let starred = response.flags().contains(&Flag::Flagged);
    let mailbox_name = mailbox.to_string();

    let body = response
        .body()
        .ok_or(Error::from("No body found".to_string()))?;

    let html = MessageParser::default()
        .parse(body)
        .ok_or(Error::from("Could not parse email message".to_string()))?
        .body_html(0)
        .ok_or(Error::from(
            "Could not get text content of root part".to_string(),
        ))?
        .to_string();

    let message = Message {
        body: html,
        uid,
        date,
        from,
        subject,
        read,
        starred,
        mailbox_name,
    };

    Ok(message)
}

/// Add flags to a message
///
/// # Arguments
/// * `session` - The IMAP session
/// * `mailbox` - The mailbox to select
/// * `uid` - The UID of the message
/// * `flags` - The flags to add
/// # Returns
/// * `Result<()>` - Ok if successful
///
pub fn add_flags(session: &mut Session, mailbox: &str, uid: u32, flags: Vec<&str>) -> Result<()> {
    session.select(mailbox)?;

    let flags = flags.join(" ");

    println!("Adding flags: {}", flags);

    session.uid_store(uid.to_string(), format!("+FLAGS ({})", flags))?;

    Ok(())
}

/// Remove flags from a message
///
/// # Arguments
/// * `session` - The IMAP session
/// * `mailbox` - The mailbox to select
/// * `uid` - The UID of the message
/// * `flags` - The flags to remove
/// # Returns
/// * `Result<()>` - Ok if successful
///
pub fn remove_flags(
    session: &mut Session,
    mailbox: &str,
    uid: u32,
    flags: Vec<&str>,
) -> Result<()> {
    session.select(mailbox)?;

    let flags = flags.join(" ");

    println!("Removing flags: {}", flags);

    session.uid_store(uid.to_string(), format!("-FLAGS ({})", flags))?;

    Ok(())
}

/// Move a message to another mailbox
///
/// # Arguments
/// * `session` - The IMAP session
/// * `mailbox` - The mailbox to select
/// * `uid` - The UID of the message
/// * `destination` - The destination mailbox
/// # Returns
/// * `Result<()>` - Ok if successful
///
pub fn move_mail(session: &mut Session, mailbox: &str, uid: u32, destination: &str) -> Result<()> {
    session.select(mailbox)?;

    session.uid_mv(uid.to_string(), destination)?;
    Ok(())
}

/// Get an authenticated IMAP session
///
/// # Arguments
/// * `auth` - The authenticator to use for authentication
/// # Returns
/// * `Result<Session>` - The IMAP session
///
pub fn get_imap_session(auth: impl Authenticator) -> Result<Session> {
    let domain = GOOGLE_IMAP_HOST;
    let port = GOOGLE_IMAP_PORT;

    let tls = native_tls::TlsConnector::new().unwrap();

    let client = imap::connect((domain, port), domain, &tls)?;

    let session = client.authenticate("XOAUTH2", &auth).map_err(|e| e.0)?;

    Ok(session)
}
