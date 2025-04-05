use anyhow::{Result, anyhow};
use clap::Args;
use std::io;
use vt_config::config;

/// Delete a channel
#[derive(Args, Debug)]
pub struct Cli {
    /// The name for the channel
    name: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = config::get().clone();

        if config.channels.is_empty() {
            return Err(anyhow!("there are no channels to delete"));
        }

        if !config.channels.contains_key(&self.name) {
            return Err(anyhow!("channel not found"));
        }

        // Confirmation prompt
        print!("are you sure you want to delete the channel {}? (y/N) ", &self.name);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input != "y" {
            println!("channel deletion cancelled");
            return Ok(());
        }

        config.channels.remove(&self.name);

        // Remove the channel from all groups
        for (_, group) in config.groups.iter_mut() {
            group.remove(&self.name);
        }

        config::save(config)?;

        println!("channel deleted: {}", &self.name);

        Ok(())
    }
}
