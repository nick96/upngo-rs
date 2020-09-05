use url::Url;

pub mod response;

mod error;
mod util;

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
                    .unwrap_or_else(|_| panic!("Couldn't add 'util' to base URL {}", base_url)),
                token,
            ),
        }
    }

    pub fn default_client(token: String) -> Self {
        Client::new(default_base_url(), token)
    }
}
