use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;
use colored::Colorize;
use reqwest::{
    blocking::ClientBuilder,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
use serde::{Deserialize, Serialize};

use crate::app;

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub available_at: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FetchOptions {
    pub channel: Option<String>,
    pub org: Option<String>,
}

pub fn fetch_feed(options: FetchOptions) -> Result<Vec<Video>> {
    let config = app::config().clone();
    let apikey: &str = &config.holodex_key.unwrap().clone();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert("X-APIKEY", HeaderValue::from_str(apikey)?);

    let mut params = HashMap::<&str, &str>::new();
    params.insert("max_upcoming_hours", "168");

    if let Some(ref alias) = options.channel {
        params.insert("channel_id", alias);
    }

    if let Some(ref org) = options.org {
        params.insert("org", org);
    }

    let client = ClientBuilder::new()
        .build()? //
        .get("https://holodex.net/api/v2/live")
        .query(&params)
        .headers(headers);

    Ok(client.send()?.json()?)
}

pub fn format_videos(videos: Vec<Video>) -> String {
    let mut video_list = Vec::<String>::new();

    for video in videos {
        let url = format!("https://www.youtube.com/watch?v={}", video.id);
        let published = DateTime::parse_from_rfc3339(&video.available_at)
            .unwrap()
            .with_timezone(&Local);
        let humanized = HumanTime::from(published).to_string();

        let entry = format!(
            "{}\n   ├─ url:       {}\n   └─ available: {} ({})\n",
            video.title.cyan(),
            url.bright_green(),
            published
                .format("%d/%m/%Y %I:%M%P")
                .to_string()
                .bright_green(),
            humanized.bright_green()
        );

        video_list.push(entry);
    }

    video_list.join("\n")
}
