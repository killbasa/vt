use anyhow::Result;
use chrono::DateTime;
use chrono_humanize::HumanTime;
use clap::Args;
use colored::Colorize;
use reqwest::blocking::ClientBuilder;
use roxmltree::Node;
use serde::Serialize;

use crate::{app, internal::display};

#[derive(Serialize)]
struct Video {
    id: String,
    title: String,
    published: String,
}

/// Get a channel's videos
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    channel: String,
    #[arg(short, long)]
    limit: Option<u8>,
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

        let limit = self.limit.unwrap_or(5).min(15);
        let body = fetch_feed(alias.unwrap())?;

        let mut video_data = Vec::<Video>::new();
        let document = roxmltree::Document::parse(&body)?;

        for entry_node in document.descendants().filter(|n| n.has_tag_name("entry")) {
            if video_data.len() >= limit as usize {
                break;
            }

            let video_id = get_property(&entry_node, "videoId");
            let video_title = get_property(&entry_node, "title");
            let video_published = get_property(&entry_node, "published");

            video_data.push(Video {
                id: video_id.unwrap(),
                title: video_title.unwrap(),
                published: video_published.unwrap(),
            });
        }

        if self.json {
            println!("{}", serde_json::to_string(&video_data)?);
            return Ok(());
        }

        display::with_print(format_videos(video_data));

        Ok(())
    }
}

fn fetch_feed(alias: String) -> Result<String> {
    let client = ClientBuilder::new()
        .build()? //
        .get(format!(
            "https://www.youtube.com/feeds/videos.xml?channel_id={}",
            alias
        ))
        .header(
            "User-agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0",
        );

    Ok(client.send()?.text()?)
}

fn get_property(node: &Node, property: &str) -> Option<String> {
    if let Some(res) = node.children().find(|n| n.has_tag_name(property)) {
        if let Some(res2) = res.text() {
            return Some(res2.to_string());
        }
    }

    None
}

fn format_videos(videos: Vec<Video>) -> String {
    let mut video_list = Vec::<String>::new();

    for video in videos {
        let url = format!("https://www.youtube.com/watch?v={}", video.id);
        let published = DateTime::parse_from_rfc3339(&video.published).unwrap();
        let humanized = HumanTime::from(published).to_string();

        let entry = format!(
            "{}\n   ├─ url:       {}\n   └─ published: {}\n",
            video.title.cyan(),
            url.bright_green(),
            humanized.bright_green()
        );

        video_list.push(entry);
    }

    video_list.join("\n")
}
