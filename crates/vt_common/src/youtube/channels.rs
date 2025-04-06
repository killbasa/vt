use regex::Regex;

use anyhow::{Ok, Result, anyhow};
use reqwest::{
    blocking::ClientBuilder,
    header::{ACCEPT, USER_AGENT},
};

use super::{CLI_USER_AGENT, WEB_USER_AGENT, YoutubeChannel, internal::ChannelApiResponse};

pub fn get_channelid_html(username: &String) -> Result<String> {
    let url = format!("https://www.youtube.com/@{}", username);

    let client = ClientBuilder::new()
        .build()? //
        .get(url)
        .header(USER_AGENT, WEB_USER_AGENT);

    let response = client.send()?;
    if response.status().as_u16() != 200 {
        return Err(anyhow!(response.status()));
    }

    let body = response.text()?;

    let re = Regex::new(r#"\{"key":"browse_id","value":"(?<id>[^"\r\n}]+)"\}"#)?;

    if let Some(caps) = re.captures(&body) {
        let id = caps.name("id").unwrap().as_str();
        return Ok(id.to_string());
    }

    Err(anyhow!("id not found"))
}

// TODO - allow handles, ids, and usernames
pub fn get_channel_api(apikey: &str, handle: &String) -> Result<YoutubeChannel> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/channels?part=id,snippet,statistics&key={}&forHandle={}",
        apikey, handle
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

    let body: ChannelApiResponse = response.json()?;

    if body.items.is_empty() {
        return Err(anyhow!("channel not found"));
    }

    let raw_channel = body.items[0].to_owned();

    let channel = YoutubeChannel {
        id: raw_channel.id,
        name: raw_channel.snippet.title,
        description: raw_channel.snippet.description,
        custom_url: raw_channel.snippet.custom_url,
        view_count: raw_channel.statistics.view_count,
        subscriber_count: raw_channel.statistics.subscriber_count,
        video_count: raw_channel.statistics.video_count,
    };

    Ok(channel)
}
