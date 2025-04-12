use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;
use colored::{ColoredString, Colorize};
use vt_common::youtube::{YoutubeChannel, YoutubeVideo};

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M";

pub fn format_video(video: &YoutubeVideo, include_channel: bool) -> String {
    let status: ColoredString = match video.start_time.is_some() {
        true => "[live]".bright_red(),
        false => "[upcoming]".bright_yellow(),
    };

    let title = video.title.bright_cyan();
    let channel = video.channel.bright_green();
    let url = format!("https://www.youtube.com/watch?v={}", video.id).bright_green();

    let mut entry = match include_channel {
        true => {
            format!("{} {}\n ├─   channel: {}\n ├─       url: {}\n", status, title, channel, url)
        }
        false => {
            format!("{} {}\n ├─       url: {}\n", status, title, url)
        }
    };

    if let Some(start_time) = &video.start_time {
        let (date, diff) = humanize_time(start_time);

        entry.push_str(&format!(
            " └─   started: {}\n",
            format!("{} ({})", date, diff).bright_green()
        ));
    } else {
        let (date, diff) = humanize_time(&video.scheduled_time);

        entry.push_str(&format!(
            " └─ scheduled: {}\n",
            format!("{} ({})", date, diff).bright_green()
        ));
    }

    entry
}

pub fn format_videos(videos: Vec<YoutubeVideo>, include_channel: bool) -> String {
    let mut video_list = Vec::<String>::new();

    for video in videos {
        video_list.push(format_video(&video, include_channel));
    }

    video_list.join("\n")
}

pub fn format_channel(channel: &YoutubeChannel) -> String {
    let url = match &channel.custom_url {
        Some(custom_url) => format!("https://www.youtube.com/{}", custom_url),
        None => format!("https://www.youtube.com/channel/{}", channel.id),
    };

    format!(
        "\n{0: <9}{1}\n{2: <9}{3}\n{4: <9}{5}\n{6: <9}{7}\n{8: <9}{9}",
        "channel:",
        channel.name.bright_cyan(),
        "url:",
        url.bright_green(),
        "id:",
        channel.id.bright_green(),
        "subs:",
        channel.subscriber_count.bright_green(),
        "videos:",
        channel.video_count.bright_green(),
    )
}

pub fn humanize_time(time: &str) -> (String, String) {
    let tz = &Local::now().timezone();
    let parsed = DateTime::parse_from_rfc3339(time).unwrap();
    let humanized = HumanTime::from(parsed);

    (
        parsed.with_timezone(tz).format(TIME_FORMAT).to_string(), //
        humanized.to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use vt_common::youtube::YoutubeVideo;

    #[test]
    fn test_format_video() {
        let video = YoutubeVideo {
            id: "12345".to_string(),
            title: "Test Video".to_string(),
            channel: "Test Channel".to_string(),
            start_time: Some("2023-10-01T12:00:00Z".to_string()),
            scheduled_time: "2023-10-01T12:00:00Z".to_string(),
        };
        let result = format_video(&video, true);

        assert!(result.contains("[live]"));
        assert!(result.contains("Test Video"));
        assert!(result.contains("Test Channel"));
        assert!(result.contains("https://www.youtube.com/watch?v=12345"));
    }

    #[test]
    fn test_format_videos() {
        let video1 = YoutubeVideo {
            id: "12345".to_string(),
            title: "Test Video 1".to_string(),
            channel: "Test Channel 1".to_string(),
            start_time: Some("2023-10-01T12:00:00Z".to_string()),
            scheduled_time: "2023-10-01T12:00:00Z".to_string(),
        };
        let video2 = YoutubeVideo {
            id: "67890".to_string(),
            title: "Test Video 2".to_string(),
            channel: "Test Channel 2".to_string(),
            start_time: None,
            scheduled_time: "2023-10-02T12:00:00Z".to_string(),
        };

        let videos = vec![video1, video2];
        let result = format_videos(videos, true);

        assert!(result.contains("[live]"));
        assert!(result.contains("Test Video 1"));
        assert!(result.contains("Test Channel 1"));
        assert!(result.contains("https://www.youtube.com/watch?v=12345"));

        assert!(result.contains("[upcoming]"));
        assert!(result.contains("Test Video 2"));
        assert!(result.contains("Test Channel 2"));
        assert!(result.contains("https://www.youtube.com/watch?v=67890"));
    }
}
