use const_format::formatcp;

pub const CLI_USER_AGENT: &str = formatcp!("vt-client/{}", env!("CARGO_PKG_VERSION"));

pub const WEB_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:137.0) Gecko/20100101 Firefox/137.0";

pub const APPLICATION_JSON: &str = "application/json";
