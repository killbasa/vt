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
    List(super::list::Cli),
    Rename(super::rename::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Create(cli) => cli.exec(),
            Commands::Delete(cli) => cli.exec(),
            Commands::List(cli) => cli.exec(),
            Commands::Rename(cli) => cli.exec(),
        }
    }
}
