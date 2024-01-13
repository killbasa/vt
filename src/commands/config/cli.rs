use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage the config file
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Find(super::find::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Find(cli) => cli.exec(),
        }
    }
}
