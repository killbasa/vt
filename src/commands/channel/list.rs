use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

/// List channels
#[derive(Args, Debug)]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let config = config::get();

        if config.channels.is_empty() {
            return Err(anyhow!("there are no channels to list"));
        }

        for (name, channel) in config.channels.iter() {
            println!("{} -> {}", name, channel.id);
        }

        Ok(())
    }
}
