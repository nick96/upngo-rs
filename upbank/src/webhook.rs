use crate::{error, resource, response, setter};
use log::*;
use serde::ser::{SerializeStruct, SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
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

    pub fn get(&self, id: &str) -> error::Result<response::Response<WebhookResponse>> {
        let url = self.base_url.join(id)?;
        debug!("Sending webhook get request to {}", url.to_string());
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<WebhookResponse>>()?;
        trace!("Webhook get request responded with {:?}", resp);
        Ok(resp)
    }

    pub fn ping(&self, id: &str) -> error::Result<response::Response<WebhookPing>> {
        let url = self.base_url.join(&(id.to_owned() + "/"))?.join("ping")?;
        debug!("Sending webhook ping rquest to {}", url.to_string());
        let resp = self
            .client
            .post(url)
            .body("")
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<WebhookPing>>()?;
        trace!("Webhook ping rquest responded with {:?}", resp);
        Ok(resp)
    }

    pub fn logs<'a>(&'a self, id: &'a str) -> LogListRequestBuilder<'a> {
        LogListRequestBuilder {
            client: &self.client,
            base_url: self.base_url.clone(),
            token: self.token.clone(),
            id,

            size: None,
        }
    }

    pub fn delete(&self, id: &str) -> error::Result<()> {
        let url = self.base_url.join(id)?;
        debug!("Sending delete webhook request to {}", url.to_string());
        let resp = self.client.delete(url).bearer_auth(&self.token).send()?;
        trace!("Delete webhook request responded with {:?}", resp);
        Ok(())
    }

    pub fn register(&self, webhook: &Webhook) -> error::Result<response::Response<WebhookResponse>> {
        debug!(
            "Sending create webhook request to {}",
            self.base_url.to_string()
        );
        let webhook_ser = serde_json::to_string(webhook)?;
        let resp = self
            .client
            .post(self.base_url.clone())
            .body(webhook_ser)
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<WebhookResponse>>()?;
        trace!("Create webhook request responded with {:?}", resp);
        Ok(resp)
    }
}

pub struct LogListRequestBuilder<'a> {
    client: &'a reqwest::blocking::Client,
    base_url: Url,
    token: String,
    id: &'a str,

    size: Option<u32>,
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

    pub fn exec(&self) -> error::Result<response::Response<Vec<WebhookResponse>>> {
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
            .json::<response::Response<Vec<WebhookResponse>>>()?;
        trace!("Webhook list webhook request responded with: {:?}", resp);
        Ok(resp)
    }
}

impl<'a> LogListRequestBuilder<'a> {
    setter!(size, u32);

    pub fn exec(&self) -> error::Result<response::Response<Vec<WebhookLogRecord>>> {
        let url = self
            .base_url
            .join(&(self.id.to_owned() + "/"))?
            .join("logs")?;
        let mut query = vec![];
        if let Some(size) = self.size {
            query.push(ListParams::PageSize(size));
        }
        debug!("Sending webhook log request to {}", url.to_string());
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .query(&query)
            .send()?
            .json::<response::Response<Vec<WebhookLogRecord>>>()?;
        trace!("Webhook log request responded with: {:?}", resp);
        Ok(resp)
    }
}

pub type WebhookResponse = resource::Resource<Attributes, Relationships>;

pub type Webhook = DataContainer<SettableAttributes>;

// Only implement serialize for webhook because not all DataContainers need to
// be serializable.
impl Serialize for Webhook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Webhook", 1)?;
        s.serialize_field("data", &self.data)?;
        s.end()
    }
}

impl Webhook {
    pub fn new(url: String, description: Option<String>) -> Self {
        Self {
            data: SettableAttributes { url, description },
            links: None,
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettableAttributes {
    pub url: String,
    pub description: Option<String>,
}

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

type WebhookLogRecord =
    resource::Resource<WebhookLogRecordAttributes, WebhookLogRecordRelationships>;

#[derive(Deserialize, Debug, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebhookDeliveryStatus {
    Delivered,
    Undeliverable,
    BadResponseCode,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebhookLogRecordAttributes {
    pub request: WebhookLogRequest,
    pub response: WebhookLogResponse,
    pub delivery_status: WebhookDeliveryStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebhookLogRequest {
    pub body: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebhookLogResponse {
    pub status_code: u32,
    pub body: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebhookLogRecordRelationships {}

type WebhookPing = resource::Resource<WebhookPingAttributes, WebhookPingRelationships>;

#[derive(Serialize, Deserialize, Debug, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    TransactionCreated,
    TransactionSettled,
    TransactionDeleted,
    Ping,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebhookPingAttributes {
    pub event_type: EventType,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebhookPingRelationships {
    pub webhook: Relationship,
    pub transaction: Option<Relationship>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataContainer<T> {
    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    data: T,
    links: Option<RelatedLinks>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeAndId {
    #[serde(rename = "type")]
    pub resource_type: resource::ResourceType,
    pub id: String,
}

pub type Relationship = DataContainer<TypeAndId>;

#[cfg(test)]
mod test {
    use super::Webhook;
    use crate::response::SuccessfulResponse;
    use crate::test_deserialization;

    test_deserialization!(test_de, "webhook.json", Webhook);
    test_deserialization!(test_multi_de, "webhook_list.json", Vec<Webhook>);
}
