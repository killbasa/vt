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
    Create(super::create::Cli),
    Delete(super::delete::Cli),
    Get(super::get::Cli),
    Set(super::set::Cli),
    Rm(super::rm::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Create(cli) => cli.exec(),
            Commands::Delete(cli) => cli.exec(),
            Commands::Get(cli) => cli.exec(),
            Commands::Rm(cli) => cli.exec(),
            Commands::Set(cli) => cli.exec(),
        }
    }
}
