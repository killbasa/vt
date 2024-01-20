use anyhow::Result;
use clap::Args;
use std::fs;

use crate::{
    app,
    internal::{display, holodex},
};

/// Check the config path
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    channel: String,
    #[arg(long)]
    json: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let alias = app::get_channel(&self.channel);

        if alias.is_none() {
            println!("Channel alias not found. Use `vt channel add` to add a channel alias.");
            return Ok(());
        }

        let body = holodex::fetch_feed(holodex::FetchOptions {
            channel: Some(alias.unwrap()),
            org: None,
        })?;

        let json_string = serde_json::to_string(&body)?;

        fs::write("test.json", &json_string)?;

        if self.json {
            println!("{}", json_string);
            return Ok(());
        }

        display::with_print(holodex::format_videos(body));

        Ok(())
    }
}
