use anyhow::{Result, anyhow};
use clap::Args;
use vt_common::{display, utils, youtube};

use crate::app;

/// Get the live or upcoming streams of the channels in a group
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The group name
    group: String,
    /// Show the output in JSON format
    #[arg(long)]
    json: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let channels = match app::config().clone().channels {
            Some(c) => c,
            None => {
                return Err(anyhow!("no channels found"));
            }
        };

        let group = match app::get_group(&self.group) {
            Some(l) => l,
            None => {
                return Err(anyhow!("group not found"));
            }
        };

        let mut video_ids = Vec::<String>::new();

        for channel in group {
            let alias = match channels.get(&channel) {
                Some(c) => c,
                None => {
                    return Err(anyhow!("channel not found"));
                }
            };

            let ids = youtube::get_video_ids_xml(alias)
                .map_err(|e| anyhow!("failed to fetch video IDs ({}): {}", &alias, e))?;

            for id in ids {
                video_ids.push(id);
            }
        }

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
