use argh::FromArgs;
use keyring::Keyring;

mod accounts {
    use argh::FromArgs;

    /// Get accounts by their ID.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "get")]
    pub struct Get {
        #[argh(positional)]
        id: String,
    }

    /// List all accounts.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "list")]
    pub struct List {}

    /// Available account subcommands.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand)]
    pub enum Subcommands {
        Get(Get),
        List(List),
    }

    /// Accounts subcommand CLI interface.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "account")]
    pub struct Accounts {
        /// subcommands available in accounts.
        #[argh(subcommand)]
        subcommands: Subcommands,
    }
}

mod transactions {
    use argh::FromArgs;

    /// Get a transaction by its ID.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "get")]
    pub struct Get {}

    /// Get all transactions.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "list")]
    pub struct List {}

    /// Subcommands available for transactions.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand)]
    pub enum Subcommands {
        Get(Get),
        List(List),
    }

    /// Transactions subcommand CLI interface.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "transaction")]
    pub struct Transactions {}
}

mod webhooks {
    use argh::FromArgs;
    /// Register a webhook.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "register")]
    pub struct Register {}

    /// Ping a webhook.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "ping")]
    pub struct Ping {}

    /// List all registered webhooks.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "list")]
    pub struct List {}

    /// Subcommands available for webhooks.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand)]
    pub enum Subcommands {
        Register(Register),
        Ping(Ping),
        List(List),
    }

    /// Webhooks subcommnad CLI interface.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "webhook")]
    pub struct Webhooks {}
}

mod ping {
    use argh::FromArgs;

    /// Ping subcommand CLI interface.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "ping")]
    pub struct Ping {}
}

mod client {
    #[derive(Debug)]
    pub struct Client {
        token: String,
        base_url: String,
    }

    impl Client {
        pub fn new(token: String, base_url: String) -> Self {
            return Client { token, base_url };
        }
    }
}

mod init {
    use argh::FromArgs;
    use keyring::Keyring;
    use rpassword;

    use super::{Result, SERVICE_NAME, USERNAME};

    /// Subcommand to initialise the token in the keychain.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "init")]
    pub struct Init {}

    impl Init {
        pub fn run(&self) -> Result<()> {
            let tok = rpassword::read_password_from_tty(Some("UpBank personal token: "))?;
            let keyring = Keyring::new(SERVICE_NAME, USERNAME);
            keyring.set_password(&tok)?;
            Ok(())
        }
    }
}

/// Subcommands available in upngo.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Accounts(accounts::Accounts),
    Transactions(transactions::Transactions),
    Webhooks(webhooks::Webhooks),
    Ping(ping::Ping),
    Init(init::Init),
}

/// UpnGo CLI interface.
#[derive(FromArgs, Debug)]
struct UpnGo {
    /// token used to authenticate with UpBank.
    #[argh(option)]
    token: Option<String>,
    /// base url for UpBank API.
    #[argh(option)]
    base_url: Option<String>,

    // subcommands.
    #[argh(subcommand)]
    subcommand: Subcommands,
}

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

const SERVICE_NAME: &str = "upngo";
const USERNAME: &str = "upngo-token";

fn derive_default_token() -> Result<String> {
    // Try to get the token via the env var first because that's nice in dev but
    // fall back to keychain
    let tok = match std::env::var("UPBANK_TOKEN") {
        Ok(var) => var,
        Err(_) => {
            let keyring = Keyring::new(SERVICE_NAME, USERNAME);
            keyring.get_password()?
        }
    };
    Ok(tok)
}

fn derive_default_base_url() -> String {
    match std::env::var("UPBANK_URL") {
        Ok(url) => url,
        Err(_) => "https://api.up.com.au".to_string(),
    }
}

fn main() -> Result<()> {
    let upngo: UpnGo = argh::from_env();

    // When initialising we don't care about the URL or the token so handle it
    // before anything else.
    if let Subcommands::Init(cmd) = &upngo.subcommand {
        return cmd.run();
    }

    let token = match upngo.token.clone() {
        Some(tok) => tok,
        None => derive_default_token()?,
    };
    let base_url = upngo
        .base_url
        .clone()
        .unwrap_or_else(derive_default_base_url);
    let client = client::Client::new(token, base_url);
    println!("{:?}", client);
    println!("{:?}", upngo);
    Ok(())
}
