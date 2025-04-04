use anyhow::Result;
use clap::Args;
use std::collections::HashMap;
use vt_config::config;

use crate::app;

/// Set a channel
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The alias for the channel
    alias: String,
    /// The channel ID
    channel_id: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        if config.channels.is_none() {
            config.channels = Some(HashMap::new());
        }

        if let Some(mut channels) = config.channels.clone() {
            if channels.contains_key(&self.alias) {
                println!("Channel already exists");
                return Ok(());
            }

            channels.insert(self.alias.clone(), self.channel_id.clone());
            config.channels = Some(channels);
        }

        config::save_config(config)?;

        println!("Channel added: {} -> {}", &self.alias, &self.channel_id);

        Ok(())
    }
}
