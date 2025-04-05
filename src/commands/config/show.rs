use anyhow::Result;
use clap::Args;
use std::{
    fs,
    io::{self, BufRead},
};
use vt_config::config;

/// Show the config
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let path = config::path()?;
        let file = fs::File::open(&path)?;

        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line?);
        }

        Ok(())
    }
}
