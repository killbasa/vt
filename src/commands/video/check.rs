use anyhow::{Result, anyhow};
use clap::Args;
use vt_common::{display, youtube};

use crate::commands::internal::utils;

/// Check a video
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The channel name
    video_id: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let apikey = vt_config::utils::get_apikey()?;
        let video = youtube::videos::get_video_api(&apikey, &self.video_id)
            .map_err(|e| anyhow!("failed to fetch video: {}", e))?;

        let content = utils::format_videos(vec![video], true);
        display::with_print(&content);

        Ok(())
    }
}
