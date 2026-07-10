//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::data::constants::{RECENTLY_ADDED_THRESHOLD, THUMBNAIL_COOLDOWN_SECS};
use crate::data::enums::ItemStatus;

// ── LibraryItem
// ───────────────────────────────────────────────────────────────

/// `serde(default)` helper for [`LibraryItem::is_available`] — serde's `bool`
/// default is `false`, but a missing field here means the cache predates the
/// flag and every item in it was current as of the last successful sync.
fn default_true() -> bool {
    true
}

/// A single item in the RPG catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    /// Stable unique identifier (e.g. `"b1"`).
    pub id:                        Arc<str>,
    /// Numeric API identifier used for SDK detail fetches.
    pub numeric_id:                u64,
    /// The `orderProductId` from the API — used for collection membership
    /// matching.
    #[serde(default)]
    pub order_product_id:          u64,
    /// The `productId` from the API — used as a fallback for collection
    /// membership matching when product list items return `productId`
    /// rather than `orderProductId`.
    #[serde(default)]
    pub product_id:                u64,
    pub title:                     Arc<str>,
    pub publisher:                 Arc<str>,
    /// Game line / series name.
    pub line:                      Arc<str>,
    /// Category tag: Core, Supplement, Adventure, Bestiary, Setting …
    pub kind:                      Arc<str>,
    /// File format string, e.g. `"PDF"` or `"PDF + EPUB"`.
    pub format:                    Arc<str>,
    pub pages:                     u32,
    pub size_mb:                   f64,
    pub year:                      u32,
    /// Relative ordering — lower means more recently added.
    pub added_order:               u32,
    pub status:                    ItemStatus,
    /// Hex color string for the generative cover background, e.g. `"#1C2A44"`.
    pub color:                     Arc<str>,
    pub desc:                      Arc<str>,
    /// Optional URL for a real cover thumbnail.
    #[serde(default)]
    pub cover_url:                 Option<Arc<str>>,
    /// Unix timestamp (seconds since epoch) when the item was added to the
    /// library.
    #[serde(default)]
    pub date_added:                Option<i64>,
    /// Unix timestamp (seconds since epoch) when the item's files were last
    /// updated by the publisher, if known.
    #[serde(default)]
    pub date_updated:              Option<i64>,
    /// Last time a thumbnail fetch was attempted for this item; not persisted
    /// to cache.
    #[serde(skip)]
    pub thumbnail_last_attempted:  Option<std::time::SystemTime>,
    /// Whether this item was present in the most recent successful live
    /// catalog fetch. `false` means the server no longer lists it, but the
    /// item is kept (not deleted) so previously-downloaded files stay
    /// reachable. Defaults to `true` for cache files written before this
    /// field existed and for items newly added from a live fetch.
    #[serde(default = "default_true")]
    pub is_available:              bool,
    /// Last time an individual server check
    /// (`catalog-item-level-reconciliation`) ran for this item; not
    /// persisted to cache.
    #[serde(skip)]
    pub availability_last_checked: Option<std::time::SystemTime>,
    /// Per-item files bundled in this catalog entry, mapped from the SDK's
    /// `OrderProductFile` array. More than one entry means this is a
    /// multi-item catalog entry (see the `catalog-entry-detail-view`
    /// capability). Defaults to empty for cache entries written before this
    /// field existed.
    #[serde(default)]
    pub files:                     Vec<LibraryItemFile>,
}

impl LibraryItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(id: &str, title: &str, publisher: &str, line: &str, kind: &str, format: &str,
               pages: u32, size_mb: f64, year: u32, added_order: u32, status: ItemStatus,
               color: &str, desc: &str, date_added: Option<i64>)
               -> Self {
        Self { id: id.into(),
               numeric_id: 0,
               order_product_id: 0,
               product_id: 0,
               title: title.into(),
               publisher: publisher.into(),
               line: line.into(),
               kind: kind.into(),
               format: format.into(),
               pages,
               size_mb,
               year,
               added_order,
               status,
               color: color.into(),
               desc: desc.into(),
               cover_url: None,
               date_added,
               date_updated: None,
               thumbnail_last_attempted: None,
               is_available: true,
               availability_last_checked: None,
               files: Vec::new() }
    }

    /// Returns `true` if this catalog entry bundles more than one
    /// downloadable file (a "multi-item" entry per
    /// `catalog-entry-detail-view`).
    #[must_use]
    pub fn is_multi_item(&self) -> bool {
        self.files.len() > 1
    }

    /// Removes duplicate entries from `files`, keeping the first occurrence
    /// of each unique `(id, name)` pair.
    ///
    /// The DriveThruRPG API has been observed to repeat a download record
    /// (identical `id` *and* `name`) verbatim across `files` for what is
    /// genuinely a single file; the SDK mapping layer (`map_order_product`)
    /// already dedupes those exact repeats on ingest, but catalog data
    /// cached to disk before that fix still has them. Without this, every
    /// row in the detail tab's item list compares equal by `id`, so
    /// selecting one row highlights all of them and further clicks appear to
    /// do nothing.
    ///
    /// `id` alone is deliberately NOT used as the key: the API has also been
    /// observed to reuse the same download id across genuinely distinct
    /// files within a multi-file bundle, so deduplicating on `id` alone
    /// would collapse a real bundle down to one file and hide its
    /// item-count badge (`catalog-entry-detail-view`). Requiring `name` to
    /// match too means two files are only merged when they're truly
    /// identical repeats.
    ///
    /// Call this on any `LibraryItem` loaded from a source this crate does
    /// not fully control (e.g. the on-disk catalog cache).
    pub fn dedupe_files(&mut self) {
        let mut seen = std::collections::HashSet::new();
        self.files
            .retain(|f| seen.insert((Arc::clone(&f.id), Arc::clone(&f.name))));
    }
}

// ── LibraryItemFile
// ───────────────────────────────────────────────────────

/// A single downloadable file within a catalog entry, e.g. the book or the
/// map sheet inside a bundled product like Moria.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LibraryItemFile {
    /// Stable identifier for this file within its entry (the SDK's
    /// `orderProductDownloadId`).
    pub id:      Arc<str>,
    /// Display name of the file (the SDK file's `title`, e.g.
    /// `"Player's Handbook.pdf"`).
    pub name:    Arc<str>,
    /// Uppercase format label derived from the filename extension (e.g.
    /// `"PDF"`).
    pub format:  Arc<str>,
    /// File size in megabytes.
    pub size_mb: f64,
}

// ── Smart section counts
// ──────────────────────────────────────────────────────

/// Counts for each smart sidebar section given the full catalog.
#[derive(Debug, Clone, Copy, Default)]
pub struct SectionCounts {
    pub all:            usize,
    pub recently_added: usize,
    pub on_device:      usize,
    pub in_cloud:       usize,
}

/// Computes smart section counts from the full catalog.
#[must_use]
pub fn section_counts(catalog: &[LibraryItem]) -> SectionCounts {
    SectionCounts { all:            catalog.len(),
                    recently_added: catalog.iter()
                                           .filter(|i| i.added_order <= RECENTLY_ADDED_THRESHOLD)
                                           .count(),
                    on_device:      catalog.iter()
                                           .filter(|i| i.status == ItemStatus::Downloaded)
                                           .count(),
                    in_cloud:       catalog.iter()
                                           .filter(|i| i.status == ItemStatus::Cloud)
                                           .count(), }
}

// ── Thumbnail cooldown
// ────────────────────────────────────────────────────────

/// Returns `true` if no thumbnail fetch has been attempted, or the last attempt
/// was more than 5 minutes ago.
#[must_use]
pub fn thumbnail_cooldown_elapsed(item: &LibraryItem) -> bool {
    let Some(last) = item.thumbnail_last_attempted
    else {
        return true;
    };
    std::time::SystemTime::now().duration_since(last)
                                .is_ok_and(|d| d.as_secs() >= THUMBNAIL_COOLDOWN_SECS)
}

// ── Footer totals
// ─────────────────────────────────────────────────────────────

/// Formats `bytes` as a human-readable size string (GB or MB).
#[must_use]
pub fn format_total_size(items: &[LibraryItem]) -> String {
    let total_mb: f64 = items.iter().map(|i| i.size_mb).sum();
    if total_mb >= 1024.0 {
        format!("{:.1} GB", total_mb / 1024.0)
    }
    else {
        format!("{:.0} MB", total_mb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(id: &str, name: &str) -> LibraryItemFile {
        LibraryItemFile { id:      id.into(),
                          name:    name.into(),
                          format:  "PDF".into(),
                          size_mb: 1.0, }
    }

    #[test]
    fn dedupe_files_removes_repeated_ids() {
        let mut item = LibraryItem::new("e1",
                                        "Moria",
                                        "Free League",
                                        "",
                                        "",
                                        "",
                                        0,
                                        0.0,
                                        0,
                                        0,
                                        ItemStatus::Cloud,
                                        "#000000",
                                        "",
                                        None);
        item.files = vec![file("1234", "Moria Rulebook"),
                          file("1234", "Moria Rulebook")];

        item.dedupe_files();

        assert_eq!(item.files.len(), 1);
        assert!(!item.is_multi_item());
    }

    #[test]
    fn is_available_defaults_to_true_when_field_is_missing() {
        let json = serde_json::json!({
            "id": "e1",
            "numeric_id": 0,
            "title": "Moria",
            "publisher": "Free League",
            "line": "",
            "kind": "",
            "format": "",
            "pages": 0,
            "size_mb": 0.0,
            "year": 0,
            "added_order": 0,
            "status": "Cloud",
            "color": "#000000",
            "desc": "",
        });

        let item: LibraryItem = match serde_json::from_value(json) {
            Ok(item) => item,
            Err(e) => panic!("deserializes: {e}"),
        };

        assert!(item.is_available);
    }

    #[test]
    fn dedupe_files_keeps_distinct_ids() {
        let mut item = LibraryItem::new("e1",
                                        "Moria",
                                        "Free League",
                                        "",
                                        "",
                                        "",
                                        0,
                                        0.0,
                                        0,
                                        0,
                                        ItemStatus::Cloud,
                                        "#000000",
                                        "",
                                        None);
        item.files = vec![file("1234", "Moria Rulebook"),
                          file("1235", "Moria Map Sheet")];

        item.dedupe_files();

        assert_eq!(item.files.len(), 2);
        assert!(item.is_multi_item());
    }

    #[test]
    fn dedupe_files_keeps_distinct_files_that_share_an_id() {
        // Regression: the API has been observed to reuse the same download
        // id across genuinely distinct files within a bundle. Deduping on
        // `id` alone would collapse this real 2-file bundle down to 1 and
        // hide its item-count badge — the `name` must differ, not just the
        // id, before two entries are treated as duplicates.
        let mut item = LibraryItem::new("e1",
                                        "Moria",
                                        "Free League",
                                        "",
                                        "",
                                        "",
                                        0,
                                        0.0,
                                        0,
                                        0,
                                        ItemStatus::Cloud,
                                        "#000000",
                                        "",
                                        None);
        item.files = vec![file("1234", "Moria Rulebook"),
                          file("1234", "Moria Map Sheet")];

        item.dedupe_files();

        assert_eq!(item.files.len(), 2);
        assert!(item.is_multi_item());
    }
}
