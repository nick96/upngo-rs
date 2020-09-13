use crate::{error::Result, resource::ResourceType, response::Response, setter};
use log::*;
use serde::Deserialize;
use url::Url;

pub struct Client {
    client: reqwest::blocking::Client,
    base_url: Url,
    token: String,
}

impl Client {
    pub fn new(base_url: Url, token: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url,
            token,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelatedLinks {
    pub related: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkRelationship {
    pub links: RelatedLinks,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    pub transactions: Option<LinkRelationship>,
}

// pub type Tag = Resource<(), Relationships>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    pub id: String,
    pub relationships: Relationships,
}

pub type TagClient = Client;

impl TagClient {
    pub fn get(&self, id: &str) -> Result<Response<Tag>> {
        let url = self.base_url.join(id)?;
        debug!("Sending get tag request to {}", url.to_string());
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<Response<Tag>>()?;
        trace!("Response from get tag request: {:?}", resp);
        Ok(resp)
    }

    pub fn list(&self) -> ListRequestBuilder {
        ListRequestBuilder {
            size: None,
            base_url: self.base_url.clone(),
            client: &self.client,
            token: self.token.clone(),
        }
    }
}

pub struct ListRequestBuilder<'a> {
    size: Option<u32>,

    base_url: Url,
    client: &'a reqwest::blocking::Client,
    token: String,
}

impl<'a> ListRequestBuilder<'a> {
    setter!(size, u32);

    pub fn exec(&self) -> Result<Response<Vec<Tag>>> {
        let mut query = vec![];
        if let Some(size) = self.size {
            query.push(("page[size]", size));
        }
        debug!("Sending list tag request to {}", self.base_url.to_string());
        let resp = self
            .client
            .get(self.base_url.clone())
            .bearer_auth(&self.token)
            .query(&query)
            .send()?
            .json::<Response<Vec<Tag>>>()?;
        trace!("Response from list tag request: {:?}", resp);
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::Tag;
    use crate::response::SuccessfulResponse;
    use crate::test_deserialization;

    test_deserialization!(test_multi_de, "tag_list.json", Vec<Tag>);
}
