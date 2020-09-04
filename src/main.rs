use argh::FromArgs;
use keyring::Keyring;

use upngo::{accounts, client, init, ping, transactions, webhooks, Result, SERVICE_NAME, USERNAME};

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
    match &upngo.subcommand {
        Subcommands::Accounts(cmd) => match &cmd.subcommands {
            accounts::Subcommands::Get(get) => get.run(client),
            accounts::Subcommands::List(list) => list.run(client),
        },
        Subcommands::Ping(_) => todo!(),
        Subcommands::Transactions(_) => todo!(),
        Subcommands::Webhooks(_) => todo!(),
        // We've already handled the init subcommand so if we're here then
        // things there's a problem.
        Subcommands::Init(_) => unreachable!(),
    }
}
