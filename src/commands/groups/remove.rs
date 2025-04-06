use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

/// Remove a channel from a group
#[derive(Args, Debug)]
pub struct Cli {
    /// The group to remove the channel from
    group: String,
    /// The name of the channel
    channel: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut config = config::get().clone();

        let group = config //
            .groups
            .get_mut(&self.group)
            .ok_or_else(|| anyhow!("group not found"))?;

        if group.remove(&self.channel) {
            config::save(config)?;

            println!("removed \"{}\" from \"{}\"", &self.channel, &self.group);
        } else {
            return Err(anyhow!("\"{}\" was not in group \"{}\"", &self.channel, &self.group));
        }

        Ok(())
    }
}
