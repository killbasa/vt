use anyhow::Result;
use clap::Args;
use vt_config::config;

use crate::app;

/// Rename a channel
#[derive(Args, Debug)]
pub struct Cli {
    /// The alias for the channel
    alias: String,
    /// The new alias for the channel
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
