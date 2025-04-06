use anyhow::Result;
use clap::Args;
use vt_config::secrets;

/// Set a YouTube API key
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut secrets = secrets::get().clone();

        let apikey = rpassword::prompt_password("Your YouTube API key: ")?;

        if apikey.is_empty() {
            println!("YouTube API key cannot be empty");
            return Ok(());
        }

        secrets.apikey = Some(apikey);
        secrets::save(secrets)?;

        println!("YouTube API key saved");

        Ok(())
    }
}
