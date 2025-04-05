use anyhow::{Result, anyhow};
use clap::Args;
use vt_common::{display, youtube};
use vt_config::config;

use crate::{app, commands::internal::utils};

/// Check the live or upcoming streams of the channels in a group
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The group name
    group: String,
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
            println!("--- checking group ---\n{}", self.group.clone());
        }

        let channels = config::get().clone().channels;
        if self.verbose {
            println!(
                "--- checking channels ---\n{}",
                channels
                    .clone()
                    .into_iter()
                    .map(|(k, v)| format!("{} ({})", k, v.id))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }

        let group = match app::get_group(&self.group) {
            Some(l) => l,
            None => {
                return Err(anyhow!("group not found"));
            }
        };

        let mut video_ids = Vec::<String>::new();

        for channel_name in group {
            let channel = match channels.get(&channel_name) {
                Some(c) => c,
                None => {
                    return Err(anyhow!("channel not found"));
                }
            };

            let ids = youtube::videos::get_video_ids_xml(&channel.id)
                .map_err(|e| anyhow!("failed to fetch video IDs ({}): {}", &channel_name, e))?;

            for id in ids {
                video_ids.push(id);
            }
        }

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
