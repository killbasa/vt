mod app;
mod commands;

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
    Tui(commands::tui::Cli),
    Video(commands::video::cli::Cli),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    config::init()?;
    secrets::init()?;

    match &cli.command {
        Commands::Channel(cli) => cli.exec(),
        Commands::Check(cli) => cli.exec(),
        Commands::Complete(cli) => cli.exec(),
        Commands::Config(cli) => cli.exec(),
        Commands::Groups(cli) => cli.exec(),
        Commands::Tui(cli) => cli.exec(),
        Commands::Video(cli) => cli.exec(),
    }
}
