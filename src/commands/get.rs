use anyhow::{anyhow, Result};
use chrono::DateTime;
use chrono_humanize::HumanTime;
use clap::Args;
use colored::{ColoredString, Colorize};

use crate::{
    app,
    internal::{
        display,
        youtube::{self, Video},
    },
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
            Some(alias) => youtube::get_video_ids_xml(alias, None)
                .map_err(|e| anyhow!("Failed to fetch video IDs: {}", e))?,
            None => {
                println!("Channel alias not found. Use `vt channel add` to add a channel alias.");
                return Ok(());
            }
        };

        let mut videos = youtube::get_videos_api(&video_ids)
            .map_err(|e| anyhow!("Failed to fetch videos: {}", e))?;

        if self.json {
            if videos.is_empty() {
                display::with_print("[]");
                return Ok(());
            }

            let json = serde_json::to_string(&videos)
                .map_err(|e| anyhow!("Failed to generate JSON: {}", e))?;

            display::with_print(&json);
            return Ok(());
        } else if videos.is_empty() {
            println!("No live or upcoming streams");
            return Ok(());
        }

        videos.sort_by(|a, b| a.scheduled_time.cmp(&b.scheduled_time));

        let content = format_videos(videos);
        display::with_print(&content);

        Ok(())
    }
}

fn format_videos(videos: Vec<Video>) -> String {
    let mut video_list = Vec::<String>::new();

    for video in videos {
        let url = format!("https://www.youtube.com/watch?v={}", video.id);

        let status: ColoredString = match video.start_time.is_some() {
            true => "[live]".bright_red(),
            false => "[upcoming]".bright_yellow(),
        };

        let entry: String;
        if let Some(start_time) = video.start_time {
            let started = humanize_time(&start_time);
            entry = format!(
                "{} {}\n ├─     url: {}\n └─ started: {}\n",
                status,
                video.title.bright_cyan(),
                url.bright_green(),
                started.bright_green()
            );
        } else {
            let scheduled = humanize_time(&video.scheduled_time);
            entry = format!(
                "{} {}\n ├─       url: {}\n └─ scheduled: {}\n",
                status,
                video.title.bright_cyan(),
                url.bright_green(),
                scheduled.bright_green()
            );
        }

        video_list.push(entry);
    }

    video_list.join("\n")
}

fn humanize_time(time: &str) -> String {
    let parsed = DateTime::parse_from_rfc3339(time).unwrap();
    HumanTime::from(parsed).to_string()
}
