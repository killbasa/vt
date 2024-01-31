use anyhow::{anyhow, Result};
use clap::Args;

use crate::{app, config};

/// Set a channel
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    list: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        match config.lists {
            Some(mut lists) => {
                if lists.contains_key(&self.list) {
                    lists.remove(&self.list);
                    config.lists = Some(lists);
                } else {
                    return Err(anyhow!("list not found"));
                }
            }
            None => {
                return Err(anyhow!("there are no lists to delete"));
            }
        }

        config::save_config(config)?;

        println!("List deleted: {}", &self.list);

        Ok(())
    }
}
