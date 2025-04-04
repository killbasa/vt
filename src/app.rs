use once_cell::sync::OnceCell;
use std::collections::HashSet;
use vt_config::config::{Config, Secrets};

static CONFIG: OnceCell<Config> = OnceCell::new();
static SECRETS: OnceCell<Secrets> = OnceCell::new();

/// Initialize the global config
pub fn config() -> &'static Config {
    CONFIG.get().expect("config is not initialized")
}

/// Set the global config
pub fn set_global_config(config: Config) {
    CONFIG.set(config).expect("could not set config")
}

/// Initialize the global secrets
pub fn secrets() -> &'static Secrets {
    SECRETS.get().expect("secrets is not initialized")
}

/// Set the global secrets
pub fn set_global_secrets(secrets: Secrets) {
    SECRETS.set(secrets).expect("could not set secrets")
}

/// Get a channel by name
pub fn get_channel(channel: &str) -> Option<String> {
    if let Some(channels) = config().clone().channels {
        return channels.get(channel).cloned();
    }

    None
}

/// Get a group by name
pub fn get_group(group: &str) -> Option<HashSet<String>> {
    if let Some(groups) = config().clone().groups {
        return groups.get(group).cloned();
    }

    None
}
