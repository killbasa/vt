use anyhow::{anyhow, Result};
use clap::Args;

use crate::{
    app::{self, config},
    internal::{display, utils, youtube},
};

/// Get a channel's live or upcoming streams
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    list: String,
    #[arg(long)]
    json: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let channels = match config().clone().channels {
            Some(c) => c,
            None => {
                return Err(anyhow!("no channels found"));
            }
        };

        let list = match app::get_list(&self.list) {
            Some(l) => l,
            None => {
                return Err(anyhow!("list not found"));
            }
        };

        let mut video_ids = Vec::<String>::new();

        for channel in list {
            let alias = match channels.get(&channel) {
                Some(c) => c,
                None => {
                    return Err(anyhow!("channel not found"));
                }
            };

            let ids = youtube::get_video_ids_xml(&alias)
                .map_err(|e| anyhow!("failed to fetch videos: {}", e))?;

            for id in ids {
                video_ids.push(id);
            }
        }

        let mut videos = youtube::get_videos_api(&video_ids)
            .map_err(|e| anyhow!("failed to fetch videos: {}", e))?;

        if self.json {
            if videos.is_empty() {
                display::with_print("[]");
                return Ok(());
            }

            let json = serde_json::to_string(&videos)
                .map_err(|e| anyhow!("failed to generate JSON: {}", e))?;

            display::with_print(&json);
            return Ok(());
        } else if videos.is_empty() {
            println!("No live or upcoming streams");
            return Ok(());
        }

        videos.sort_by(|a, b| a.scheduled_time.cmp(&b.scheduled_time));

        let content = utils::format_videos(videos, true);
        display::with_print(&content);

        Ok(())
    }
}
