use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::{channel::VTChannel, config};

/// List channels
#[derive(Args, Debug)]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let config = config::get();

        if config.channels.is_empty() {
            return Err(anyhow!("there are no channels to list"));
        }

        let mut channels: Vec<&VTChannel> = config.channels.values().collect();
        channels.sort();

        for channel in channels {
            println!("{} -> {}", channel.name, channel.id);
        }

        Ok(())
    }
}
