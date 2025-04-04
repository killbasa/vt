use anyhow::{Result, anyhow};
use clap::Args;
use vt_common::{display, utils, youtube};

use crate::app;

/// Get a channel's live or upcoming streams
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The channel name
    channel: String,
    /// Show the output in JSON format
    #[arg(long)]
    json: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let alias = app::get_channel(&self.channel);

        let video_ids = match alias {
            Some(alias) => youtube::get_video_ids_xml(&alias)
                .map_err(|e| anyhow!("failed to fetch video IDs ({}): {}", &alias, e))?,
            None => {
                println!("Channel \"{}\" not found", &self.channel);
                return Ok(());
            }
        };

        let apikey = match app::secrets().clone().apikey {
            Some(apikey) => apikey,
            None => {
                return Err(anyhow!(
                    "API key not found. Use `vt config apikey` to set an API key."
                ));
            }
        };

        let mut videos = youtube::get_videos_api(&apikey, &video_ids)
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
