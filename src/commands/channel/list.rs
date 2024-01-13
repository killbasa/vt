use anyhow::Result;
use clap::Args;

use crate::app;

/// List channel aliases
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        if config.aliases.is_none() || config.aliases.as_ref().unwrap().is_empty() {
            println!("No channels set");
            return Ok(());
        }

        let mut channels = Vec::<String>::new();

        if let Some(aliases) = config.aliases {
            for (k, v) in aliases.iter() {
                channels.push(format!("{} -> {}", k, v));
            }
        }

        println!("Channels:\n{}", channels.join("\n"));

        Ok(())
    }
}
