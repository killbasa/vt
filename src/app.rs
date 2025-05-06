use std::collections::HashSet;

use vt_config::{channel::VTChannel, config};

/// Get a channel by name
pub fn get_channel(channel_name: &str) -> Option<&VTChannel> {
    config::get().channels.get(channel_name)
}

/// Get a group by name
pub fn get_group(group: &str) -> Option<&HashSet<String>> {
    config::get().groups.get(group)
}
