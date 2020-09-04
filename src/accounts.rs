use crate::Result;
use crate::client::Client;
use argh::FromArgs;
use prettytable::*;

/// Get accounts by their ID.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "get")]
pub struct Get {
    #[argh(positional)]
    id: String,
}

impl Get {
    pub fn run(&self, client: Client) -> Result<()> {
        todo!()
    }
}

/// List all accounts.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
pub struct List {}

impl List {
    pub fn run(&self, client: Client) -> Result<()> {
        let accounts = client.accounts.list()?;
        let mut table = Table::new();
        for account in accounts.data {
            table.add_row(
                row![
                   format!("{}", account.attributes.display_name),
                   format!("{}", account.attributes.account_type),
                   format!("{}", account.attributes.balance),
                   format!("{}", account.attributes.created_at),
                   format!("{}", account.id),
                ]
            );
        }
        Ok(())
    }
}

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
    pub subcommands: Subcommands,
}
