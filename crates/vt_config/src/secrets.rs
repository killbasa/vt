use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::constants::{APP_NAME, SECRETS_FILE};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Secrets {
    pub apikey: Option<String>,
}

static SECRETS: OnceCell<Secrets> = OnceCell::new();

/// Set the global secrets
pub fn init() -> Result<()> {
    let config: Secrets = confy::load(APP_NAME, SECRETS_FILE) //
        .with_context(|| "unable to load secrets")?;

    SECRETS.set(config).expect("could not set secrets");

    Ok(())
}

/// Initialize the global secrets
pub fn get() -> &'static Secrets {
    SECRETS.get().expect("secrets is not initialized")
}

/// Save the secrets file
pub fn save(config: Secrets) -> Result<()> {
    confy::store(APP_NAME, SECRETS_FILE, config).with_context(|| "unable to save secrets")
}
