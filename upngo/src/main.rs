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
    Ping(PingCommand),
    Tag(TagCommand),
    ListLogs(ListLogCommand),
}

/// Ping UpBank.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ping")]
struct PingCommand {
    /// resource to ping.
    #[argh(subcommand)]
    resource: Option<PingResourceCommand>,
}

/// List a resource.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
struct ListCommand {
    /// resource to list.
    #[argh(subcommand)]
    resource: ListResourceCommand,
}

/// Tag a resource.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tag")]
struct TagCommand {
    /// resource to tag.
    #[argh(subcommand)]
    resource: TagResourceCommand,
}

/// List logs.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "logs")]
struct ListLogCommand {
    /// resource to list logs of.
    #[argh(subcommand)]
    resource: ListLogResourceCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum TagResourceCommand {
    Transaction(TagTransaction),
}

/// Tag a transaction.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "transaction")]
struct TagTransaction {
    /// id of the transaction to tag.
    #[argh(positional)]
    transaction_id: String,
    /// tags to add to the transaction.
    #[argh(positional)]
    tags: Vec<String>,
    /// delete the specified tags.
    #[argh(switch, short = 'd')]
    delete: bool,
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

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum ListLogResourceCommand {
    Webhooks(ListWebhookLogs),
}

/// List transactions.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "transactions")]
struct ListTransactions {
    /// max number of transactions to list.
    #[argh(option, short = 'n')]
    size: Option<u32>,
    /// filter transactions by status.
    #[argh(option, short = 's')]
    status: Option<upbank::transaction::Status>,
    /// filter transactions since the given date.
    #[argh(option, short = 'a')]
    since: Option<chrono::DateTime<chrono::Utc>>,
    /// filter transactions upto the given date.
    #[argh(option, short = 'b')]
    until: Option<chrono::DateTime<chrono::Utc>>,
    /// filter by category.
    #[argh(option, short = 'c')]
    category: Option<String>,
    /// filter by tag.
    #[argh(option, short = 't')]
    tag: Option<String>,
}

/// List accounts.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "accounts")]
struct ListAccounts {
    /// max number of accounts to list.
    #[argh(option, short = 'n')]
    size: Option<u32>,
}

/// List categories.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "categories")]
struct ListCategories {
    /// filter categories to only those with this parent.
    #[argh(option, short = 'p')]
    parent: Option<String>,
}

/// List tags.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tags")]
struct ListTags {
    /// max number of tags to show.
    #[argh(option, short = 'n')]
    size: Option<u32>,
}

/// List webhooks.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "webhooks")]
struct ListWebhooks {
    /// max number of webhooks to list.
    #[argh(option, short = 'n')]
    size: Option<u32>,
}

/// List webhook logs.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "webhook")]
struct ListWebhookLogs {
    /// id of the webhook to get the logs for.
    #[argh(positional)]
    id: String,

    /// max number of webhooks to list.
    #[argh(option, short = 'n')]
    size: Option<u32>,
}

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
    Webhook(GetWebhook),
}

/// Get a transactions.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "transaction")]
struct GetTransaction {
    /// id of the transaction to get.
    #[argh(positional)]
    id: String,
}

/// Get am account..
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "account")]
struct GetAccount {
    /// id of the account to get.
    #[argh(positional)]
    id: String,
}

/// Get a category.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "category")]
struct GetCategory {
    /// id of the category to get.
    #[argh(positional)]
    id: String,
}

/// Get a webhook.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "webhook")]
struct GetWebhook {
    /// id of the webhook to get.
    #[argh(positional)]
    id: String,
}

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

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum PingResourceCommand {
    Webhook(PingWebhook),
}

/// Ping a webhook.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "webhook")]
struct PingWebhook {
    /// webhook ID to ping.
    #[argh(positional)]
    id: String,
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
        Get(get) => run_get(client, get),
        List(list) => run_list(client, list),
        Register(register) => run_register(client, register),
        Ping(ping) => run_ping(client, ping),
        Tag(tag) => run_tag(client, tag),
        ListLogs(logs) => run_list_logs(client, logs),
    }
}

fn run_ping(client: Client, ping: PingCommand) -> Result<()> {
    if let Some(res) = ping.resource {
        match res {
            PingResourceCommand::Webhook(w) => run_ping_webhook(client, w),
        }
    } else {
        let resp = client.util.ping()?;
        match resp {
            upbank::util::PingResponse::Ok(ping) => {
                println!("UpBank is up {}", ping.meta.status_emoji);
                Ok(())
            }
            upbank::util::PingResponse::Err(e) => Err(anyhow!("Failed to ping UpBank: {}", e)),
        }
    }
}

fn run_ping_webhook(client: Client, webhook: PingWebhook) -> Result<()> {
    let resp = client
        .webhook
        .ping(&webhook.id)
        .with_context(|| format!("Failed to ping webhook with ID {}", webhook.id))?;
    use upbank::response::Response;
    match resp {
        Response::Ok(_) => {
            println!("Webhook {} is up! 🚀", webhook.id);
            Ok(())
        }
        Response::Err(e) => Err(anyhow!("Failed to pring webhook {}: {}", webhook.id, e)),
    }
}

fn run_get(client: Client, get: GetCommand) -> Result<()> {
    use GetResourceCommand::*;
    match get.resource {
        Account(account) => run_get_account(client, account),
        Transaction(transaction) => run_get_transaction(client, transaction),
        Webhook(webhook) => run_get_webhook(client, webhook),
        Category(cat) => run_get_category(client, cat),
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
    let resp = client
        .webhook
        .get(&webhook.id)
        .with_context(|| format!("Failed to get webhook with ID {}", webhook.id))?;
    use upbank::response::Response;
    match resp {
        Response::Ok(w) => {
            let attrs = w.data.attributes;
            let table = table!(
                ["Description", "URL", "Created", "ID"],
                [
                    attrs
                        .description
                        .map_or("None".to_string(), std::convert::identity),
                    attrs.url,
                    attrs.created_at,
                    w.data.id
                ]
            );
            table.printstd();
            Ok(())
        }
        Response::Err(e) => Err(anyhow!(
            "Failed to get webhook with ID {}:\n{}",
            &webhook.id,
            e
        )),
    }
}

fn run_get_category(client: Client, category: GetCategory) -> Result<()> {
    let resp = client
        .category
        .get(&category.id)
        .with_context(|| format!("Failed to get category with ID {}", category.id))?;
    match resp {
        upbank::response::Response::Ok(category) => {
            let attrs: upbank::category::Attributes = category.data.attributes;
            let table = table!(["Name"], [attrs.name]);
            table.printstd();
            Ok(())
        }
        upbank::response::Response::Err(e) => Err(anyhow!(
            "Failed to get transaction with ID {}:\n{}",
            &category.id,
            e
        )),
    }
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
    let mut req = client.account.list();
    if let Some(size) = accounts.size {
        req.size(size);
    }
    let resp = req.exec().context("Failed to list accounts")?;
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
    let mut req = client.transaction.list();

    if let Some(size) = transactions.size {
        req.size(size);
    }

    if let Some(cat) = transactions.category {
        req.category(cat);
    }

    if let Some(since) = transactions.since {
        req.since(since);
    }

    if let Some(status) = transactions.status {
        req.status(status);
    }

    let resp = req.exec().context("Failed to list transactions")?;
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
    let mut req = client.category.list();
    if let Some(parent) = categories.parent {
        req.parent(parent);
    }
    let resp = req.exec().context("Failed to list categories")?;
    match resp {
        upbank::response::Response::Ok(categories) => {
            let mut table = table!(["Name", "ID"]);
            for category in categories.data {
                table.add_row(row![category.attributes.name, category.id]);
            }
            table.printstd();
            Ok(())
        }
        upbank::response::Response::Err(e) => Err(anyhow!("Failed to list categories:\n{}", e)),
    }
}

fn run_list_tags(client: Client, tags: ListTags) -> Result<()> {
    let mut req = client.tag.list();
    if let Some(size) = tags.size {
        req.size(size);
    }
    let resp = req.exec().context("Failed to list tags")?;
    match resp {
        upbank::response::Response::Ok(tags) => {
            let mut table = table!(["ID"]);
            for tag in tags.data {
                table.add_row(row![tag.id]);
            }
            table.printstd();
            Ok(())
        }
        upbank::response::Response::Err(e) => Err(anyhow!("Failed to list tags:\n{}", e)),
    }
}

fn run_list_webhooks(client: Client, webhooks: ListWebhooks) -> Result<()> {
    let mut req = client.webhook.list();
    if let Some(size) = webhooks.size {
        req.size(size);
    }
    let resp = req.exec().context("Failed to list webhooks")?;
    match resp {
        upbank::response::Response::Ok(webhooks) => {
            let mut table = Table::new();
            table.add_row(row!["Description", "URL", "Created", "ID"]);
            for webhook in webhooks.data {
                table.add_row(row![
                    webhook.attributes.url,
                    webhook
                        .attributes
                        .description
                        .map_or_else(|| "N/A".to_string(), std::convert::identity),
                    webhook.attributes.created_at,
                    webhook.id,
                ]);
            }
            table.printstd();
            Ok(())
        }
        upbank::response::Response::Err(e) => Err(anyhow!("Failed to list webhooks:\n{}", e)),
    }
}

fn run_list_logs(client: Client, logs: ListLogCommand) -> Result<()> {
    use ListLogResourceCommand::*;
    match logs.resource {
        Webhooks(webhooks) => run_list_webhook_logs(client, webhooks),
    }
}

fn run_list_webhook_logs(client: Client, webhooks: ListWebhookLogs) -> Result<()> {
    let mut req = client.webhook.logs(&webhooks.id);
    if let Some(size) = webhooks.size {
        req.size(size);
    }
    let resp = req
        .exec()
        .with_context(|| format!("Failed to get logs for webhook with ID {}", webhooks.id))?;
    use upbank::response::Response;
    match resp {
        Response::Ok(w) => {
            let mut table = table!([
                "Time",
                "Request",
                "Response Code",
                "Response",
                "Status",
                "ID"
            ]);
            for record in w.data {
                table.add_row(row![
                    record.attributes.created_at,
                    truncate(record.attributes.request.body, 10),
                    record.attributes.response.status_code,
                    truncate(record.attributes.response.body, 10),
                    record.attributes.delivery_status,
                    record.id
                ]);
            }
            table.printstd();
            Ok(())
        }
        Response::Err(e) => Err(anyhow!(
            "Failed to get logs for webhook with ID {}: {}",
            webhooks.id,
            e
        )),
    }
}

fn run_register(client: Client, register: RegisterCommand) -> Result<()> {
    unimplemented!()
}

fn run_tag(client: Client, tag: TagCommand) -> Result<()> {
    use TagResourceCommand::*;
    match tag.resource {
        Transaction(tag_transaction) => run_tag_transaction(client, tag_transaction),
    }
}

fn run_tag_transaction(client: Client, tag: TagTransaction) -> Result<()> {
    let id = tag.transaction_id;
    let tags = tag.tags.clone();

    if tag.delete {
        client
            .transaction
            .delete_tag(&id, tags.clone())
            .with_context(|| format!("Failed to delete tags {:?} on transaction {}", tags, id))
    } else {
        client
            .transaction
            .tag(&id, tags.clone())
            .with_context(|| format!("Failed to add tags {:?} on transaction {}", tags, id))
    }
}

fn truncate(msg: String, max_length: usize) -> String {
    if msg.len() <= max_length {
        msg
    } else {
        let mut truncated = msg;
        truncated.truncate(max_length);
        truncated.push_str("...");
        truncated
    }
}
