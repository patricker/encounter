//! Catalog loader: reads `.toml` + `.fabula` file pairs from a directory tree.

use crate::Error;
use crate::affordance::{AffordanceSpec, CatalogEntry};
use std::fs;
use std::path::Path;

/// Load all catalog entries from a directory (recursively nested).
pub fn load_catalog_dir(dir: &Path) -> Result<Vec<CatalogEntry<String>>, Error> {
    let mut entries = Vec::new();
    load_dir_recursive(dir, &mut entries)?;
    Ok(entries)
}

fn load_dir_recursive(dir: &Path, entries: &mut Vec<CatalogEntry<String>>) -> Result<(), Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            load_dir_recursive(&path, entries)?;
        } else if path.extension().map(|e| e == "toml").unwrap_or(false) {
            let toml_content = fs::read_to_string(&path)?;
            let spec: AffordanceSpec =
                toml::from_str(&toml_content).map_err(|e| Error::CatalogParse {
                    path: path.display().to_string(),
                    reason: e.to_string(),
                })?;

            let fabula_path = path.with_extension("fabula");
            let precondition = if fabula_path.exists() {
                fs::read_to_string(&fabula_path)?
            } else {
                String::new()
            };

            entries.push(CatalogEntry { spec, precondition });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let entries = load_catalog_dir(dir.path()).unwrap();
        assert!(entries.is_empty());
    }
}
