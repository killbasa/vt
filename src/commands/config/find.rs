use anyhow::Result;
use clap::Args;
use vt_config::config;

/// Check the config path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        println!("{}", config::path()?.display());

        Ok(())
    }
}
