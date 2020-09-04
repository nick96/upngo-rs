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
