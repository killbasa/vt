use anyhow::{anyhow, Result};
use clap::Args;
use std::collections::{HashMap, HashSet};

use crate::{app, config};

/// Create a list
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    list: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        if config.lists.is_none() {
            config.lists = Some(HashMap::new());
        }

        if let Some(mut lists) = config.lists {
            if lists.contains_key(&self.list) {
                return Err(anyhow!("list already exists"));
            }

            lists.insert(self.list.clone(), HashSet::new());
            config.lists = Some(lists);
        }

        config::save_config(config)?;

        println!("List created: {}", &self.list);

        Ok(())
    }
}
