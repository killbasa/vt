use std::collections::HashSet;

use anyhow::{Result, anyhow};
use clap::Args;
use vt_config::config;

/// List groups
#[derive(Args, Debug)]
pub struct Cli {
    /// The name of the group to list
    group: Option<String>,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let config = config::get();

        if config.groups.is_empty() {
            return Err(anyhow!("there are no groups to list"));
        }

        if let Some(group) = &self.group {
            let channels = config.groups.get(group).ok_or_else(|| anyhow!("group not found"))?;

            print_group(group, channels);
        } else {
            let mut groups: Vec<_> = config.groups.iter().collect();
            groups.sort_by_key(|(name, _)| *name);

            for (name, channels) in groups {
                print_group(name, channels);
            }
        }

        Ok(())
    }
}

fn print_group(name: &str, channels: &HashSet<String>) {
    println!("{}", name);

    let mut channels: Vec<_> = channels.iter().collect();
    channels.sort();

    for channel in channels {
        println!("  {}", channel);
    }
}
