use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use vt_common::youtube::YoutubeChannel;

use crate::constants::{APP_NAME, CONFIG_FILE};

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub version: String,
    pub channels: HashMap<String, YoutubeChannel>,
    pub groups: HashMap<String, HashSet<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "v2".to_string(), //
            channels: HashMap::new(),
            groups: HashMap::new(),
        }
    }
}

/// Set the global config
pub fn init() -> Result<()> {
    let config: Config = confy::load(APP_NAME, CONFIG_FILE) //
        .with_context(|| "unable to load config")?;

    CONFIG.set(config).expect("could not set config");

    Ok(())
}

/// Get the config file path
pub fn path() -> Result<PathBuf> {
    confy::get_configuration_file_path(APP_NAME, CONFIG_FILE)
        .with_context(|| "unable to find the config file")
}

/// Get the config file
pub fn get() -> &'static Config {
    CONFIG.get().expect("config is not initialized")
}

/// Save the config file
pub fn save(config: Config) -> Result<()> {
    confy::store(APP_NAME, CONFIG_FILE, &config) //
        .with_context(|| "unable to save config")
}
