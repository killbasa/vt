use const_format::formatcp;

pub const APP_NAME: &str = "vt";

pub const CONFIG_FILE: &str = formatcp!("{}.config", APP_NAME);
pub const SECRETS_FILE: &str = formatcp!("{}.secret", APP_NAME);
