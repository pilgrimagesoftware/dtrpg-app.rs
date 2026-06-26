//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use std::sync::Arc;
use crate::data::library::LibraryItem;
use crate::data::enums::ItemStatus;
use crate::data::constants::RECENTLY_ADDED_THRESHOLD;

// ── Publisher aggregation ─────────────────────────────────────────────────────

/// A publisher entry with the count of its items in the current result set.
#[derive(Debug, Clone)]
pub struct PublisherEntry {
    pub name: Arc<str>,
    pub count: usize,
}

/// Returns publisher entries sorted by count descending, then name ascending.
#[must_use]
pub fn publisher_entries(items: &[LibraryItem]) -> Vec<PublisherEntry> {
    let mut map: std::collections::HashMap<Arc<str>, usize> =
        std::collections::HashMap::new();
    for item in items {
        *map.entry(Arc::clone(&item.publisher)).or_insert(0) += 1;
    }
    let mut entries: Vec<PublisherEntry> = map
        .into_iter()
        .map(|(name, count)| PublisherEntry { name, count })
        .collect();
    entries.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));
    entries
}

// ── Grouped view ──────────────────────────────────────────────────────────────

/// A publisher group containing its items (already filtered and sorted).
#[derive(Debug, Clone)]
pub struct PublisherGroup {
    pub publisher: Arc<str>,
    pub items: Vec<LibraryItem>,
}

/// Partitions `items` into publisher groups in the same order as `publisher_entries`.
#[must_use]
pub fn group_by_publisher(items: Vec<LibraryItem>) -> Vec<PublisherGroup> {
    let entries = publisher_entries(&items);
    let mut map: std::collections::HashMap<Arc<str>, Vec<LibraryItem>> =
        std::collections::HashMap::new();
    for item in items {
        map.entry(Arc::clone(&item.publisher))
            .or_default()
            .push(item);
    }
    entries
        .into_iter()
        .filter_map(|e| {
            map.remove(&e.name).map(|group_items| PublisherGroup {
                publisher: e.name,
                items: group_items,
            })
        })
        .collect()
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
