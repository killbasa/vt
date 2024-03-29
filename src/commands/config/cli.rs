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
    Apikey(super::apikey::Cli),
    Find(super::find::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Apikey(cli) => cli.exec(),
            Commands::Find(cli) => cli.exec(),
        }
    }
}
