use anyhow::Result;
use clap::Args;

/// Display the TUI
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        vt_tui::init()?;

        Ok(())
    }
}
