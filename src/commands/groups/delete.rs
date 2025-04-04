use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

use crate::app;

// TODO - add confirmation prompt

/// Delete a group
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The name of the group to delete
    group: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        match config.groups {
            Some(mut groups) => {
                if groups.contains_key(&self.group) {
                    groups.remove(&self.group);
                    config.groups = Some(groups);
                } else {
                    return Err(anyhow!("group not found"));
                }
            }
            None => {
                return Err(anyhow!("there are no groups to delete"));
            }
        }

        config::save_config(config)?;

        println!("Group deleted: {}", &self.group);

        Ok(())
    }
}
