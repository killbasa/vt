use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

use crate::app;

/// Add a channel to a group
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The group to add the channel to
    group: String,
    /// The alias for the channel
    alias: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();
        let mut groups = config.groups.unwrap_or_default();

        let mut group = match groups.get(&self.group) {
            Some(group) => group.clone(),
            None => {
                return Err(anyhow!("group not found"));
            }
        };

        let alias = match app::get_channel(&self.alias) {
            Some(_) => &self.alias,
            None => {
                return Err(anyhow!("channel not found"));
            }
        };

        match group.insert(alias.clone()) {
            true => {
                groups.insert(self.group.clone(), group);
                config.groups = Some(groups);
                config::save_config(config)?;

                println!("Added {} to {}", &self.alias, &self.group);
            }
            false => println!("Channel already exists in group"),
        };

        Ok(())
    }
}
