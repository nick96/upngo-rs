use crate::{error, resource, response, setter};
use log::*;
use serde::Deserialize;
use url::{form_urlencoded, Url};

pub struct WebhookClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    token: String,
}

impl WebhookClient {
    pub fn new(base_url: Url, token: String) -> Self {
        WebhookClient {
            client: reqwest::blocking::Client::new(),
            base_url,
            token,
        }
    }

    pub fn list(&self) -> ListRequestBuilder {
        ListRequestBuilder {
            client: &self.client,
            base_url: self.base_url.clone(),
            token: self.token.clone(),

            size: None,
        }
    }
}

pub struct ListRequestBuilder<'a> {
    client: &'a reqwest::blocking::Client,
    base_url: Url,
    token: String,

    size: Option<u32>,
}

impl<'a> ListRequestBuilder<'a> {
    setter!(size, u32);

    pub fn exec(&self) -> error::Result<response::Response<Vec<Webhook>>> {
        let url = self.base_url.clone();
        let mut query = vec![];
        if let Some(size) = self.size {
            let value: String =
                form_urlencoded::byte_serialize(size.to_string().as_bytes()).collect();
            query.push(("filter[size]", value))
        }
        debug!("Sending list webhook request to {}", url);
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .query(&query)
            .send()?
            .json::<response::Response<Vec<Webhook>>>()?;
        trace!("Webhook list webhook request responded with: {:?}", resp);
        Ok(resp)
    }
}

pub type Webhook = resource::Resource<Attributes, Relationships>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub url: String,
    pub description: Option<String>,
    pub secret_key: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    pub logs: LogRelationship,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogRelationship {
    pub logs: Option<RelatedLinks>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelatedLinks {
    pub related: Option<String>,
}

#[cfg(test)]
mod test {
    use super::Webhook;
    use crate::test_deserialization;
    use crate::response::SuccessfulResponse;

    test_deserialization!(test_de, "webhook.json", Webhook);
    test_deserialization!(test_multi_de, "webhook_list.json", Vec<Webhook>);
}