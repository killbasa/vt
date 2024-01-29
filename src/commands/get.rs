use anyhow::{anyhow, Result};
use clap::Args;

use crate::{
    app,
    internal::{display, utils, youtube},
};

/// Get a channel's live or upcoming streams
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

        let video_ids = match alias {
            Some(alias) => youtube::get_video_ids_xml(&alias)
                .map_err(|e| anyhow!("failed to fetch video IDs: {}", e))?,
            None => {
                return Err(anyhow!("channel not found"));
            }
        };

        let mut videos = youtube::get_videos_api(&video_ids)
            .map_err(|e| anyhow!("failed to fetch videos: {}", e))?;

        if self.json {
            if videos.is_empty() {
                display::with_print("[]");
                return Ok(());
            }

            let json = serde_json::to_string(&videos)
                .map_err(|e| anyhow!("failed to serialize JSON: {}", e))?;

            display::with_print(&json);
            return Ok(());
        } else if videos.is_empty() {
            println!("No live or upcoming streams");
            return Ok(());
        }

        videos.sort_by(|a, b| a.scheduled_time.cmp(&b.scheduled_time));

        let content = utils::format_videos(videos, false);
        display::with_print(&content);

        Ok(())
    }
}
