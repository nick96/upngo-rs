use crate::error;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Links {
    prev: Option<String>,
    next: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SuccessfulResponse<T> {
    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    data: T,
    links: Option<Links>,
}

#[derive(Debug)]
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

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let succ = SuccessfulResponse::deserialize(deserializer)?;
        Ok(Response::Ok(succ))
    }
}
