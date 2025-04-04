use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

use crate::app;

/// Remove a channel from a group
#[derive(Args, Debug)]
pub struct Cli {
    /// The group to remove the channel from
    group: String,
    /// The alias for the channel
    alias: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();
        let groups = config.groups.get_or_insert_with(Default::default);

        let group = groups
            .get_mut(&self.group)
            .ok_or_else(|| anyhow!("group '{}' not found", &self.group))?;

        if group.remove(&self.alias) {
            config::save_config(config)?;
            println!("Removed '{}' from '{}'", &self.alias, &self.group);
        } else {
            println!("'{}' was not in group '{}'", &self.alias, &self.group);
        }

        Ok(())
    }
}
