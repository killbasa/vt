use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Set a channel alias
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    alias: String,
    channel_id: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        if config.channels.is_none() {
            config.channels = Some(std::collections::HashMap::new());
        }

        if let Some(mut channels) = config.channels.clone() {
            if channels.contains_key(&self.alias) {
                println!("Channel alias already exists");
                return Ok(());
            }

            channels.insert(self.alias.clone(), self.channel_id.clone());
            config.channels = Some(channels);
        }

        config::save(config)?;

        println!(
            "Channel alias added: {} -> {}",
            self.alias.clone(),
            self.channel_id.clone()
        );

        Ok(())
    }
}
