pub mod channels;
mod internal;
pub mod videos;

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

pub const CLI_USER_AGENT: &str = "vt-client";

pub const WEB_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:137.0) Gecko/20100101 Firefox/137.0";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct YoutubeChannel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub custom_url: Option<String>,
    pub view_count: String,
    pub subscriber_count: String,
    pub video_count: String,
}

impl Ord for YoutubeChannel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for YoutubeChannel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YoutubeVideo {
    pub id: String,
    pub channel: String,
    pub title: String,
    pub start_time: Option<String>,
    pub scheduled_time: String,
}
