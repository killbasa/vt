use anyhow::Result;

use crate::app::run;

pub fn init() -> Result<()> {
    let terminal = ratatui::init();
    let app_result = run(terminal);
    ratatui::restore();

    app_result
}
