use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Set a YouTube API key
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        let apikey = rpassword::prompt_password("Your YouTube API key: ")?;

        config.apikey = Some(apikey);
        config::save(config)?;

        println!("YouTube API key saved");

        Ok(())
    }
}
