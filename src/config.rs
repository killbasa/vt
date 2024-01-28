use anyhow::{Context, Result};
use confy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

const APP_NAME: &str = "vt";
const FILE_STEM: &str = "vt.config";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub apikey: Option<String>,
    pub channels: Option<HashMap<String, String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            apikey: None,
            channels: None,
        }
    }
}

pub fn path() -> Result<PathBuf> {
    confy::get_configuration_file_path(APP_NAME, FILE_STEM)
        .with_context(|| "unable to find the config file")
}

pub fn load() -> Result<Config> {
    confy::load(APP_NAME, FILE_STEM).with_context(|| "unable to load config")
}

pub fn save(config: Config) -> Result<()> {
    confy::store(APP_NAME, FILE_STEM, config).with_context(|| "unable to save config")
}
