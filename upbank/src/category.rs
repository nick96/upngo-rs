use crate::{error::Result, resource::Resource, response::Response, setter};
use log::*;
use serde::Deserialize;
use url::{form_urlencoded, Url};

pub struct CategoryClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    token: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    pub parent: ParentRelationships,
    pub children: ChildRelationships,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParentRelationships {
    pub data: Option<Relationship>,
    pub links: Option<RelatedLinks>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChildRelationships {
    pub data: Vec<Relationship>,
    pub links: Option<RelatedLinks>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelatedLinks {
    pub related: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    #[serde(rename = "type")]
    pub relationship_type: String,
    pub id: String,
}

pub type Category = Resource<Attributes, Relationships>;

impl CategoryClient {
    pub fn new(base_url: Url, token: String) -> Self {
        CategoryClient {
            client: reqwest::blocking::Client::new(),
            base_url,
            token,
        }
    }
    pub fn get(&self, id: &str) -> Result<Response<Category>> {
        let url = self.base_url.join(&id)?;
        debug!("Sending category get request to {}", url.to_string());
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<Response<Category>>()?;
        trace!("Get category responded with {:?}", resp);
        Ok(resp)
    }

    pub fn list(&self) -> ListRequestBuilder {
        ListRequestBuilder {
            parent: None,
            base_url: self.base_url.clone(),
            client: &self.client,
            token: self.token.clone(),
        }
    }
}

pub struct ListRequestBuilder<'a> {
    parent: Option<String>,
    base_url: Url,
    client: &'a reqwest::blocking::Client,
    token: String,
}

impl<'a> ListRequestBuilder<'a> {
    setter!(parent, String);

    pub fn exec(&self) -> Result<Response<Vec<Category>>> {
        let mut query = vec![];
        if let Some(parent) = &self.parent {
            query.push(("filter[parent]", parent));
        }

        debug!(
            "Sending category list request to {}",
            self.base_url.to_string()
        );
        let resp = self
            .client
            .get(self.base_url.clone())
            .bearer_auth(&self.token)
            .query(&query)
            .send()?
            .json::<Response<Vec<Category>>>()?;
        trace!("List category responded with {:?}", resp);
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::Category;
    use crate::response::SuccessfulResponse;
    use crate::test_deserialization;

    test_deserialization!(test_de, "category.json", Category);
    test_deserialization!(test_multi_de, "category_list.json", Vec<Category>);
}
