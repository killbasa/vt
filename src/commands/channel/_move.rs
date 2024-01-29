use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Change a channel
#[derive(Args, Debug)]
#[command(alias = "mv")]
pub struct Cli {
    alias: String,
    new_alias: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        match config.channels {
            Some(mut channels) => {
                if let Some(channel) = channels.get(&self.alias) {
                    channels.insert(self.new_alias.clone(), channel.clone());
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

        println!("Channel moved: {} -> {}", &self.alias, &self.new_alias);

        Ok(())
    }
}
