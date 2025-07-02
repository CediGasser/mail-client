use crate::error::{Error, Result};
use std::{collections::HashMap, net::TcpStream};
use tauri::utils::config::parse;
use utf7_imap::decode_utf7_imap;

use imap::types::NameAttribute;
use imap::Authenticator;
use mail_parser::{Address, MessageParser};
use native_tls::TlsStream;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{GOOGLE_IMAP_HOST, GOOGLE_IMAP_PORT};

pub type Session = imap::Session<TlsStream<TcpStream>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    pub name: Option<String>,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub uid: Option<u32>,
    pub date: Option<String>,
    pub from: Vec<EmailAddress>,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: Option<String>,
    pub headers: HashMap<String, String>,
    pub flags: Vec<String>,
    pub mailbox_name: String,
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
    pub uid: Option<u32>,
    pub date: Option<String>,
    pub from: Vec<EmailAddress>,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: Option<String>,
    pub headers: HashMap<String, String>,
    pub flags: Vec<String>,
    pub mailbox_name: String,
    pub body: String,
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
    let parser = MessageParser::default();

    let envelopes = responses
        .iter()
        .filter_map(|fetch| {
            let header_bytes = fetch.header()?;
            let message = parser.parse(header_bytes)?;
            let uid = fetch.uid;
            let date = message.date().map(|d| d.to_string());

            Some(Envelope {
                uid,
                date,
                from: parse_addrs(message.from()).unwrap_or_default(),
                to: parse_addrs(message.to()).unwrap_or_default(),
                cc: parse_addrs(message.cc()).unwrap_or_default(),
                bcc: parse_addrs(message.bcc()).unwrap_or_default(),
                subject: message.subject().map(|s| s.to_string()),
                headers: message
                    .headers()
                    .iter()
                    .map(|h| {
                        (
                            h.name().to_string(),
                            h.value().as_text().unwrap_or_default().to_string(),
                        )
                    })
                    .collect(),
                flags: fetch.flags().iter().map(|f| f.to_string()).collect(),
                mailbox_name: mailbox.to_string(),
            })
        })
        .collect();

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

    let message = response
        .first()
        .ok_or(Error::from("Could not get mail content"))?;

    let body = message
        .body()
        .ok_or(Error::from("Message is missing body"))?;

    let parser = MessageParser::new();

    let parsed = parser.parse(body).ok_or("Could not parse message")?;

    let html = parsed
        .body_html(0)
        .map(|s| s.to_string())
        .unwrap_or_default();

    Ok(Message {
        uid: message.uid,
        date: parsed.date().map(|d| d.to_string()),
        from: parse_addrs(parsed.from()).unwrap_or_default(),
        to: parse_addrs(parsed.to()).unwrap_or_default(),
        cc: parse_addrs(parsed.cc()).unwrap_or_default(),
        bcc: parse_addrs(parsed.bcc()).unwrap_or_default(),
        subject: parsed.subject().map(|s| s.to_string()),
        headers: parsed
            .headers()
            .iter()
            .map(|h| {
                (
                    h.name().to_string(),
                    h.value().as_text().unwrap_or_default().to_string(),
                )
            })
            .collect(),
        flags: message.flags().iter().map(|f| f.to_string()).collect(),
        mailbox_name: mailbox.to_string(),
        body: html,
    })
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

/// Save a draft message
///
/// # Arguments
/// * `imap_session` - The IMAP session
/// * `mailbox` - The mailbox to save the draft in
/// * `uid` - The UID of the draft to update, if any
/// * `subject` - The subject of the draft
/// * `body` - The body of the draft
/// # Returns
/// * `Result<u32>` - The new UID of the saved draft
///
pub fn save_draft(
    imap_session: &mut Session,
    mailbox: &str,
    uid: Option<u32>,
    subject: Option<&str>,
    body: Option<&str>,
    to: Option<Vec<EmailAddress>>,
    cc: Option<Vec<EmailAddress>>,
    bcc: Option<Vec<EmailAddress>>,
) -> Result<u32> {
    // Select the mailbox
    imap_session.select(mailbox)?;

    // If UID is provided, fetch and delete the existing draft
    if let Some(uid) = uid {
        let fetch_result = imap_session.uid_fetch(uid.to_string(), "RFC822")?;
        if fetch_result.is_empty() {
            return Err(Error::from("Draft not found"));
        }

        // Delete the existing draft
        imap_session.uid_store(uid.to_string(), "+FLAGS (\\Deleted)")?;
        imap_session.expunge()?;
    }

    // Generate a unique Message-ID
    let message_id = format!("<{}@mail-client>", Uuid::new_v4());

    // Create the raw email content
    let mut raw_email = format!(
        "Message-ID: {}\r\nSubject: {}\r\n",
        message_id,
        subject.unwrap_or("")
    );

    if let Some(to) = to {
        raw_email.push_str(&format!("To: {}\r\n", parse_addrs_to_string(to)));
    }
    if let Some(cc) = cc {
        raw_email.push_str(&format!("Cc: {}\r\n", parse_addrs_to_string(cc)));
    }
    if let Some(bcc) = bcc {
        raw_email.push_str(&format!("Bcc: {}\r\n", parse_addrs_to_string(bcc)));
    }

    raw_email.push_str(&format!("\r\n{}", body.unwrap_or("")));

    // Append the new draft to the mailbox
    imap_session.append(mailbox, raw_email.as_bytes())?;

    // Search for the newly created draft using the Message-ID
    imap_session.select(mailbox)?;
    let uids = imap_session.uid_search(format!("HEADER Message-ID {}", message_id))?;
    if let Some(new_uid) = uids.iter().max() {
        // Set the \Draft flag for the new draft
        imap_session.uid_store(new_uid.to_string(), "+FLAGS (\\Draft)")?;
        Ok(*new_uid)
    } else {
        Err(Error::from("Failed to retrieve UID of the new draft"))
    }
}

fn parse_addrs_to_string(addrs: Vec<EmailAddress>) -> String {
    addrs
        .into_iter()
        .map(|addr| {
            if let Some(name) = addr.name {
                format!("{} <{}>", name, addr.address)
            } else {
                addr.address
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn parse_addrs<'x>(addrs: Option<&Address<'x>>) -> Option<Vec<EmailAddress>> {
    let addr = addrs?;
    Some(
        addr.iter()
            .filter_map(|addr| {
                if let Some(email) = &addr.address {
                    Some(EmailAddress {
                        name: addr.name().map(|n| n.to_string()),
                        address: addr.address().map(|a| a.to_string()).unwrap_or_default(),
                    })
                } else {
                    None
                }
            })
            .collect(),
    )
}
