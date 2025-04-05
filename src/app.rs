use std::collections::HashSet;
use vt_common::youtube::YoutubeChannel;
use vt_config::config;

/// Get a channel by name
pub fn get_channel(channel_name: &str) -> Option<YoutubeChannel> {
    config::get().clone().channels.get(channel_name).cloned()
}

/// Get a group by name
pub fn get_group(group: &str) -> Option<HashSet<String>> {
    config::get().clone().groups.get(group).cloned()
}
