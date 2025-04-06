use anyhow::{Result, anyhow};
use clap::Args;
use std::collections::HashSet;
use vt_config::config;

/// Create a group
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The name of the group to create
    group: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut config = config::get().clone();

        if config.groups.contains_key(&self.group) {
            return Err(anyhow!("group already exists"));
        }

        config.groups.insert(self.group.clone(), HashSet::new());
        config::save(config)?;

        println!("group created: {}", &self.group);

        Ok(())
    }
}
