//! `.cargo/config` parser

#[derive(Deserialize)]
pub struct Config {
    pub build: Option<Build>,
}

#[derive(Deserialize)]
pub struct Build {
    pub target: Option<String>,
    #[serde(rename = "target-dir")]
    pub target_dir: Option<String>,
}

#[cfg(test)]
mod tests {
    use toml;

    use super::Config;

    #[test]
    fn empty_config() {
        let config: Config = toml::from_str("").unwrap();

        assert!(config.build.is_none());
    }

    #[test]
    fn config_empty_build() {
        let config: Config = toml::from_str("[build]").unwrap();

        assert!(config.build.unwrap().target.is_none());
    }

    #[test]
    fn config_build_target() {
        let config: Config = toml::from_str(
            r#"
[build]
target = "thumbv7m-none-eabi"
"#,
        ).unwrap();

        assert_eq!(config.build.unwrap().target.unwrap(), "thumbv7m-none-eabi")
    }

    #[test]
    fn config_no_build_target() {
        let config: Config = toml::from_str(
            r#"
[target.thumbv7m-none-eabi]
runner = "arm-none-eabi-gdb"
"#,
        ).unwrap();

        assert!(config.build.is_none());
    }

    #[test]
    fn config_build_target_plus() {
        let config: Config = toml::from_str(
            r#"
[target.thumbv7m-none-eabi]
runner = "arm-none-eabi-gdb"

[build]
target = "thumbv7m-none-eabi"
"#,
        ).unwrap();

        assert_eq!(config.build.unwrap().target.unwrap(), "thumbv7m-none-eabi");
    }

    #[test]
    fn config_build_target_dir() {
        let config: Config = toml::from_str(
            r#"
[build]
target-dir = "custom-target"
"#,
        ).unwrap();

        assert_eq!(config.build.unwrap().target_dir.unwrap(), "custom-target")
    }
}
