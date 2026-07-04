//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::data::constants::RECENTLY_ADDED_THRESHOLD;
use crate::data::enums::ItemStatus;

// ── LibraryItem
// ───────────────────────────────────────────────────────────────

/// A single item in the RPG catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    /// Stable unique identifier (e.g. `"b1"`).
    pub id:                       Arc<str>,
    /// Numeric API identifier used for SDK detail fetches.
    pub numeric_id:               u64,
    /// The `orderProductId` from the API — used for collection membership
    /// matching.
    #[serde(default)]
    pub order_product_id:         u64,
    /// The `productId` from the API — used as a fallback for collection
    /// membership matching when product list items return `productId`
    /// rather than `orderProductId`.
    #[serde(default)]
    pub product_id:               u64,
    pub title:                    Arc<str>,
    pub publisher:                Arc<str>,
    /// Game line / series name.
    pub line:                     Arc<str>,
    /// Category tag: Core, Supplement, Adventure, Bestiary, Setting …
    pub kind:                     Arc<str>,
    /// File format string, e.g. `"PDF"` or `"PDF + EPUB"`.
    pub format:                   Arc<str>,
    pub pages:                    u32,
    pub size_mb:                  f64,
    pub year:                     u32,
    /// Relative ordering — lower means more recently added.
    pub added_order:              u32,
    pub status:                   ItemStatus,
    /// Hex color string for the generative cover background, e.g. `"#1C2A44"`.
    pub color:                    Arc<str>,
    pub desc:                     Arc<str>,
    /// Optional URL for a real cover thumbnail.
    #[serde(default)]
    pub cover_url:                Option<Arc<str>>,
    /// Unix timestamp (seconds since epoch) when the item was added to the
    /// library.
    #[serde(default)]
    pub date_added:               Option<i64>,
    /// Last time a thumbnail fetch was attempted for this item; not persisted
    /// to cache.
    #[serde(skip)]
    pub thumbnail_last_attempted: Option<std::time::SystemTime>,
    /// Per-item files bundled in this catalog entry, mapped from the SDK's
    /// `OrderProductFile` array. More than one entry means this is a
    /// multi-item catalog entry (see the `catalog-entry-detail-view`
    /// capability). Defaults to empty for cache entries written before this
    /// field existed.
    #[serde(default)]
    pub files:                    Vec<LibraryItemFile>,
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
               thumbnail_last_attempted: None,
               files: Vec::new() }
    }

    /// Returns `true` if this catalog entry bundles more than one
    /// downloadable file (a "multi-item" entry per
    /// `catalog-entry-detail-view`).
    #[must_use]
    pub fn is_multi_item(&self) -> bool {
        self.files.len() > 1
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

const THUMBNAIL_COOLDOWN_SECS: u64 = 300;

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
