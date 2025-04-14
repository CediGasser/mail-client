use std::{
    error::{self, Error as StdError},
    fmt,
    io::Error as IoError,
    result,
};

use oauth2::reqwest::Error as ReqwestError;
use oauth2::{url::ParseError as UrlParseError, ErrorResponse, RequestTokenError};
use serde::{ser::SerializeStruct, Serialize};
use serde_json::Error as JsonError;

#[derive(Debug)]
pub struct Error {
    message: String,
    kind: ErrorKind,
}

impl Error {
    pub fn new<S: Into<String>>(kind: ErrorKind, msg: S) -> Self {
        Self {
            message: msg.into(),
            kind,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl From<ReqwestError> for Error {
    fn from(reqwest_error: ReqwestError) -> Self {
        Error::new(
            ErrorKind::Reqwest(reqwest_error),
            "Failed to make a request",
        )
    }
}

impl<RE: fmt::Debug + StdError + 'static, T: fmt::Debug + ErrorResponse + 'static>
    From<RequestTokenError<RE, T>> for Error
{
    fn from(request_token_error: RequestTokenError<RE, T>) -> Self {
        Error::new(
            ErrorKind::RequestTokenError,
            format!("Failed to parse redirect URL: {:?}", request_token_error),
        )
    }
}

impl From<JsonError> for Error {
    fn from(json_error: JsonError) -> Self {
        Error::new(
            ErrorKind::Json(json_error),
            "Failed to serialize/deserialize json data",
        )
    }
}

impl From<IoError> for Error {
    fn from(io_error: IoError) -> Self {
        Error::new(ErrorKind::Io(io_error), "IO error")
    }
}

impl From<UrlParseError> for Error {
    fn from(url_parse_error: UrlParseError) -> Self {
        Error::new(ErrorKind::UrlParse(url_parse_error), "Failed to parse URL")
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let source = self.source().unwrap_or(&self);
        let mut state = serializer.serialize_struct("Error", 2)?;

        state.serialize_field("message", &source.to_string())?;
        state.serialize_field("kind", "MailError")?;
        state.end()
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.kind() {
            ErrorKind::Io(e) => Some(e),
            ErrorKind::Reqwest(e) => Some(e),
            ErrorKind::Json(e) => Some(e),
            ErrorKind::UrlParse(e) => Some(e),
            _ => None,
        }
    }

    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(IoError),
    Reqwest(ReqwestError),
    Json(JsonError),
    UrlParse(UrlParseError),
    RequestTokenError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type Result<T> = result::Result<T, Error>;
