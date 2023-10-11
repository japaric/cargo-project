//! `Cargo.toml` parser

#[derive(Deserialize)]
pub struct Manifest {
    pub package: Package,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Workspace {
    workspace: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Version {
    Str(String),
    Map(Workspace),
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use toml;

    use crate::manifest::Workspace;
    use crate::manifest::Version;

    use super::Manifest;

    #[test]
    fn package_name() {
        let manifest: Manifest = toml::from_str(
            r#"
[package]
name = "foo"
version = "0.1"
"#,
        )
        .unwrap();

        assert_eq!(manifest.package.name, "foo");
    }

    #[test]
    fn package_description_missing() {
        let manifest: Manifest = toml::from_str(
            r#"
[package]
name = "foo"
version = "0.1"
"#,
        )
        .unwrap();

        assert_eq!(manifest.package.description, None);
    }

    #[test]
    fn package_description_present() {
        let manifest: Manifest = toml::from_str(
            r#"
[package]
name = "foo"
version = "0.1"
description = "Test description"
"#,
        )
        .unwrap();

        assert_eq!(manifest.package.description.unwrap(), "Test description");
    }

    #[test]
    fn version_is_workspace() {
        let manifest: Manifest = toml::from_str(
            r#"
[package]
name = "foo"
version = { workspace = true }
description = "Test description"
"#,
        )
        .unwrap();

        assert_eq!(manifest.package.version, Version::Map(Workspace{workspace: true}));
    }
}
