use serde::Deserialize;
use crate::error;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Ok(T),
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
