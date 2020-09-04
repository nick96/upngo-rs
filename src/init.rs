
use argh::FromArgs;
use keyring::Keyring;
use rpassword;

use crate::{Result, SERVICE_NAME, USERNAME};

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
