use anyhow::{Result, anyhow};

use crate::secrets;

/// Get the apikey from the config
pub fn get_apikey() -> Result<String> {
    let secrets = secrets::get();

    match &secrets.apikey {
        Some(apikey) => Ok(apikey.to_string()),
        None => Err(anyhow!("API key not found. Use `vt config apikey` to set an API key.")),
    }
}
