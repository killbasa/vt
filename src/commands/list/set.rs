use anyhow::{anyhow, Result};
use clap::Args;
use std::collections::HashMap;

use crate::{app, config};

/// Set a channel
#[derive(Args, Debug)]
#[command()]
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

        let alias = match app::get_channel(&self.alias) {
            Some(_) => &self.alias,
            None => {
                return Err(anyhow!("channel not found"));
            }
        };

        match list.insert(alias.clone()) {
            true => {
                lists.insert(self.list.clone(), list);
                config.lists = Some(lists);
                config::save_config(config)?;

                println!("Channel added to list");
            }
            false => println!("Channel already exists in list"),
        };

        Ok(())
    }
}
