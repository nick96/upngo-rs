use anyhow::{anyhow, Context, Result};
use argh::FromArgs;
use log::*;
use prettytable::{cell, row, table, Table};
use upbank::Client;
use url::Url;

/// UpBank CLI.
#[derive(FromArgs)]
struct Upngo {
    /// token to authenticate with.
    #[argh(option, short = 't')]
    token: Option<String>,
    /// url to use as base.
    #[argh(option, default = "default_url()", short = 'u')]
    url: String,
    #[argh(subcommand)]
    subcomand: Subcommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommand {
    List(ListCommand),
    Get(GetCommand),
    Register(RegisterCommand),
}

/// List a resource.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
struct ListCommand {
    /// resource to list.
    #[argh(subcommand)]
    resource: ListResourceCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum ListResourceCommand {
    Transactions(ListTransactions),
    Accounts(ListAccounts),
    Categories(ListCategories),
    Tags(ListTags),
    Webhooks(ListWebhooks),
}

/// List transactions.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "transactions")]
struct ListTransactions {
    /// max number of transactions to list.
    #[argh(option, short = 'n')]
    max_count: Option<u32>,
    /// filter transactions by status.
    #[argh(option, short = 's')]
    status: Option<upbank::transaction::Status>,
}

/// List accounts.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "accounts")]
struct ListAccounts {
    /// max number of accounts to list.
    #[argh(option, short = 'n')]
    max_count: Option<u32>,
}

/// List categories.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "categories")]
struct ListCategories {}

/// List tags.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tags")]
struct ListTags {}

/// List webhooks.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "webhooks")]
struct ListWebhooks {}

/// Get a resource by its ID.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "get")]
struct GetCommand {
    /// resource to get.
    #[argh(subcommand)]
    resource: GetResourceCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum GetResourceCommand {
    Transaction(GetTransaction),
    Account(GetAccount),
    Category(GetCategory),
    Tag(GetTag),
    Webhook(GetWebhook),
}

/// Get a transactions.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "transactions")]
struct GetTransaction {
    /// id of the transaction to get.
    #[argh(positional)]
    id: String,
}

/// Get am account..
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "accounts")]
struct GetAccount {
    /// id of the account to get.
    #[argh(positional)]
    id: String,
}

/// Get a category.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "categories")]
struct GetCategory {}

/// Get a tag.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tags")]
struct GetTag {}

/// Get a webhook.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "webhooks")]
struct GetWebhook {}

/// Register a resource.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "register")]
struct RegisterCommand {
    /// resource to register.
    #[argh(subcommand)]
    resource: RegisterResourceCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum RegisterResourceCommand {
    Webhook(RegisterWebhook),
}

/// Register a webhook.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "webhook")]
struct RegisterWebhook {
    /// url the webhook is located at.
    #[argh(positional)]
    url: String,
    /// description of the webhook.
    #[argh(option, short = 'd')]
    description: Option<String>,
}

fn default_url() -> String {
    "https://api.up.com.au/api/v1/".to_string()
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut args: Upngo = argh::from_env();
    // Make sure the URL ends with "/" otherwise URLs won't be built properly.
    if !args.url.ends_with('/') {
        args.url = format!("{}/", args.url);
    }

    let url = Url::parse(&args.url)?;

    let token = match args.token {
        Some(token) => token,
        None => {
            debug!("Retrieving UpBank token from UPBANK_TOKEN environment variable");
            std::env::var("UPBANK_TOKEN").expect(
                "Failed to retrieve UpBank token from flag or UPBANK_TOKEN environment variable",
            )
        }
    };

    let client = Client::new(url, token);

    use Subcommand::*;
    match args.subcomand {
        Get(get) => run_get(client, get)?,
        List(list) => run_list(client, list)?,
        Register(register) => run_register(client, register)?,
    }

    Ok(())
}

fn run_get(client: Client, get: GetCommand) -> Result<()> {
    use GetResourceCommand::*;
    match get.resource {
        Account(account) => run_get_account(client, account),
        Transaction(transaction) => run_get_transaction(client, transaction),
        Webhook(webhook) => run_get_webhook(client, webhook),
        Category(cat) => run_get_category(client, cat),
        Tag(tag) => run_get_tag(client, tag),
    }
}

fn run_get_account(client: Client, account: GetAccount) -> Result<()> {
    let resp = client
        .account
        .get(account.id.clone())
        .with_context(|| format!("Failed to get account with ID {}", account.id))?;
    match resp {
        upbank::response::Response::Ok(acc) => {
            let attrs: upbank::account::Attributes = acc.data.attributes;
            let table = table!(
                ["Name", "Type", "Balance", "Created At"],
                [
                    attrs.display_name,
                    attrs.account_type,
                    attrs.balance,
                    attrs.created_at
                ]
            );
            table.printstd();
            Ok(())
        }
        upbank::response::Response::Err(e) => Err(anyhow!(
            "Failed to get account with ID {}:\n{}",
            &account.id,
            e
        )),
    }
}

fn run_get_transaction(client: Client, transaction: GetTransaction) -> Result<()> {
    let resp = client
        .transaction
        .get(transaction.id.clone())
        .with_context(|| format!("Failed to get transaction with ID {}", transaction.id))?;
    match resp {
        upbank::response::Response::Ok(transac) => {
            let attrs = transac.data.attributes;
            let table = table!(
                ["Description", "Amount", "Status", "Created", "Settled"],
                [
                    attrs.description,
                    attrs.amount,
                    attrs.status,
                    attrs.created_at,
                    attrs
                        .settled_at
                        .map_or_else(|| "N/A".to_string(), |d| d.to_string(),),
                ]
            );
            table.printstd();
            Ok(())
        }
        upbank::response::Response::Err(e) => Err(anyhow!(
            "Failed to get transaction with ID {}:\n{}",
            &transaction.id,
            e
        )),
    }
}

fn run_get_webhook(client: Client, webhook: GetWebhook) -> Result<()> {
    unimplemented!()
}

fn run_get_category(client: Client, account: GetCategory) -> Result<()> {
    unimplemented!()
}

fn run_get_tag(client: Client, account: GetTag) -> Result<()> {
    unimplemented!()
}

fn run_list(client: Client, list: ListCommand) -> Result<()> {
    use ListResourceCommand::*;
    match list.resource {
        Accounts(accounts) => run_list_accounts(client, accounts),
        Transactions(transactions) => run_list_transactions(client, transactions),
        Categories(categories) => run_list_categories(client, categories),
        Tags(tags) => run_list_tags(client, tags),
        Webhooks(webhooks) => run_list_webhooks(client, webhooks),
    }
}

fn run_list_accounts(client: Client, accounts: ListAccounts) -> Result<()> {
    if accounts.max_count.is_some() {
        warn!("Limiting accounts with max-count is not implemented yet");
    }
    let resp = client.account.list().context("Failed to list accounts")?;
    match resp {
        upbank::response::Response::Ok(accs) => {
            let mut table = Table::new();
            table.add_row(row!["Name", "Balance", "Type", "Created", "ID"]);
            for acc in accs.data {
                table.add_row(row![
                    acc.attributes.display_name,
                    acc.attributes.balance,
                    acc.attributes.account_type,
                    acc.attributes.created_at,
                    acc.id,
                ]);
            }
            table.printstd();
            Ok(())
        }
        upbank::response::Response::Err(e) => Err(anyhow!("Failed to list accounts:\n{}", e)),
    }
}

fn run_list_transactions(client: Client, transactions: ListTransactions) -> Result<()> {
    if transactions.max_count.is_some() {
        warn!("Limiting transactions with max-count is not implemented yet")
    }
    let resp = client
        .transaction
        .list()
        .context("Failed to list transactions")?;
    match resp {
        upbank::response::Response::Ok(transacts) => {
            let mut table = Table::new();
            table.add_row(row![
                "Description",
                "Amount",
                "Status",
                "Created",
                "Settled",
                "ID",
            ]);
            for transaction in transacts.data {
                table.add_row(row![
                    transaction.attributes.description,
                    transaction.attributes.amount,
                    transaction.attributes.status,
                    transaction.attributes.created_at,
                    transaction
                        .attributes
                        .settled_at
                        .map_or_else(|| "N/A".to_string(), |d| d.to_string(),),
                    transaction.id,
                ]);
            }
            table.printstd();
            Ok(())
        }
        upbank::response::Response::Err(e) => Err(anyhow!("Failed to list transactions:\n{}", e)),
    }
}

fn run_list_categories(client: Client, categories: ListCategories) -> Result<()> {
    unimplemented!()
}

fn run_list_tags(client: Client, tags: ListTags) -> Result<()> {
    unimplemented!()
}

fn run_list_webhooks(client: Client, webhooks: ListWebhooks) -> Result<()> {
    unimplemented!()
}

fn run_register(client: Client, register: RegisterCommand) -> Result<()> {
    unimplemented!()
}
