use url::Url;

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
                    .expect(&format!("Couldn't add 'util' to base URL {}", base_url)),
                token,
            ),
        }
    }

    pub fn default_client(token: String) -> Self {
        Client::new(default_base_url(), token)
    }
}

mod util {
    use super::{error, response::Response};
    use log::*;
    use serde::Deserialize;
    use url::Url;

    pub struct Util {
        client: reqwest::blocking::Client,
        base_url: Url,
        token: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Meta {
        pub id: String,
        pub status_emoji: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Ping {
        pub meta: Meta,
    }

    impl Util {
        pub fn new(base_url: Url, token: String) -> Self {
            Util {
                client: reqwest::blocking::Client::new(),
                base_url,
                token,
            }
        }

        pub fn ping(&self) -> error::Result<Response<Ping>> {
            let ping_url = self
                .base_url
                .join("ping")
                .expect("could not join 'ping' to base URL");
            debug!("Sending ping request to {}", ping_url.to_string(),);
            let resp = self
                .client
                .get(ping_url)
                .bearer_auth(&self.token)
                .send()?
                .json::<Response<Ping>>()?;
            Ok(resp)
        }
    }
}

pub mod response {
    use super::error;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Response<T> {
        Ok(T),
        Err(error::Error),
    }

    impl<T> Response<T> {
        pub fn is_ok(&self) -> bool {
            match self {
                Response::Ok(_) => true,
                Response::Err(_) => false,
            }
        }

        pub fn is_err(&self) -> bool {
            !self.is_ok()
        }
    }
}

mod error {
    use serde::Deserialize;
    use std::option::Option;

    #[derive(Deserialize, Debug)]
    pub struct Source {
        parameter: Option<String>,
        pointer: Option<String>,
    }

    impl std::fmt::Display for Source {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({:?}, {:?})", self.parameter, self.pointer)
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct ErrorObject {
        pub status: String,
        pub title: String,
        pub detail: String,
        pub source: Option<String>,
    }

    impl std::fmt::Display for ErrorObject {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{} - {} - {} - {:?}",
                self.status, self.title, self.detail, self.source
            )
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct Error {
        pub errors: Vec<ErrorObject>,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            for error in &self.errors {
                let res = writeln!(f, "- {}", error);
                if res.is_err() {
                    return res;
                }
            }
            Ok(())
        }
    }

    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}
