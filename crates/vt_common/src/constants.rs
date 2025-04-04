use const_format::formatcp;

pub const APP_NAME: &str = "vt";

pub const CLI_USER_AGENT: &str = formatcp!("vt-client/{}", env!("CARGO_PKG_VERSION"));
