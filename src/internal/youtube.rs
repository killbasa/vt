use anyhow::{anyhow, Result};
use reqwest::{
    blocking::ClientBuilder,
    header::{ACCEPT, USER_AGENT},
};
use serde::{Deserialize, Serialize};

use super::{headers, xml};
use crate::app;

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
    scheduled_start_time: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RawVideo {
    id: String,
    snippet: Snippet,
    live_streaming_details: LiveStreamingDetails,
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
        .get(format!(
            "https://www.youtube.com/feeds/videos.xml?channel_id={}",
            alias
        ))
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

            if video_id.is_some() {
                video_ids.push(video_id.unwrap());
            }
        }
    }

    Ok(video_ids)
}

/**
 * Fetches videos from the YouTube API
 */
pub fn get_videos_api(video_ids: &Vec<String>) -> Result<Vec<Video>> {
    let apikey = match app::secrets().clone().apikey {
        Some(apikey) => apikey,
        None => {
            return Err(anyhow!(
                "API key not found. Use `vt config apikey` to set an API key."
            ));
        }
    };

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
            let end_time = raw_video.live_streaming_details.actual_end_time;

            if end_time.is_none() {
                let start_time = raw_video.live_streaming_details.actual_start_time;
                let scheduled_time = raw_video.live_streaming_details.scheduled_start_time;

                videos.push(Video {
                    id: raw_video.id,
                    channel: raw_video.snippet.channel_title,
                    title: raw_video.snippet.title,
                    start_time,
                    scheduled_time,
                });
            }
        }
    }

    Ok(videos)
}
