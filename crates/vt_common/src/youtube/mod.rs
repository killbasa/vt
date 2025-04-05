mod internal;
pub mod videos;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct YoutubeChannel {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YoutubeVideo {
    pub id: String,
    pub channel: String,
    pub title: String,
    pub start_time: Option<String>,
    pub scheduled_time: String,
}
