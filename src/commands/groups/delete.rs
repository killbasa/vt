use anyhow::{Result, anyhow};
use clap::Args;
use inquire::Confirm;
use vt_config::config;

/// Delete a group
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The name of the group to delete
    group: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut config = config::get().clone();

        if config.groups.is_empty() {
            return Err(anyhow!("there are no groups to delete"));
        }

        if !config.groups.contains_key(&self.group) {
            return Err(anyhow!("group not found"));
        }

        let confirmation =
            Confirm::new(&format!("are you sure you want to delete \"{}\"?", &self.group))
                .with_default(false)
                .prompt()?;
        if !confirmation {
            return Ok(());
        }

        config.groups.remove(&self.group);
        config::save(config)?;

        println!("group deleted: {}", &self.group);

        Ok(())
    }
}
