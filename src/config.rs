use anyhow::{Context, Result};
use confy;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

const APP_NAME: &str = "vt";
const CONFIG_FILE: &str = "vt.config";
const SECRETS_FILE: &str = "vt.secret";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub channels: Option<HashMap<String, String>>,
    pub lists: Option<HashMap<String, HashSet<String>>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Secrets {
    pub apikey: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            channels: None,
            lists: None,
        }
    }
}

impl Default for Secrets {
    fn default() -> Self {
        Self { apikey: None }
    }
}

pub fn path() -> Result<PathBuf> {
    confy::get_configuration_file_path(APP_NAME, CONFIG_FILE)
        .with_context(|| "unable to find the config file")
}

pub fn load_config() -> Result<Config> {
    confy::load(APP_NAME, CONFIG_FILE).with_context(|| "unable to load config")
}

pub fn load_secrets() -> Result<Secrets> {
    confy::load(APP_NAME, SECRETS_FILE).with_context(|| "unable to load secrets")
}

pub fn save_config(config: Config) -> Result<()> {
    confy::store(APP_NAME, CONFIG_FILE, config).with_context(|| "unable to save config")
}

pub fn save_secrets(config: Secrets) -> Result<()> {
    confy::store(APP_NAME, SECRETS_FILE, config).with_context(|| "unable to save secrets")
}
