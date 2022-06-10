//! `Cargo.toml` parser -- workspace root variant

#[derive(Deserialize)]
pub struct Manifest {
    pub workspace: Workspace,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub members: Vec<String>,
}

#[cfg(test)]
mod tests {
    use toml;

    use super::Manifest;

    #[test]
    fn members() {
        let manifest: Manifest = toml::from_str(
            r#"
[workspace]
members = ["foo", "bar"]
"#,
        )
        .unwrap();

        assert_eq!(manifest.workspace.members[0], "foo");
        assert_eq!(manifest.workspace.members[1], "bar");
    }
}
