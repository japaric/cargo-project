//! `Cargo.toml` parser

#[derive(Deserialize)]
pub struct Manifest {
    pub package: Package,
    #[serde(default)]
    pub bin: Vec<Binary>,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>
}

#[derive(Deserialize)]
/// A binary target in the project
pub struct Binary {
    /// The name of the binary target
    pub name: String,
    /// The source path of the target
    pub path: String,
}

#[cfg(test)]
mod tests {
    use toml;

    use super::Manifest;

    #[test]
    fn package_name() {
        let manifest: Manifest = toml::from_str(
            r#"
[package]
name = "foo"
version = "0.1"
"#,
        ).unwrap();

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
        ).unwrap();

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
        ).unwrap();

        assert_eq!(manifest.package.description.unwrap(), "Test description");
    }

    #[test]
    fn binaries() {
        let manifest: Manifest = toml::from_str(
            r#"
[package]
name = "foo"
version = "0.1"

[[bin]]
name = "foobar"
path = "src/foo.rs"

[[bin]]
name = "barfoo"
path = "src/bar.rs"
"#,
        ).unwrap();

        assert_eq!(manifest.bin.len(), 2);

        assert_eq!(manifest.bin[0].name, "foobar");
        assert_eq!(manifest.bin[0].path, "src/foo.rs");

        assert_eq!(manifest.bin[1].name, "barfoo");
        assert_eq!(manifest.bin[1].path, "src/bar.rs");
    }
}
