use const_format::formatcp;

pub const CLI_USER_AGENT: &str = formatcp!("vt-client/{}", env!("CARGO_PKG_VERSION"));

pub const WEB_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0";

pub const APPLICATION_JSON: &str = "application/json";
