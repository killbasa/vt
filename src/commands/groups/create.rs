use anyhow::{Result, anyhow};
use clap::Args;
use std::collections::{HashMap, HashSet};
use vt_config::config;

use crate::app;

/// Create a group
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The name of the group
    group: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        if config.groups.is_none() {
            config.groups = Some(HashMap::new());
        }

        if let Some(mut groups) = config.groups {
            if groups.contains_key(&self.group) {
                return Err(anyhow!("group already exists"));
            }

            groups.insert(self.group.clone(), HashSet::new());
            config.groups = Some(groups);
        }

        config::save_config(config)?;

        println!("Group created: {}", &self.group);

        Ok(())
    }
}
