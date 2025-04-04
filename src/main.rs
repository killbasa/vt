mod app;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use vt_config::config;

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
    Complete(commands::complete::Cli),
    Config(commands::config::cli::Cli),
    Get(commands::get::Cli),
    Groups(commands::groups::cli::Cli),
    UI(commands::ui::Cli),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    app::set_global_config(config::load_config()?);
    app::set_global_secrets(config::load_secrets()?);

    match &cli.command {
        Commands::Channel(cli) => cli.exec(),
        Commands::Complete(cli) => cli.exec(),
        Commands::Config(cli) => cli.exec(),
        Commands::Get(cli) => cli.exec(),
        Commands::Groups(cli) => cli.exec(),
        Commands::UI(cli) => cli.exec(),
    }
}
