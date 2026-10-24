//! What the app's regenerable caches hold right now: how many titles the saved
//! catalog lists, how many cover thumbnails are kept, whether an avatar is,
//! and when the catalog and the last availability check were saved.

#[cfg(test)]
use std::path::Path;

use crate::data::avatar::avatar_cached;
use crate::data::library_metadata::{LibraryMetadata, load_library_metadata};
use crate::data::paths::{cache_dir, covers_dir};

/// Per-type counts of regenerable app cache data.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CacheCounts {
    /// Number of items in the cached catalog/collections metadata.
    pub metadata_items:             usize,
    /// Number of cached cover thumbnail files.
    pub cover_thumbnails:           usize,
    /// `true` if an avatar image is currently cached.
    pub avatar_cached:              bool,
    /// Unix timestamp (seconds) when the catalog/collections metadata cache
    /// was last written. `None` when no cache metadata exists yet.
    pub metadata_saved_at_secs:     Option<i64>,
    /// Unix timestamp (seconds) of the last per-item availability check
    /// batch (manual or automatic). `None` when no batch has run yet, or
    /// predates the field being tracked.
    pub last_item_check_batch_secs: Option<i64>,
}

impl CacheCounts {
    /// Reads the counts from disk: a cache-metadata sidecar read (no full
    /// catalog parse), one `read_dir` of the covers directory, and one avatar
    /// `exists()` check. It touches disk, so call it when the page is opened or
    /// the cache changes, never while drawing.
    #[must_use]
    pub fn read() -> Self {
        let cover_thumbnails = std::fs::read_dir(covers_dir()).map(Iterator::count)
                                                              .unwrap_or(0);
        Self::from_parts(load_library_metadata(&cache_dir()).as_ref(),
                         cover_thumbnails,
                         avatar_cached())
    }

    /// The counts for the given cache metadata, cover file count and avatar
    /// state.
    #[must_use]
    pub fn from_parts(metadata: Option<&LibraryMetadata>, cover_thumbnails: usize,
                      avatar_cached: bool)
                      -> Self {
        let secs = |secs: u64| i64::try_from(secs).unwrap_or(i64::MAX);
        Self { metadata_items: metadata.map_or(0, |m| m.item_count),
               cover_thumbnails,
               avatar_cached,
               metadata_saved_at_secs: metadata.map(|m| secs(m.saved_at_secs)),
               last_item_check_batch_secs: metadata.and_then(|m| m.last_item_check_batch_secs)
                                                   .map(secs) }
    }
}

/// Counts for the cache under `cache` and `covers`, for tests that build their
/// own folders.
#[cfg(test)]
fn read_from(cache: &Path, covers: &Path, avatar_cached: bool) -> CacheCounts {
    let cover_thumbnails = std::fs::read_dir(covers).map(Iterator::count).unwrap_or(0);
    CacheCounts::from_parts(load_library_metadata(cache).as_ref(),
                            cover_thumbnails,
                            avatar_cached)
}

// A failed unwrap is how a test fails.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn with_no_cache_everything_is_zero_or_none() {
        let counts = CacheCounts::from_parts(None, 0, false);

        assert_eq!(counts, CacheCounts::default());
    }

    #[test]
    fn the_metadata_gives_the_item_count_and_the_two_times() {
        let metadata = LibraryMetadata { saved_at_secs:                   1_000,
                                       item_count:                      42,
                                       schema_version:                  0,
                                       last_item_check_batch_secs:      Some(2_000),
                                       last_fresh_install_request_secs: None, };

        let counts = CacheCounts::from_parts(Some(&metadata), 7, true);

        assert_eq!(counts.metadata_items, 42);
        assert_eq!(counts.cover_thumbnails, 7);
        assert!(counts.avatar_cached);
        assert_eq!(counts.metadata_saved_at_secs, Some(1_000));
        assert_eq!(counts.last_item_check_batch_secs, Some(2_000));
    }

    #[test]
    fn covers_in_a_folder_are_counted() {
        let folder = tempfile::tempdir().unwrap();
        std::fs::write(folder.path().join("a.jpg"), b"x").unwrap();
        std::fs::write(folder.path().join("b.jpg"), b"x").unwrap();
        let missing = folder.path().join("none");

        let counts = read_from(&missing, folder.path(), false);

        assert_eq!(counts.cover_thumbnails, 2);
        assert_eq!(counts.metadata_items, 0);
    }
}
