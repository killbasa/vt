use anyhow::{Result, anyhow};
use clap::Args;
use vt_common::youtube::YoutubeChannel;
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

        let mut channels: Vec<&YoutubeChannel> = config.channels.values().collect();
        channels.sort();

        for channel in channels {
            println!("{} -> {}", channel.name, channel.id);
        }

        Ok(())
    }
}
