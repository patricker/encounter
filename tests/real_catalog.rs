use encounter::catalog::load_catalog_dir;
use std::path::Path;

const SOCIETAS_CATALOG: &str = "../societas/catalog";

fn catalog_available() -> bool {
    Path::new(SOCIETAS_CATALOG).is_dir()
}

#[test]
fn loads_all_30_actions_from_societas_catalog() {
    if !catalog_available() {
        eprintln!("SKIP: societas catalog not found at {}", SOCIETAS_CATALOG);
        return;
    }
    let entries = load_catalog_dir(Path::new(SOCIETAS_CATALOG)).unwrap();
    assert!(
        entries.len() >= 30,
        "expected at least 30 catalog entries, got {}",
        entries.len()
    );

    // Spot-check known actions.
    let names: Vec<&str> = entries.iter().map(|e| e.spec.name.as_str()).collect();
    assert!(names.contains(&"reveal_secret"), "missing reveal_secret");
    assert!(names.contains(&"betray"), "missing betray");
    assert!(names.contains(&"propose_alliance"), "missing propose_alliance");
    assert!(names.contains(&"trade"), "missing trade");
    assert!(names.contains(&"charm"), "missing charm");
}

#[test]
fn every_entry_has_at_least_one_binding() {
    if !catalog_available() { return; }
    let entries = load_catalog_dir(Path::new(SOCIETAS_CATALOG)).unwrap();
    for entry in &entries {
        assert!(
            !entry.spec.bindings.is_empty(),
            "{} has no bindings",
            entry.spec.name
        );
        assert!(
            entry.spec.bindings.contains(&"self".to_string()),
            "{} is missing 'self' binding",
            entry.spec.name
        );
    }
}

#[test]
fn every_entry_has_precondition_source() {
    if !catalog_available() { return; }
    let entries = load_catalog_dir(Path::new(SOCIETAS_CATALOG)).unwrap();
    for entry in &entries {
        assert!(
            !entry.precondition.is_empty(),
            "{} has no .fabula file",
            entry.spec.name
        );
    }
}
