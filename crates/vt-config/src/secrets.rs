use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::constants::{APP_NAME, SECRETS_FILE};

static SECRETS: OnceCell<Secrets> = OnceCell::new();

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Secrets {
    pub apikey: Option<String>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secrets_default() {
        let config = Secrets::default();
        assert_eq!(config.apikey, None);
    }
}
