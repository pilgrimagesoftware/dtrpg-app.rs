//! Catalog cache: read and write the catalog to/from a JSON file on disk.

use std::fs;
use std::path::Path;

use thiserror::Error;
use tracing::warn;

use crate::data::library::LibraryItem;

const CACHE_FILE: &str = "catalog_cache.json";
const CACHE_TMP: &str = "catalog_cache.json.tmp";

// ── CatalogCacheError ─────────────────────────────────────────────────────────

/// Errors that can occur when writing the catalog cache.
#[derive(Debug, Error)]
pub enum CatalogCacheError {
    /// An I/O error occurred while reading or writing the cache file.
    #[error("catalog cache I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// The catalog data could not be serialized to JSON.
    #[error("catalog cache JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

// ── load_catalog_cache ────────────────────────────────────────────────────────

/// Reads the catalog cache from `{root}/catalog_cache.json`.
///
/// Returns `None` on any error (missing file, malformed JSON) so callers
/// can fall through to the live API fetch without surfacing errors to the user.
pub fn load_catalog_cache(root: &Path) -> Option<Vec<LibraryItem>> {
    let path = root.join(CACHE_FILE);
    let text = fs::read_to_string(&path)
        .map_err(|e| warn!(path = %path.display(), error = %e, "catalog cache not readable"))
        .ok()?;
    serde_json::from_str(&text)
        .map_err(|e| warn!(path = %path.display(), error = %e, "catalog cache malformed"))
        .ok()
}

// ── save_catalog_cache ────────────────────────────────────────────────────────

/// Writes `items` to `{root}/catalog_cache.json` atomically via a `.tmp` rename.
///
/// # Errors
///
/// Returns [`CatalogCacheError::Io`] if the directory cannot be created or the
/// file cannot be written or renamed. Returns [`CatalogCacheError::Json`] if
/// serialization fails.
pub fn save_catalog_cache(root: &Path, items: &[LibraryItem]) -> Result<(), CatalogCacheError> {
    fs::create_dir_all(root)?;
    let tmp = root.join(CACHE_TMP);
    let json = serde_json::to_string(items)?;
    fs::write(&tmp, &json)?;
    fs::rename(&tmp, root.join(CACHE_FILE))?;
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::data::enums::ItemStatus;
    use crate::data::library::LibraryItem;

    fn test_dir(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("dtrpg_cache_test_{name}"))
    }

    fn make_item(id: &str) -> LibraryItem {
        LibraryItem::new(
            id, "Test Title", "Test Publisher", "", "Core", "PDF",
            100, 10.0, 2024, 1, ItemStatus::Cloud, "#1C2A44", "Desc.", None,
        )
    }

    #[test]
    fn load_missing_file_returns_none() {
        let dir = std::path::PathBuf::from("/nonexistent/dtrpg_cache_surely_missing");
        assert!(load_catalog_cache(&dir).is_none());
    }

    #[test]
    fn load_malformed_json_returns_none() {
        let dir = test_dir("malformed");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(CACHE_FILE), b"not valid json { ").unwrap();
        let result = load_catalog_cache(&dir);
        let _ = fs::remove_dir_all(&dir);
        assert!(result.is_none());
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = test_dir("roundtrip");
        fs::create_dir_all(&dir).unwrap();
        let items = vec![make_item("a1"), make_item("a2")];
        save_catalog_cache(&dir, &items).unwrap();

        let loaded = load_catalog_cache(&dir);
        let _ = fs::remove_dir_all(&dir);

        let loaded = loaded.unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id.as_ref(), "a1");
        assert_eq!(loaded[1].id.as_ref(), "a2");
    }
}
