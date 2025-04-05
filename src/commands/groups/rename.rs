use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

/// Rename a group
#[derive(Args, Debug)]
pub struct Cli {
    /// The name for the group
    name: String,
    /// The new name for the group
    new_name: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = config::get().clone();

        if config.groups.is_empty() {
            return Err(anyhow!("there are no groups set"));
        }

        if config.channels.contains_key(&self.new_name) {
            return Err(anyhow!("group with new name already exists"));
        }

        if self.name == self.new_name {
            return Err(anyhow!("existing name and new name are the same"));
        }

        if let Some(group) = config.groups.get(&self.name) {
            config.groups.insert(self.new_name.clone(), group.clone());
            config.groups.remove(&self.name);
        } else {
            return Err(anyhow!("group not found"));
        }

        config::save(config)?;

        println!("group renamed: {} -> {}", &self.name, &self.new_name);

        Ok(())
    }
}
