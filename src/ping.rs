
use argh::FromArgs;

/// Ping subcommand CLI interface.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ping")]
pub struct Ping {}
