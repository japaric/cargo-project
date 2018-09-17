//! `Cargo.toml` parser

#[derive(Deserialize)]
pub struct Manifest {
    pub package: Package,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
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
"#,
        ).unwrap();

        assert_eq!(manifest.package.name, "foo");
    }
}
