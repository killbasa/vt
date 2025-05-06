use anyhow::Result;
use clap::Args;
use vt_common::{display, youtube::channels};
use vt_config::utils;

use crate::internal::utils::format_channel;

/// Get information about a channel
#[derive(Args, Debug)]
pub struct Cli {
    handle: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let apikey = utils::get_apikey()?;

        let channel = channels::get_channel_api(&apikey, &self.handle)?;
        let content = format_channel(&channel);
        display::with_print(&content);

        Ok(())
    }
}
