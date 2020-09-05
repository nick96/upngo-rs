use serde::{Deserialize, Deserializer};
use crate::error;

#[derive(Deserialize, Debug)]
pub struct Links {
    prev: Option<String>,
    next: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SuccessfulResponse<T> {
    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    data: T,
    links: Option<Links>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Ok(SuccessfulResponse<T>),
    Err(error::Error),
}

impl<T> Response<T> {
    pub fn is_ok(&self) -> bool {
        match self {
            Response::Ok(_) => true,
            Response::Err(_) => false,
        }
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}
