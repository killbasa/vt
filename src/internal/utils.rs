use chrono::DateTime;
use chrono_humanize::HumanTime;
use colored::{ColoredString, Colorize};

use super::youtube::Video;

pub fn format_videos(videos: Vec<Video>, include_channel: bool) -> String {
    let mut video_list = Vec::<String>::new();

    for video in videos {
        let status: ColoredString = match video.start_time.is_some() {
            true => "[live]".bright_red(),
            false => "[upcoming]".bright_yellow(),
        };

        let title = video.title.bright_cyan();
        let channel = video.channel.bright_green();
        let url = format!("https://www.youtube.com/watch?v={}", video.id).bright_green();

        let mut entry = match include_channel {
            true => {
                format!(
                    "{} {}\n ├─   channel: {}\n ├─       url: {}\n",
                    status, title, channel, url
                )
            }
            false => {
                format!("{} {}\n ├─       url: {}\n", status, title, url)
            }
        };

        if let Some(start_time) = video.start_time {
            let started = humanize_time(&start_time);
            entry.push_str(&format!(" └─   started: {}\n", started.bright_green()));
        } else {
            let scheduled = humanize_time(&video.scheduled_time);
            entry.push_str(&format!(" └─ scheduled: {}\n", scheduled.bright_green()));
        }

        video_list.push(entry);
    }

    video_list.join("\n")
}

pub fn humanize_time(time: &str) -> String {
    let parsed = DateTime::parse_from_rfc3339(time).unwrap();
    HumanTime::from(parsed).to_string()
}
