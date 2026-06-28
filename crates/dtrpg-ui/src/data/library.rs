//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use std::sync::Arc;
use crate::data::enums::ItemStatus;
use crate::data::constants::RECENTLY_ADDED_THRESHOLD;

// ── LibraryItem ───────────────────────────────────────────────────────────────

/// A single item in the RPG catalog.
#[derive(Debug, Clone)]
pub struct LibraryItem {
    /// Stable unique identifier (e.g. `"b1"`).
    pub id: Arc<str>,
    /// Numeric API identifier used for SDK detail fetches.
    pub numeric_id: u64,
    pub title: Arc<str>,
    pub publisher: Arc<str>,
    /// Game line / series name.
    pub line: Arc<str>,
    /// Category tag: Core, Supplement, Adventure, Bestiary, Setting …
    pub kind: Arc<str>,
    /// File format string, e.g. `"PDF"` or `"PDF + EPUB"`.
    pub format: Arc<str>,
    pub pages: u32,
    pub size_mb: f64,
    pub year: u32,
    /// Relative ordering — lower means more recently added.
    pub added_order: u32,
    pub status: ItemStatus,
    /// Hex color string for the generative cover background, e.g. `"#1C2A44"`.
    pub color: Arc<str>,
    pub desc: Arc<str>,
    /// Optional URL for a real cover thumbnail.
    pub cover_url: Option<Arc<str>>,
    /// Unix timestamp (seconds since epoch) when the item was added to the library.
    pub date_added: Option<i64>,
}

impl LibraryItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        title: &str,
        publisher: &str,
        line: &str,
        kind: &str,
        format: &str,
        pages: u32,
        size_mb: f64,
        year: u32,
        added_order: u32,
        status: ItemStatus,
        color: &str,
        desc: &str,
        date_added: Option<i64>,
    ) -> Self {
        Self {
            id: id.into(),
            numeric_id: 0,
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
        }
    }
}

// ── Smart section counts ──────────────────────────────────────────────────────

/// Counts for each smart sidebar section given the full catalog.
#[derive(Debug, Clone, Copy, Default)]
pub struct SectionCounts {
    pub all: usize,
    pub recently_added: usize,
    pub on_device: usize,
    pub in_cloud: usize,
}

/// Computes smart section counts from the full catalog.
#[must_use]
pub fn section_counts(catalog: &[LibraryItem]) -> SectionCounts {
    SectionCounts {
        all: catalog.len(),
        recently_added: catalog
            .iter()
            .filter(|i| i.added_order <= RECENTLY_ADDED_THRESHOLD)
            .count(),
        on_device: catalog
            .iter()
            .filter(|i| i.status == ItemStatus::Downloaded)
            .count(),
        in_cloud: catalog
            .iter()
            .filter(|i| i.status == ItemStatus::Cloud)
            .count(),
    }
}

// ── Footer totals ─────────────────────────────────────────────────────────────

/// Formats `bytes` as a human-readable size string (GB or MB).
#[must_use]
pub fn format_total_size(items: &[LibraryItem]) -> String {
    let total_mb: f64 = items.iter().map(|i| i.size_mb).sum();
    if total_mb >= 1024.0 {
        format!("{:.1} GB", total_mb / 1024.0)
    } else {
        format!("{:.0} MB", total_mb)
    }
}
