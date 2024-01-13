use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Add a channel alias
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    alias: String,
    channel_id: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        if config.aliases.is_none() {
            config.aliases = Some(std::collections::HashMap::new());
        }

        if let Some(mut aliases) = config.aliases.clone() {
            if aliases.contains_key(&self.alias) {
                println!("Channe alias already exists");
                return Ok(());
            }

            aliases.insert(self.alias.clone(), self.channel_id.clone());
            config.aliases = Some(aliases);
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
