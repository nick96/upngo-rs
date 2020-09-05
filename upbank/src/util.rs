use crate::{error, response::Response};
use log::*;
use serde::Deserialize;
use url::Url;

pub struct Util {
    client: reqwest::blocking::Client,
    base_url: Url,
    token: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub id: String,
    pub status_emoji: String,
}

#[derive(Deserialize, Debug)]
pub struct Ping {
    pub meta: Meta,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PingResponse {
    Ok(Ping),
    Err(error::Error),
}

impl Util {
    pub fn new(base_url: Url, token: String) -> Self {
        Util {
            client: reqwest::blocking::Client::new(),
            base_url,
            token,
        }
    }

    pub fn ping(&self) -> error::Result<PingResponse> {
        let ping_url = self
            .base_url
            .join("ping")
            .expect("could not join 'ping' to base URL");
        debug!("Sending ping request to {}", ping_url.to_string(),);
        let resp = self
            .client
            .get(ping_url)
            .bearer_auth(&self.token)
            .send()?
            .json::<PingResponse>()?;
        trace!("Ping responded with {:?}", resp);
        Ok(resp)
    }
}
