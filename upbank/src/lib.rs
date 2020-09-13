use url::Url;

pub mod response;

// Resources
pub mod account;
pub mod transaction;
pub mod util;
pub mod webhook;
pub mod category;

// Utilities that we share between modules but don't expose.
mod currency;
mod error;
mod iso4217;
mod resource;
#[macro_use]
mod macros;

pub fn default_base_url() -> Url {
    // Ending the URL in a slash is important because otherwise the v1 gets
    // stomped as we tack things on.
    Url::parse("https://api.up.com.au/api/v1/")
        .expect("Failed to parse string literal as URL. The dev is a nuff-nuff")
}

pub struct Client {
    base_url: Url,
    token: String,

    pub util: util::Util,
    pub account: account::AccountClient,
    pub transaction: transaction::TransactionClient,
    pub webhook: webhook::WebhookClient,
    pub category: category::CategoryClient,
}

macro_rules! client {
    ($typ:ty, $base:expr, $path:expr, $tok:expr) => {
        <$typ>::new(
            $base.join($path).unwrap_or_else(|_| panic!("Couldn't add '{}' to base URL {}", $path, $base)),
            $tok.clone(),
        );
    };
}

impl Client {
    pub fn new(base_url: Url, token: String) -> Self {
        Client {
            base_url: base_url.clone(),
            token: token.clone(),

            util: util::Util::new(
                base_url
                    // Has to end in a slash otherwise it gets stomped by subsequent joins
                    .join("util/")
                    .unwrap_or_else(|_| panic!("Couldn't add 'util/' to base URL {}", base_url)),
                token.clone(),
            ),
            account: account::AccountClient::new(
                base_url.join("accounts/").unwrap_or_else(|_| {
                    panic!("Couldn't add 'accounts/' to base URL {}", base_url)
                }),
                token.clone(),
            ),
            transaction: transaction::TransactionClient::new(
                base_url.join("transactions/").unwrap_or_else(|_| {
                    panic!("Couldn't add 'transactions/' to base URL {}", base_url)
                }),
                token.clone(),
            ),
            webhook: client!(webhook::WebhookClient, base_url, "webhooks/", token),
            category: category::CategoryClient::new(
                base_url.join("categories/").unwrap_or_else(|_| {
                    panic!("Couldn't add 'categories/' to base URL {}", base_url)
                }),
                token,
            ),
        }
    }

    pub fn default_client(token: String) -> Self {
        Client::new(default_base_url(), token)
    }
}
