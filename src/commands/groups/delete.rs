use anyhow::{Result, anyhow};
use clap::Args;
use std::io;
use vt_config::config;

/// Delete a group
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The name of the group to delete
    group: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = config::get().clone();

        if config.groups.is_empty() {
            return Err(anyhow!("there are no groups to delete"));
        }

        if !config.groups.contains_key(&self.group) {
            return Err(anyhow!("group not found"));
        }

        // Confirmation prompt
        print!("are you sure you want to delete the group {}? (y/N) ", &self.group);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input != "y" {
            println!("group deletion cancelled");
            return Ok(());
        }

        config.groups.remove(&self.group);
        config::save(config)?;

        println!("group deleted: {}", &self.group);

        Ok(())
    }
}
