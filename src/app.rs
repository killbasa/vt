use std::collections::HashSet;

use once_cell::sync::OnceCell;

use crate::config::{Config, Secrets};

static CONFIG: OnceCell<Config> = OnceCell::new();
static SECRETS: OnceCell<Secrets> = OnceCell::new();

pub fn config() -> &'static Config {
    CONFIG.get().expect("config is not initialized")
}

pub fn set_global_config(config: Config) {
    CONFIG.set(config).expect("could not set config")
}

pub fn secrets() -> &'static Secrets {
    SECRETS.get().expect("secrets is not initialized")
}

pub fn set_global_secrets(secrets: Secrets) {
    SECRETS.set(secrets).expect("could not set secrets")
}

pub fn get_channel(channel: &str) -> Option<String> {
    if let Some(channels) = config().clone().channels {
        return channels.get(channel).cloned();
    }

    None
}

pub fn get_list(list: &str) -> Option<HashSet<String>> {
    if let Some(lists) = config().clone().lists {
        return lists.get(list).cloned();
    }

    None
}
