use anyhow::{Result, anyhow};
use clap::Args;

use crate::{app, config};

/// Add a channel to a list
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    list: String,
    alias: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();
        let mut lists = config.lists.unwrap_or_default();

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

                println!("added {} to {}", &self.alias, &self.list);
            }
            false => println!("Channel already exists in list"),
        };

        Ok(())
    }
}
