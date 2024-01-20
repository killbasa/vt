use anyhow::Result;
use clap::Args;

use crate::{
    app,
    internal::{display, holodex},
};

/// Check the config path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    org: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let alias = app::get_org(&self.org);

        if alias.is_none() {
            println!("Org alias not found. Use `vt org add` to add an organization alias.");
            return Ok(());
        }

        let body = holodex::fetch_feed(holodex::FetchOptions {
            channel: None,
            org: alias.clone(),
        })?;

        let videos = holodex::format_videos(body);
        display::with_print(videos);

        Ok(())
    }
}
