use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage groups
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add(super::add::Cli),
    Check(super::check::Cli),
    Create(super::create::Cli),
    Delete(super::delete::Cli),
    List(super::list::Cli),
    Remove(super::remove::Cli),
    Rename(super::rename::Cli),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Add(cli) => cli.run(),
            Commands::Check(cli) => cli.run(),
            Commands::Create(cli) => cli.run(),
            Commands::Delete(cli) => cli.run(),
            Commands::List(cli) => cli.run(),
            Commands::Remove(cli) => cli.run(),
            Commands::Rename(cli) => cli.run(),
        }
    }
}
