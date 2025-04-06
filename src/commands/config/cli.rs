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
    Show(super::show::Cli),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Apikey(cli) => cli.run(),
            Commands::Find(cli) => cli.run(),
            Commands::Show(cli) => cli.run(),
        }
    }
}
