use anyhow::Result;
use clap::Args;
use self_update::{backends, cargo_crate_version};

/// Update the vt install
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let status = backends::github::Update::configure()
            .repo_owner("killbasa")
            .repo_name("vt")
            .bin_name("vt")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;

        if status.uptodate() {
            println!("up to date");
        } else {
            println!("updated to version {}", status.version());
        }

        Ok(())
    }
}
