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
#[command(alias = "ch")]
enum Commands {
    Add(super::add::Cli),
    List(super::list::Cli),
    Move(super::_move::Cli),
    Remove(super::remove::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Add(cli) => cli.exec(),
            Commands::List(cli) => cli.exec(),
            Commands::Move(cli) => cli.exec(),
            Commands::Remove(cli) => cli.exec(),
        }
    }
}
