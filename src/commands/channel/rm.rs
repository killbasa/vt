use anyhow::Result;
use clap::Args;

/// Remove a channel alias
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        println!("not implemented");
        Ok(())
    }
}
