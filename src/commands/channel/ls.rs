use anyhow::Result;
use clap::Args;

use crate::app;

/// List channels
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        if config.channels.is_none() || config.channels.as_ref().unwrap().is_empty() {
            println!("No channels set");
            return Ok(());
        }

        let mut list = Vec::<String>::new();

        if let Some(channels) = config.channels {
            for (k, v) in channels.iter() {
                list.push(format!("{} -> {}", k, v));
            }
        }

        println!("{}", list.join("\n"));

        Ok(())
    }
}
