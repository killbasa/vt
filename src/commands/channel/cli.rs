use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage channels
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
    Info(super::info::Cli),
    List(super::list::Cli),
    Rename(super::rename::Cli),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Create(cli) => cli.run(),
            Commands::Delete(cli) => cli.run(),
            Commands::Info(cli) => cli.run(),
            Commands::List(cli) => cli.run(),
            Commands::Rename(cli) => cli.run(),
        }
    }
}
