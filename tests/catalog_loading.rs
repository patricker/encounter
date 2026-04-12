use encounter::catalog::load_catalog_dir;

#[test]
fn loads_fixture_catalog() {
    let fixtures = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let entries = load_catalog_dir(&fixtures).unwrap();
    assert!(!entries.is_empty());
    let reveal = entries
        .iter()
        .find(|e| e.spec.name == "reveal_secret")
        .unwrap();
    assert_eq!(reveal.spec.domain, "information");
    assert!(!reveal.precondition.is_empty());
    assert!(!reveal.spec.effects_on_accept.is_empty());
}

#[test]
fn handles_missing_fabula_file_gracefully() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("orphan.toml"),
        r#"name = "orphan"
domain = "test"
bindings = ["self"]
"#,
    )
    .unwrap();
    let entries = load_catalog_dir(dir.path()).unwrap();
    assert_eq!(entries.len(), 1);
    assert!(entries[0].precondition.is_empty());
}
