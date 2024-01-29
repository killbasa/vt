use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Remove a channel
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
                    println!("Channel not found");
                    return Ok(());
                }
            }
            None => {
                println!("There are no channels set");
                return Ok(());
            }
        }

        config::save_config(config)?;

        println!("Channel removed: {}", &self.alias);

        Ok(())
    }
}
