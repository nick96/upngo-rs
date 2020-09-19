use crate::{currency, error, resource, response, setter};
use log::*;
use serde::ser::{SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use url::Url;

pub struct TransactionClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    token: String,
}

#[derive(Deserialize, Debug, Display, PartialEq, Serialize, Clone)]
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagInputResource {
    #[serde(rename = "type")]
    pub resource_type: resource::ResourceType,
    pub id: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagInputResources {
    pub data: Vec<TagInputResource>,
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

    pub fn tag(&self, id: &str, tags: Vec<String>) -> error::Result<()> {
        self.add_or_delete_tag(id, tags, false)
    }

    pub fn delete_tag(&self, id: &str, tags: Vec<String>) -> error::Result<()> {
        self.add_or_delete_tag(id, tags, true)
    }

    fn add_or_delete_tag(&self, id: &str, tags: Vec<String>, delete: bool) -> error::Result<()> {
        let url = self.base_url.join(&format!("{}/relationships/tags", id))?;
        debug!("Tagging transaction {} with tags {:?}", id, tags);
        let body = TagInputResources {
            data: tags
                .iter()
                .map(|t| TagInputResource {
                    resource_type: resource::ResourceType::Tags,
                    id: t.clone(),
                })
                .collect(),
        };
        let se_body = serde_json::to_string(&body)?;
        let req = if delete {
            self.client.delete(url)
        } else {
            self.client.post(url)
        };
        req.bearer_auth(&self.token).body(se_body).send()?;
        Ok(())
    }
}

pub struct ListRequestBuilder<'a> {
    base_url: Url,
    client: &'a reqwest::blocking::Client,
    token: String,

    size: Option<u32>,
    status: Option<Status>,
    since: Option<chrono::DateTime<chrono::Utc>>,
    until: Option<chrono::DateTime<chrono::Utc>>,
    category: Option<String>,
    tag: Option<String>,
}

enum ListParams {
    Size(u32),
    Status(Status),
    Since(chrono::DateTime<chrono::Utc>),
    Until(chrono::DateTime<chrono::Utc>),
    Category(String),
    Tag(String),
}

impl Serialize for ListParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tu = serializer.serialize_tuple(2)?;
        match self {
            ListParams::Size(val) => {
                tu.serialize_element("page[size]")?;
                tu.serialize_element(val)?;
            }
            ListParams::Status(val) => {
                tu.serialize_element("filter[status]")?;
                tu.serialize_element(val)?;
            }
            ListParams::Since(val) => {
                tu.serialize_element("filter[since]")?;
                tu.serialize_element(val)?;
            }
            ListParams::Until(val) => {
                tu.serialize_element("filter[until]")?;
                tu.serialize_element(val)?;
            }
            ListParams::Category(val) => {
                tu.serialize_element("filter[category]")?;
                tu.serialize_element(val)?;
            }
            ListParams::Tag(val) => {
                tu.serialize_element("filter[tag]")?;
                tu.serialize_element(val)?;
            }
        };
        tu.end()
    }
}

impl<'a> ListRequestBuilder<'a> {
    setter!(size, u32);
    setter!(status, Status);
    setter!(since, chrono::DateTime<chrono::Utc>);
    setter!(until, chrono::DateTime<chrono::Utc>);
    setter!(category, String);
    setter!(tag, String);

    pub fn exec(&self) -> error::Result<response::Response<Vec<Transaction>>> {
        let url = self.base_url.clone();

        let mut query = vec![];

        if let Some(size) = self.size {
            query.push(ListParams::Size(size));
        }

        if let Some(status) = &self.status {
            query.push(ListParams::Status(status.clone()));
        }

        if let Some(since) = self.since {
            query.push(ListParams::Since(since))
        }

        if let Some(until) = self.until {
            query.push(ListParams::Until(until));
        }

        if let Some(category) = &self.category {
            query.push(ListParams::Category(category.clone()));
        }

        if let Some(tag) = &self.tag {
            query.push(ListParams::Tag(tag.clone()));
        }

        debug!("Sending transaction list request to {}", url.to_string(),);
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .query(&query)
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
    use crate::test_deserialization;

    test_deserialization!(test_de, "transaction.json", Transaction);
    test_deserialization!(test_multi_de, "transaction_list.json", Vec<Transaction>);
}
