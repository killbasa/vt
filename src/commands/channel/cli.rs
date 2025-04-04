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
    List(super::list::Cli),
    Remove(super::remove::Cli),
    Rename(super::rename::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Add(cli) => cli.exec(),
            Commands::List(cli) => cli.exec(),
            Commands::Remove(cli) => cli.exec(),
            Commands::Rename(cli) => cli.exec(),
        }
    }
}
