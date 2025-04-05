use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

/// Rename a channel
#[derive(Args, Debug)]
pub struct Cli {
    /// The name for the channel
    name: String,
    /// The new name for the channel
    new_name: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = config::get().clone();

        if config.channels.is_empty() {
            return Err(anyhow!("there are no channels set"));
        }

        if config.channels.contains_key(&self.new_name) {
            return Err(anyhow!("channel with new name already exists"));
        }

        if self.name == self.new_name {
            return Err(anyhow!("existing name and new name are the same"));
        }

        if let Some(channel) = config.channels.get(&self.name) {
            config.channels.insert(self.new_name.clone(), channel.clone());
            config.channels.remove(&self.name);
        } else {
            return Err(anyhow!("channel not found"));
        }

        config::save(config)?;

        println!("channel renamed: {} -> {}", &self.name, &self.new_name);

        Ok(())
    }
}
