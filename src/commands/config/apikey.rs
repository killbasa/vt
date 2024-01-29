use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Set a YouTube API key
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut secrets = app::secrets().clone();

        let apikey = rpassword::prompt_password("Your YouTube API key: ")?;

        secrets.apikey = Some(apikey);
        config::save_secrets(secrets)?;

        println!("YouTube API key saved");

        Ok(())
    }
}
