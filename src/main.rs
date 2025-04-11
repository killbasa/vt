mod app;
mod commands;
mod internal;

use anyhow::Result;
use clap::{Parser, Subcommand};
use vt_config::{config, secrets};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "Check live and upcoming YouTube streams from your terminal")]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Channel(commands::channel::cli::Cli),
    Check(commands::check::Cli),
    Complete(commands::complete::Cli),
    Config(commands::config::cli::Cli),
    Groups(commands::groups::cli::Cli),
    Update(commands::update::Cli),
    Video(commands::video::cli::Cli),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    config::init()?;
    secrets::init()?;

    match &cli.command {
        Commands::Channel(cli) => cli.run(),
        Commands::Check(cli) => cli.run(),
        Commands::Complete(cli) => cli.run(),
        Commands::Config(cli) => cli.run(),
        Commands::Groups(cli) => cli.run(),
        Commands::Update(cli) => cli.run(),
        Commands::Video(cli) => cli.run(),
    }
}
