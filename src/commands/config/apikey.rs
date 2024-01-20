use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Check the config path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        let password = rpassword::prompt_password("Your password: ")?;

        config.holodex_key = Some(password);
        config::save(config)?;

        println!("Holodex API key saved");

        Ok(())
    }
}
