mod app;
mod commands;
mod config;
mod internal;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "Check past and upcoming YouTube streams")]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Channel(commands::channel::cli::Cli),
    Complete(commands::complete::Cli),
    Config(commands::config::cli::Cli),
    Get(commands::get::Cli),
    Holodex(commands::holodex::cli::Cli),
    List(commands::list::Cli),
    Org(commands::org::cli::Cli),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    app::set_global_config(config::load()?);

    match &cli.command {
        Commands::Channel(cli) => cli.exec(),
        Commands::Complete(cli) => cli.exec(),
        Commands::Config(cli) => cli.exec(),
        Commands::Get(cli) => cli.exec(),
        Commands::Holodex(cli) => cli.exec(),
        Commands::List(cli) => cli.exec(),
        Commands::Org(cli) => cli.exec(),
    }
}
