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
    Add(super::add::Cli),
    Create(super::create::Cli),
    Delete(super::delete::Cli),
    Get(super::get::Cli),
    Remove(super::remove::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Add(cli) => cli.exec(),
            Commands::Create(cli) => cli.exec(),
            Commands::Delete(cli) => cli.exec(),
            Commands::Get(cli) => cli.exec(),
            Commands::Remove(cli) => cli.exec(),
        }
    }
}
