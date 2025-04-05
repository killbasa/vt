use anyhow::Result;
use clap::{Args, Subcommand};

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
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Add(cli) => cli.exec(),
            Commands::Check(cli) => cli.exec(),
            Commands::Create(cli) => cli.exec(),
            Commands::Delete(cli) => cli.exec(),
            Commands::List(cli) => cli.exec(),
            Commands::Remove(cli) => cli.exec(),
            Commands::Rename(cli) => cli.exec(),
        }
    }
}
