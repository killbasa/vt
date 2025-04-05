use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

/// List channels
#[derive(Args, Debug)]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = config::get().clone();

        if config.channels.is_empty() {
            return Err(anyhow!("there are no channels to list"));
        }

        for (k, v) in config.channels.iter() {
            println!("{} -> {}", k, v.id);
        }

        Ok(())
    }
}
