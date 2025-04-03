use anyhow::{Result, anyhow};
use reqwest::{
    blocking::ClientBuilder,
    header::{ACCEPT, USER_AGENT},
};
use serde::{Deserialize, Serialize};

use super::{headers, xml};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Snippet {
    title: String,
    channel_title: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LiveStreamingDetails {
    actual_start_time: Option<String>,
    actual_end_time: Option<String>,
    scheduled_start_time: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RawVideo {
    id: String,
    snippet: Snippet,
    live_streaming_details: Option<LiveStreamingDetails>,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    items: Vec<RawVideo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Video {
    pub id: String,
    pub channel: String,
    pub title: String,
    pub start_time: Option<String>,
    pub scheduled_time: String,
}

/**
 * Fetches video IDs from a channel's XML page
 */
pub fn get_video_ids_xml(alias: &str) -> Result<Vec<String>> {
    let client = ClientBuilder::new()
        .build()? //
        .get(format!("https://www.youtube.com/feeds/videos.xml?channel_id={}", alias))
        .header(USER_AGENT, headers::WEB_USER_AGENT);

    let response = client.send()?;
    if response.status().as_u16() != 200 {
        return Err(anyhow!(response.status()));
    }

    let body = response.text()?;
    let document = roxmltree::Document::parse(&body)?;
    let mut video_ids = Vec::<String>::new();

    for entry_node in document.descendants() {
        if entry_node.has_tag_name("entry") {
            let video_id = xml::get_property(&entry_node, "videoId");

            if let Some(video_id) = video_id {
                video_ids.push(video_id);
            }
        }
    }

    Ok(video_ids)
}

/**
 * Fetches videos from the YouTube API
 */
pub fn get_videos_api(apikey: &str, video_ids: &[String]) -> Result<Vec<Video>> {
    let mut videos = Vec::<Video>::new();

    for chunk in video_ids.chunks(50) {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?part=snippet,liveStreamingDetails&key={}&id={}",
            apikey,
            chunk.join(",")
        );

        let client = ClientBuilder::new()
            .build()? //
            .get(url)
            .header(USER_AGENT, headers::CLI_USER_AGENT)
            .header(ACCEPT, headers::APPLICATION_JSON);

        let response = client.send()?;
        if response.status().as_u16() != 200 {
            return Err(anyhow!(response.status()));
        }

        let body: ApiResponse = response.json()?;

        for raw_video in body.items {
            if let Some(live) = raw_video.live_streaming_details {
                let end_time = live.actual_end_time;
                let scheduled_time = live.scheduled_start_time;

                if scheduled_time.is_some() && end_time.is_none() {
                    let start_time = live.actual_start_time;

                    videos.push(Video {
                        id: raw_video.id,
                        channel: raw_video.snippet.channel_title,
                        title: raw_video.snippet.title,
                        start_time,
                        scheduled_time: scheduled_time.unwrap(),
                    });
                }
            }
        }
    }

    Ok(videos)
}
