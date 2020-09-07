use crate::{currency, error, resource, response, setter};
use log::*;
use serde::Deserialize;
use strum_macros::Display;
use url::Url;

pub struct TransactionClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    token: String,
}

#[derive(Deserialize, Debug, Display, PartialEq)]
pub enum Status {
    HELD,
    SETTLED,
}

impl std::str::FromStr for Status {
    type Err = error::ClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "held" => Ok(Status::HELD),
            "settled" => Ok(Status::SETTLED),
            _ => Err(error::ClientError::ConversionError {
                value: s.into(),
                reason: "Must be one of [held, settled] (case insensitive)".into(),
            }),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HoldInfo {
    pub amount: currency::Money,
    pub foreign_amount: Option<currency::Money>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoundUp {
    pub amount: currency::Money,
    pub boost_portion: Option<currency::Money>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cashback {
    pub description: String,
    pub amount: currency::Money,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub status: Status,
    pub raw_text: Option<String>,
    pub description: String,
    pub message: Option<String>,
    pub hold_info: Option<HoldInfo>,
    pub round_up: Option<RoundUp>,
    pub cashback: Option<Cashback>,
    pub amount: currency::Money,
    pub foreign_amount: Option<currency::Money>,
    pub settled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelatedLinks {
    pub related: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipData {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub data: Option<RelationshipData>,
    pub links: Option<RelatedLinks>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagsRelationship {
    pub data: Vec<RelationshipData>,
    pub links: resource::SelfLinks,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    pub account: Relationship,
    pub category: Relationship,
    pub parent_category: Relationship,
    pub tags: TagsRelationship,
}

pub type Transaction = resource::Resource<Attributes, Relationships>;

impl TransactionClient {
    pub fn new(base_url: Url, token: String) -> Self {
        TransactionClient {
            client: reqwest::blocking::Client::new(),
            base_url,
            token,
        }
    }

    pub fn list(&self) -> ListRequestBuilder {
        ListRequestBuilder {
            base_url: self.base_url.clone(),
            client: &self.client,
            token: self.token.clone(),

            size: None,
            status: None,
            since: None,
            until: None,
            category: None,
            tag: None,
        }
    }

    pub fn get(&self, id: String) -> error::Result<response::Response<Transaction>> {
        let url = self.base_url.join(&id)?;
        debug!("Sending transaction get request to {}", url.to_string());
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<Transaction>>()?;
        trace!("Transaction get request responded with {:?}", resp);
        Ok(resp)
    }
}

pub struct ListRequestBuilder<'a> {
    base_url: Url,
    client: &'a reqwest::blocking::Client,
    token: String,

    size: Option<u32>,
    status: Option<Status>,
    since: Option<chrono::DateTime<chrono::Local>>,
    until: Option<chrono::DateTime<chrono::Local>>,
    category: Option<String>,
    tag: Option<String>,
}

impl<'a> ListRequestBuilder<'a> {
    setter!(size, u32);
    setter!(status, Status);
    setter!(since, chrono::DateTime<chrono::Local>);
    setter!(until, chrono::DateTime<chrono::Local>);
    setter!(category, String);
    setter!(tag, String);

    pub fn exec(&self) -> error::Result<response::Response<Vec<Transaction>>> {
        let url = self.base_url.clone();
        debug!("Sending transaction list request to {}", url.to_string());
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<Vec<Transaction>>>()?;
        trace!("Transaction list request responded with {:?}", resp);
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::Transaction;
    use crate::response::SuccessfulResponse;

    #[test]
    fn test_transaction_de() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("data");
        path.push("transaction.json");
        let contents = std::fs::read_to_string(path).unwrap();
        let _ = serde_json::from_str::<SuccessfulResponse<Transaction>>(&contents).unwrap();
    }

    #[test]
    fn test_transactions_de() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("data");
        path.push("transaction_list.json");
        let contents = std::fs::read_to_string(path).unwrap();
        let _ = serde_json::from_str::<SuccessfulResponse<Vec<Transaction>>>(&contents).unwrap();
    }
}
