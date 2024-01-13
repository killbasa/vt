use once_cell::sync::OnceCell;

use crate::config::Config;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn config() -> &'static Config {
    CONFIG.get().expect("config is not initialized")
}

pub fn set_global_config(config: Config) {
    CONFIG.set(config).expect("could not set config")
}

pub fn get_alias(channel: &str) -> Option<String> {
    if let Some(aliases) = config().clone().aliases {
        return aliases.get(channel).cloned();
    }

    None
}
