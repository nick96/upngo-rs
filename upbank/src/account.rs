use crate::{currency, error, resource, response, transaction};
use log::*;
use serde::Deserialize;
use strum_macros::Display;
use url::Url;

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

    pub fn list(&self) -> ListRequestBuilder {
        ListRequestBuilder {
            count: None,
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

    pub fn transactions(
        &self,
        id: String,
    ) -> error::Result<response::Response<Vec<transaction::Transaction>>> {
        // Append "/" to the ID so that appending "transactions" after doesn't
        // stomp it.
        let id_part = id + "/";
        let url = self.base_url.join(&id_part)?.join("transactions")?;
        debug!(
            "Sending account transactions get request to {}",
            url.to_string()
        );
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<Vec<transaction::Transaction>>>()?;
        trace!("Get account transactions responded with {:?}", resp);
        Ok(resp)
    }
}

pub struct ListRequestBuilder<'a> {
    count: Option<u32>,
    base_url: Url,
    client: &'a reqwest::blocking::Client,
    token: String,
}

impl<'a> ListRequestBuilder<'a> {
    pub fn count(&mut self, count: u32) -> &mut Self {
        self.count = Some(count);
        self
    }

    pub fn exec(&self) -> error::Result<response::Response<Vec<Account>>> {
        debug!(
            "Sending account list request to {}",
            self.base_url.to_string()
        );
        let resp = self
            .client
            .get(self.base_url.clone())
            .query(&[("page[size]", self.count)])
            .bearer_auth(&self.token)
            .send()?
            .json::<response::Response<Vec<Account>>>()?;
        trace!("List accounts responded with {:?}", resp);
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::Account;
    use crate::response::SuccessfulResponse;
    use serde_json;

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
