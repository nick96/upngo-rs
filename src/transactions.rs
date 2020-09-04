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
