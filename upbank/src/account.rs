use crate::{error, iso4217, resource, response};
use log::*;
use serde::Deserialize;
use url::Url;

pub struct AccountClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    token: String,
}

#[derive(Deserialize, Debug)]
pub enum AccountType {
    SAVER,
    TRANSACTIONAL,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    pub currency_code: iso4217::CurrencyCode,
    pub value: String,
    pub value_in_base_units: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub display_name: String,
    pub account_type: AccountType,
    pub balance: Money,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
pub struct RelatedLinks {
    pub related: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionLinks {
    pub links: RelatedLinks,
}

#[derive(Deserialize, Debug)]
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

    pub fn list(&self) -> error::Result<response::Response<Vec<Account>>> {
        debug!(
            "Sending account list request to {}",
            self.base_url.to_string()
        );
        let resp = self
            .client
            .get(self.base_url.clone())
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<Vec<Account>>>()?;
        trace!("List accounts responded with {:?}", resp);
        Ok(resp)
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
}
