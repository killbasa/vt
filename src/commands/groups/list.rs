use anyhow::Result;
use clap::Args;
use std::collections::HashSet;

use crate::app;

// There has to be a better name for this

/// List groups
#[derive(Args, Debug)]
pub struct Cli {
    /// The name of the group to list
    group: Option<String>,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = app::config().clone();

        if config.groups.is_none() || config.groups.as_ref().unwrap().is_empty() {
            println!("No groups set");
            return Ok(());
        }

        let groups = config.groups.unwrap();

        if self.group.is_none() {
            for (k, v) in groups.iter() {
                print_list(k, v)
            }

            return Ok(());
        }

        let list_name = self.group.as_ref().unwrap();
        match groups.get(list_name) {
            Some(group) => print_list(list_name, group),
            None => println!("Group not found"),
        }

        Ok(())
    }
}

fn print_list(name: &String, group: &HashSet<String>) {
    println!("{}", name);

    for k in group {
        println!("  {}", k);
    }
}
