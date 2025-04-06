mod internal;
pub mod videos;

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct YoutubeChannel {
    pub id: String,
    pub name: String,
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
