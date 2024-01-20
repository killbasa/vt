use anyhow::Result;
use clap::Args;

use crate::app;

/// List channel channels
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        if config.orgs.is_none() || config.orgs.as_ref().unwrap().is_empty() {
            println!("No orgs set");
            return Ok(());
        }

        let mut list = Vec::<String>::new();

        if let Some(orgs) = config.orgs {
            for (k, v) in orgs.iter() {
                list.push(format!("{} -> {}", k, v));
            }
        }

        println!("Organizations:\n{}", list.join("\n"));

        Ok(())
    }
}
