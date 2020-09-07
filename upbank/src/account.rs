use crate::{currency, error, resource, response, setter, transaction};
use log::*;
use serde::Deserialize;
use strum_macros::Display;
use url::{form_urlencoded, Url};

pub struct AccountClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    token: String,
}

#[derive(Deserialize, Debug, Display)]
pub enum AccountType {
    SAVER,
    TRANSACTIONAL,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub display_name: String,
    pub account_type: AccountType,
    pub balance: currency::Money,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelatedLinks {
    pub related: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLinks {
    pub links: RelatedLinks,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    pub transactions: TransactionLinks,
}

pub type Account = resource::Resource<Attributes, Relationships>;

impl AccountClient {
    pub fn new(base_url: Url, token: String) -> Self {
        AccountClient {
            client: reqwest::blocking::Client::new(),
            base_url,
            token,
        }
    }

    pub fn list(&self) -> AccountListRequestBuilder {
        AccountListRequestBuilder {
            size: None,
            client: &self.client,
            base_url: self.base_url.clone(),
            token: self.token.clone(),
        }
    }

    pub fn get(&self, id: String) -> error::Result<response::Response<Account>> {
        let account_url = self.base_url.join(&id)?;
        debug!("Sending account get request to {}", account_url.to_string());
        let resp = self
            .client
            .get(account_url)
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<Account>>()?;
        trace!("Get accounts responded with {:?}", resp);
        Ok(resp)
    }

    pub fn transactions(&self, id: String) -> TransactionListRequestBuilder {
        TransactionListRequestBuilder {
            base_url: self.base_url.clone(),
            client: &self.client,
            token: self.token.clone(),
            id,

            size: None,
            status: None,
            since: None,
            until: None,
            category: None,
            tag: None,
        }
    }
}

pub struct AccountListRequestBuilder<'a> {
    size: Option<u32>,
    base_url: Url,
    client: &'a reqwest::blocking::Client,
    token: String,
}

impl<'a> AccountListRequestBuilder<'a> {
    setter!(size, u32);

    pub fn exec(&self) -> error::Result<response::Response<Vec<Account>>> {
        let mut query = vec![];
        if let Some(size) = self.size {
            query.push(("page[size]", size))
        }
        debug!(
            "Sending account list request to {}",
            self.base_url.to_string()
        );
        let resp = self
            .client
            .get(self.base_url.clone())
            .bearer_auth(&self.token)
            .query(&query)
            .send()?
            .json::<response::Response<Vec<Account>>>()?;
        trace!("List accounts responded with {:?}", resp);
        Ok(resp)
    }
}

pub struct TransactionListRequestBuilder<'a> {
    base_url: Url,
    client: &'a reqwest::blocking::Client,
    token: String,
    id: String,

    size: Option<u32>,
    status: Option<transaction::Status>,
    since: Option<chrono::DateTime<chrono::Local>>,
    until: Option<chrono::DateTime<chrono::Local>>,
    category: Option<String>,
    tag: Option<String>,
}

impl<'a> TransactionListRequestBuilder<'a> {
    setter!(size, u32);
    setter!(status, transaction::Status);
    setter!(since, chrono::DateTime<chrono::Local>);
    setter!(until, chrono::DateTime<chrono::Local>);
    setter!(category, String);
    setter!(tag, String);

    pub fn exec(&self) -> error::Result<response::Response<Vec<transaction::Transaction>>> {
        let mut query = vec![];

        if let Some(size) = self.size {
            let value: String =
                form_urlencoded::byte_serialize(size.to_string().as_bytes()).collect();
            query.push(("filter[size]", value))
        }

        if let Some(status) = &self.status {
            let value: String =
                form_urlencoded::byte_serialize(status.to_string().as_bytes()).collect();
            query.push(("filter[status]", value))
        }

        if let Some(since) = self.since {
            let value: String =
                form_urlencoded::byte_serialize(since.to_rfc3339().as_bytes()).collect();
            query.push(("filter[since]", value));
        }

        if let Some(until) = self.until {
            let value: String =
                form_urlencoded::byte_serialize(until.to_rfc3339().as_bytes()).collect();
            query.push(("filter[until]", value));
        }

        if let Some(category) = &self.category {
            let value: String = form_urlencoded::byte_serialize(category.as_bytes()).collect();
            query.push(("filter[category]", value));
        }

        if let Some(tag) = &self.tag {
            let value: String = form_urlencoded::byte_serialize(tag.as_bytes()).collect();
            query.push(("filter[tag]", value));
        }

        // Append "/" to the ID so that appending "transactions" after doesn't
        // stomp it.
        let id_part = format!("{}/", self.id);
        let url = self.base_url.join(&id_part)?.join("transactions")?;
        debug!(
            "Sending account transactions get request to {}",
            url.to_string()
        );
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .query(&query)
            .send()?
            .json::<response::Response<Vec<transaction::Transaction>>>()?;
        trace!("Get account transactions responded with {:?}", resp);
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::Account;
    use crate::response::SuccessfulResponse;

    #[test]
    fn test_account_de() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("data");
        path.push("account.json");
        let contents = std::fs::read_to_string(path).unwrap();
        let _ = serde_json::from_str::<SuccessfulResponse<Account>>(&contents).unwrap();
    }

    #[test]
    fn test_accounts_de() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("data");
        path.push("account_list.json");
        let contents = std::fs::read_to_string(path).unwrap();
        let _ = serde_json::from_str::<SuccessfulResponse<Vec<Account>>>(&contents).unwrap();
    }
}
