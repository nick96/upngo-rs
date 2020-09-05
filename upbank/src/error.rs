use serde::Deserialize;
use std::option::Option;

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

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
