use serde::Deserialize;
use std::option::Option;
use thiserror::{Error};

#[derive(Deserialize, Debug)]
pub struct Source {
    parameter: Option<String>,
    pointer: Option<String>,
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.parameter, self.pointer)
    }
}

#[derive(Deserialize, Debug)]
pub struct ErrorObject {
    pub status: String,
    pub title: String,
    pub detail: String,
    pub source: Option<String>,
}

impl std::fmt::Display for ErrorObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} - {} - {} - {:?}",
            self.status, self.title, self.detail, self.source
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub errors: Vec<ErrorObject>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.errors {
            let res = writeln!(f, "- {}", error);
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ClientError {
    /// Error that occurs because of something I screwed up.
    #[error("The developer is a nuff-nuff: {0}")]
    InternalError(String),

    /// Error in the UpBank response (use or them).
    #[error("{0}")]
    UpBankError(Error),

    /// Error due to parsing or something like that.
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Error due to serialization.
    #[error("Serialization failed: {0}")]
    SerializationError(#[from] serde_json::error::Error),

    /// Error working with URLs.
    #[error("Failed to parse URL: {0}")]
    UrlError(#[from] url::ParseError),

    /// Conversion error.
    #[error("Failed to convert {value}: {reason}")]
    ConversionError {
        value: String,
        reason: String
    }
}

pub type Result<T> = std::result::Result<T, ClientError>;
