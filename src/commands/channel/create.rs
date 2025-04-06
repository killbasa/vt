use anyhow::{Result, anyhow};
use clap::Args;
use vt_common::youtube::YoutubeChannel;
use vt_config::config::{self};

/// Create a channel
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The name for the channel
    name: String,
    /// The channel ID
    channel_id: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut config = config::get().clone();

        if config.channels.contains_key(&self.name) {
            return Err(anyhow!("channel already exists"));
        }

        let channel = YoutubeChannel {
            name: self.name.clone(), //
            id: self.channel_id.clone(),
        };

        config.channels.insert(self.name.clone(), channel);

        config::save(config)?;

        println!("channel created: {} -> {}", &self.name, &self.channel_id);

        Ok(())
    }
}
