//! Disk-backed cache for downloaded cover thumbnail image bytes.
//!
//! Without this, every launch re-downloads every catalog item's cover thumbnail from
//! the network, since [`crate::ui::library::cover::CoverCache`] (the in-memory decoded
//! image cache) always starts empty. This module persists the raw fetched bytes to
//! disk so a warm cache is available on the very first thumbnail request of a new
//! session, before any network round trip.
//!
//! Each cached image is stored as a single file under [`crate::data::paths::covers_dir`],
//! named by a stable hash of the owning `LibraryItem` id — the id is a URL path (e.g.
//! `/api/vBeta/order_products/12345`) and unsafe to use directly as a filename. The
//! image format is not encoded in the filename; callers sniff it from the leading bytes
//! the same way the in-memory cache does (see `ui::library::cover::sniff_image_format`).
//!
//! Unlike [`crate::data::catalog_cache`], writes are not atomic (no `.tmp`-then-rename):
//! a torn write here only means that one cover re-downloads on the next launch, not a
//! corrupted multi-item dataset, so the extra write complexity isn't warranted.

use std::fs;
use std::path::Path;

use crate::util::hash::fnv1a_32;

/// Returns the on-disk filename for `item_id`'s cached cover.
fn cover_filename(item_id: &str) -> String {
    format!("{:08x}.cover", fnv1a_32(item_id))
}

/// Reads the cached cover image bytes for `item_id` from `{root}/{hash}.cover`.
///
/// Returns `None` on any error (missing file, unreadable, empty) so callers fall
/// through to a live fetch without surfacing errors to the user.
pub fn load_cached_cover(root: &Path, item_id: &str) -> Option<Vec<u8>> {
    let path = root.join(cover_filename(item_id));
    let bytes = fs::read(&path).ok()?;
    if bytes.is_empty() { None } else { Some(bytes) }
}

/// Writes `bytes` to `{root}/{hash}.cover`, creating `root` if needed.
///
/// Failures are not surfaced as an error type — a failed cache write only costs a
/// repeat network fetch next launch, not correctness, so callers log a warning and
/// continue rather than threading a `Result` through the thumbnail-fetch path.
pub fn save_cached_cover(root: &Path, item_id: &str, bytes: &[u8]) {
    if let Err(e) = fs::create_dir_all(root) {
        tracing::warn!(path = %root.display(), error = %e, "cover cache: failed to create dir");
        return;
    }
    let path = root.join(cover_filename(item_id));
    if let Err(e) = fs::write(&path, bytes) {
        tracing::warn!(path = %path.display(), error = %e, "cover cache: failed to write");
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn test_dir(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("dtrpg_cover_cache_test_{name}"))
    }

    #[test]
    fn load_missing_file_returns_none() {
        let dir = std::path::PathBuf::from("/nonexistent/dtrpg_cover_cache_surely_missing");
        assert!(load_cached_cover(&dir, "/api/vBeta/order_products/1").is_none());
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = test_dir("roundtrip");
        let id = "/api/vBeta/order_products/515276";
        let bytes = vec![0xFFu8, 0xD8, 0xFF, 1, 2, 3];

        save_cached_cover(&dir, id, &bytes);
        let loaded = load_cached_cover(&dir, id);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(loaded, Some(bytes));
    }

    #[test]
    fn different_ids_do_not_collide() {
        let dir = test_dir("distinct_ids");
        save_cached_cover(&dir, "/api/vBeta/order_products/1", &[1, 1, 1]);
        save_cached_cover(&dir, "/api/vBeta/order_products/2", &[2, 2, 2]);

        let a = load_cached_cover(&dir, "/api/vBeta/order_products/1");
        let b = load_cached_cover(&dir, "/api/vBeta/order_products/2");
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(a, Some(vec![1, 1, 1]));
        assert_eq!(b, Some(vec![2, 2, 2]));
    }

    #[test]
    fn empty_file_is_treated_as_missing() {
        let dir = test_dir("empty");
        fs::create_dir_all(&dir).unwrap();
        let id = "/api/vBeta/order_products/9";
        fs::write(dir.join(cover_filename(id)), []).unwrap();

        let loaded = load_cached_cover(&dir, id);
        let _ = fs::remove_dir_all(&dir);

        assert!(loaded.is_none());
    }
}
