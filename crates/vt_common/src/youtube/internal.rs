use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub title: String,
    pub channel_title: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LiveStreamingDetails {
    pub actual_start_time: Option<String>,
    pub actual_end_time: Option<String>,
    pub scheduled_start_time: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawYoutubeVideo {
    pub id: String,
    pub snippet: Snippet,
    pub live_streaming_details: Option<LiveStreamingDetails>,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub items: Vec<RawYoutubeVideo>,
}
