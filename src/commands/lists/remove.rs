use anyhow::{anyhow, Result};
use clap::Args;
use std::collections::HashMap;

use crate::{app, config};

/// Remove a channel from a list
#[derive(Args, Debug)]
#[command(alias = "rm")]
pub struct Cli {
    list: String,
    alias: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();
        let mut lists = match config.lists {
            Some(lists) => lists,
            None => HashMap::new(),
        };

        let mut list = match lists.get(&self.list) {
            Some(list) => list.clone(),
            None => {
                return Err(anyhow!("list not found"));
            }
        };

        match list.remove(&self.alias) {
            true => {
                lists.insert(self.list.clone(), list);
                config.lists = Some(lists);
                config::save_config(config)?;

                println!("removed {} from {}", &self.alias, &self.list);
            }
            false => println!("{} was not in that list", &self.alias),
        };

        Ok(())
    }
}
