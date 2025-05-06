mod app;

use anyhow::Result;

use crate::app::App;

pub fn init() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();

    result
}
