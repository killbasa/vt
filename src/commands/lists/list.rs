use std::collections::HashSet;

use anyhow::Result;
use clap::Args;

use crate::app;

// There has to be a better name for this

/// List lists
#[derive(Args, Debug)]
#[command(alias = "ls")]
pub struct Cli {
    list: Option<String>,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        if config.lists.is_none() || config.lists.as_ref().unwrap().is_empty() {
            println!("No lists set");
            return Ok(());
        }

        let lists = config.lists.unwrap();

        if self.list.is_none() {
            for (k, v) in lists.iter() {
                print_list(k, v)
            }

            return Ok(());
        }

        let list_name = self.list.as_ref().unwrap();
        match lists.get(list_name) {
            Some(list) => print_list(list_name, list),
            None => println!("List not found"),
        }

        Ok(())
    }
}

fn print_list(name: &String, list: &HashSet<String>) {
    println!("{}", name);

    for k in list {
        println!("  {}", k);
    }
}
