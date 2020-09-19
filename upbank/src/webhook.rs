use crate::{error, resource, response, setter};
use log::*;
use serde::ser::{Serialize, SerializeTuple, Serializer};
use serde::Deserialize;
use url::Url;

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

    pub fn get(&self, id: &str) -> error::Result<response::Response<Webhook>> {
        let url = self.base_url.join(id)?;
        debug!("Sending webhook get request to {}", url.to_string());
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<Webhook>>()?;
        trace!("Webhook get request responded with {:?}", resp);
        Ok(resp)
    }
}

pub struct ListRequestBuilder<'a> {
    client: &'a reqwest::blocking::Client,
    base_url: Url,
    token: String,

    size: Option<u32>,
}

enum ListParams {
    PageSize(u32),
}

impl serde::Serialize for ListParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ListParams::PageSize(size) => {
                let mut tu = serializer.serialize_tuple(2)?;
                tu.serialize_element("page[size]")?;
                tu.serialize_element(size)?;
                tu.end()
            }
        }
    }
}

impl<'a> ListRequestBuilder<'a> {
    setter!(size, u32);

    pub fn exec(&self) -> error::Result<response::Response<Vec<Webhook>>> {
        let url = self.base_url.clone();
        let mut query = vec![];
        if let Some(size) = self.size {
            query.push(ListParams::PageSize(size));
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
    use crate::response::SuccessfulResponse;
    use crate::test_deserialization;

    test_deserialization!(test_de, "webhook.json", Webhook);
    test_deserialization!(test_multi_de, "webhook_list.json", Vec<Webhook>);
}
