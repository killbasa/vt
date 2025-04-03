use anyhow::Result;
use clap::Args;
use vt_ui::ui;

/// Display the TUI
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        ui::init()?;

        Ok(())
    }
}
