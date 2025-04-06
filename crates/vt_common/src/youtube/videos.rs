use anyhow::{Result, anyhow};
use reqwest::{
    blocking::ClientBuilder,
    header::{ACCEPT, USER_AGENT},
};

use crate::xml;

use super::{
    YoutubeVideo,
    internal::{ApiResponse, RawYoutubeVideo},
};

pub const WEB_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:137.0) Gecko/20100101 Firefox/137.0";

pub const CLI_USER_AGENT: &str = "vt-client";

/**
 * Fetches video IDs from a channel's XML page
 */
pub fn get_video_ids_xml(channel_id: &str) -> Result<Vec<String>> {
    let client = ClientBuilder::new()
        .build()? //
        .get(format!("https://www.youtube.com/feeds/videos.xml?channel_id={}", channel_id))
        .header(USER_AGENT, WEB_USER_AGENT);

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
 * Fetches a video from the YouTube API
 */
pub fn get_video_api(apikey: &str, video_id: &String) -> Result<YoutubeVideo> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=snippet,liveStreamingDetails&key={}&id={}",
        apikey, video_id
    );

    let client = ClientBuilder::new()
        .build()? //
        .get(url)
        .header(USER_AGENT, CLI_USER_AGENT)
        .header(ACCEPT, "application/json");

    let response = client.send()?;
    if response.status().as_u16() != 200 {
        return Err(anyhow!(response.status()));
    }

    let body: ApiResponse = response.json()?;

    for raw_video in body.items {
        if let Some(video) = process_raw_video(raw_video) {
            return Ok(video);
        }
    }

    Err(anyhow!("video not found"))
}

/**
 * Fetches videos from the YouTube API
 */
pub fn get_videos_api(apikey: &str, video_ids: &[String]) -> Result<Vec<YoutubeVideo>> {
    let mut videos = Vec::<YoutubeVideo>::new();

    for chunk in video_ids.chunks(50) {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?part=snippet,liveStreamingDetails&key={}&id={}",
            apikey,
            chunk.join(",")
        );

        let client = ClientBuilder::new()
            .build()? //
            .get(url)
            .header(USER_AGENT, CLI_USER_AGENT)
            .header(ACCEPT, "application/json");

        let response = client.send()?;
        if response.status().as_u16() != 200 {
            return Err(anyhow!(response.status()));
        }

        let body: ApiResponse = response.json()?;

        for raw_video in body.items {
            if let Some(video) = process_raw_video(raw_video) {
                videos.push(video);
            }
        }
    }

    Ok(videos)
}

fn process_raw_video(raw_video: RawYoutubeVideo) -> Option<YoutubeVideo> {
    if let Some(live) = raw_video.live_streaming_details {
        let end_time = live.actual_end_time;
        let scheduled_time = live.scheduled_start_time;

        if scheduled_time.is_some() && end_time.is_none() {
            let start_time = live.actual_start_time;

            return Some(YoutubeVideo {
                id: raw_video.id,
                channel: raw_video.snippet.channel_title,
                title: raw_video.snippet.title,
                start_time,
                scheduled_time: scheduled_time.unwrap(),
            });
        }
    }

    None
}
