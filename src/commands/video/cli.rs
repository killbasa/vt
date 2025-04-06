use anyhow::Result;
use clap::{Args, Subcommand};

/// Video utilities
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check(super::check::Cli),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Check(cli) => cli.run(),
        }
    }
}
