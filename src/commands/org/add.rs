use anyhow::Result;
use clap::Args;

use crate::{app, config};

/// Add an organization alias
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    alias: String,
    org_id: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = app::config().clone();

        if config.orgs.is_none() {
            config.orgs = Some(std::collections::HashMap::new());
        }

        if let Some(mut orgs) = config.orgs.clone() {
            if orgs.contains_key(&self.alias) {
                println!("Channe alias already exists");
                return Ok(());
            }

            orgs.insert(self.alias.clone(), self.org_id.clone());
            config.orgs = Some(orgs);
        }

        config::save(config)?;

        println!(
            "Org alias added: {} -> {}",
            self.alias.clone(),
            self.org_id.clone()
        );

        Ok(())
    }
}
