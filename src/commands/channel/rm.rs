use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Remove a channel alias
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    alias: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        match config.channels {
            Some(mut channels) => {
                if channels.contains_key(&self.alias) {
                    channels.remove(&self.alias);
                    config.channels = Some(channels);
                } else {
                    println!("Channel alias not found");
                    return Ok(());
                }
            }
            None => {
                println!("There are no channels set");
                return Ok(());
            }
        }

        config::save(config)?;

        println!("Channel alias remove: {}", self.alias.clone());

        Ok(())
    }
}
