use anyhow::Result;

use crate::app;

pub fn init() -> Result<()> {
    let terminal = ratatui::init();
    let app_result = app::run(terminal);
    ratatui::restore();

    app_result
}
