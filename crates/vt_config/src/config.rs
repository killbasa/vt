use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use vt_common::constants::APP_NAME;

const CONFIG_FILE: &str = "vt.config";
const SECRETS_FILE: &str = "vt.secret";

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
    pub channels: Option<HashMap<String, String>>,
    pub groups: Option<HashMap<String, HashSet<String>>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Secrets {
    pub apikey: Option<String>,
}

/// Get the config file path
pub fn path() -> Result<PathBuf> {
    confy::get_configuration_file_path(APP_NAME, CONFIG_FILE)
        .with_context(|| "unable to find the config file")
}

/// Load config from the config file
pub fn load_config() -> Result<Config> {
    confy::load(APP_NAME, CONFIG_FILE).with_context(|| "unable to load config")
}

/// Load secrets from the config file
pub fn load_secrets() -> Result<Secrets> {
    confy::load(APP_NAME, SECRETS_FILE).with_context(|| "unable to load secrets")
}

/// Save config to the config file
pub fn save_config(config: Config) -> Result<()> {
    confy::store(APP_NAME, CONFIG_FILE, config).with_context(|| "unable to save config")
}

/// Save secrets to the config file
pub fn save_secrets(config: Secrets) -> Result<()> {
    confy::store(APP_NAME, SECRETS_FILE, config).with_context(|| "unable to save secrets")
}
