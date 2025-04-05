use anyhow::{Result, anyhow};
use clap::Args;
use vt_common::{display, youtube};

use crate::app;

use super::internal::utils;

/// Get a channel's live or upcoming streams
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The channel name
    channel: String,
    /// Show the output in JSON format
    #[arg(long)]
    json: bool,
    /// Show verbose output
    #[clap(long = "verbose", short = 'v')]
    verbose: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        if self.verbose {
            println!("--- checking channel ---\n{}", self.channel.clone());
        }

        let channel = app::get_channel(&self.channel);

        let video_ids = match channel {
            Some(_channel) => youtube::videos::get_video_ids_xml(&_channel.id)
                .map_err(|e| anyhow!("failed to fetch video IDs ({}): {}", &_channel.id, e))?,
            None => {
                println!("channel \"{}\" not found", &self.channel);
                return Ok(());
            }
        };

        if self.verbose {
            println!("--- checking videos ---\n{}", video_ids.clone().join("\n"));
        }

        let apikey = vt_config::utils::get_apikey()?;
        let mut videos = youtube::videos::get_videos_api(&apikey, &video_ids)
            .map_err(|e| anyhow!("failed to fetch videos: {}", e))?;

        if self.verbose {
            println!("--- video check result ---\n{} videos", videos.len());
        }

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
            println!("no live or upcoming streams");
            return Ok(());
        }

        videos.sort_by(|a, b| a.scheduled_time.cmp(&b.scheduled_time));

        let content = utils::format_videos(videos, false);
        display::with_print(&content);

        Ok(())
    }
}
