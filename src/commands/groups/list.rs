use anyhow::{Result, anyhow};
use clap::Args;
use std::collections::HashSet;
use vt_config::config;

/// List groups
#[derive(Args, Debug)]
pub struct Cli {
    /// The name of the group to list
    group: Option<String>,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = config::get().clone();

        if config.groups.is_empty() {
            return Err(anyhow!("there are no groups to list"));
        }

        if self.group.is_none() {
            for (k, v) in config.groups.iter() {
                print_list(k, v)
            }

            return Ok(());
        }

        let list_name = self.group.as_ref().unwrap();
        match config.groups.get(list_name) {
            Some(group) => print_list(list_name, group),
            None => {
                return Err(anyhow!("group not found"));
            }
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
