use anyhow::Result;
use vt_config::{config, secrets};
use vt_tui::init;

pub fn main() -> Result<()> {
    config::init()?;
    secrets::init()?;

    init()
}
