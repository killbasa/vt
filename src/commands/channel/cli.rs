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
    Set(super::set::Cli),
    Ls(super::ls::Cli),
    Rm(super::rm::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Set(cli) => cli.exec(),
            Commands::Ls(cli) => cli.exec(),
            Commands::Rm(cli) => cli.exec(),
        }
    }
}
