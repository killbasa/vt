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
    /// The name for the channel
    channel: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = config::get().clone();

        let mut group = match config.groups.get(&self.group) {
            Some(group) => group.clone(),
            None => {
                return Err(anyhow!("group not found"));
            }
        };

        let name = match app::get_channel(&self.channel) {
            Some(_) => &self.channel,
            None => {
                return Err(anyhow!("channel not found"));
            }
        };

        match group.insert(name.clone()) {
            true => {
                config.groups.insert(self.group.clone(), group);
                config::save(config)?;

                println!("added channel \"{}\" to group \"{}\"", &self.channel, &self.group);
            }
            false => {
                return Err(anyhow!(
                    "channel \"{}\" is already in group \"{}\"",
                    &self.channel,
                    &self.group
                ));
            }
        };

        Ok(())
    }
}
