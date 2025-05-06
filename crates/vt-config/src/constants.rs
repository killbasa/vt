use const_format::formatcp;

pub const APP_NAME: &str = "vt";

pub const CONFIG_FILE: &str = formatcp!("{}.config", APP_NAME);
pub const SECRETS_FILE: &str = formatcp!("{}.secret", APP_NAME);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_name() {
        assert_eq!(APP_NAME, "vt");
    }

    #[test]
    fn test_config_file() {
        assert_eq!(CONFIG_FILE, "vt.config");
    }

    #[test]
    fn test_secrets_file() {
        assert_eq!(SECRETS_FILE, "vt.secret");
    }
}
