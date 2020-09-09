extern crate cargo_project;

use cargo_project::Project;

#[test]
fn test_inferred_binary() {
    let project = Project::query("./test-projects/default-binary").unwrap();

    let binaries = project.binaries().collect::<Vec<_>>();
    assert_eq!(binaries.len(), 1);
    assert_eq!(binaries[0].name, "foo");
    assert_eq!(binaries[0].path, "src/main.rs");
}

#[test]
fn test_no_inferred_binary() {
    let project = Project::query("./test-projects/no-default-binary").unwrap();

    let binaries = project.binaries().collect::<Vec<_>>();
    assert_eq!(binaries.len(), 0);
}