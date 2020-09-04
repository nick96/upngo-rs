use crate::{rfc4217, Result};
use chrono::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum ResourceType {
    Accounts,
}

#[derive(Debug)]
pub struct Resource<A, R> {
    pub typ: ResourceType,
    pub id: String,
    pub attributes: A,
    pub relationships: R,
}

#[derive(Debug)]
pub struct Link {
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug)]
pub enum AccountType {
    Saver,
    Transactional,
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Saver => "Saver",
            Self::Transactional => "Transactional",
        };
        write!(f, "{}", repr)
    }
}

#[derive(Debug)]
pub struct MoneyObject {
    pub currency_code: rfc4217::CurrencyCode,
    pub value: String,
    pub value_in_base_units: i64,
}

impl fmt::Display for MoneyObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.currency_code)
    }
}

#[derive(Debug)]
pub struct AccountAttributes {
    pub display_name: String,
    pub account_type: AccountType,
    pub balance: MoneyObject,
    pub created_at: DateTime<Local>,
}

#[derive(Debug)]
pub struct AccountRelationships {}

#[derive(Debug)]
pub struct Accounts {
    pub data: Vec<Resource<AccountAttributes, AccountRelationships>>,
    pub links: Link,
}

#[derive(Debug)]
pub struct AccountsClient {
    token: String,
    base_url: String,
}

impl AccountsClient {
    pub fn new(token: String, base_url: String) -> Self {
        return AccountsClient { token, base_url };
    }

    pub fn list(&self) -> Result<Accounts> {
            todo!()
    }
}

#[derive(Debug)]
pub struct Client {
    pub accounts: AccountsClient,
}

impl Client {
    pub fn new(token: String, base_url: String) -> Self {
        return Client {
            accounts: AccountsClient::new(token, base_url),
        };
    }
}
