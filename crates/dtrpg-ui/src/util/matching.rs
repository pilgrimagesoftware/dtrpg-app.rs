//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use crate::util::filter::{SidebarFilter};
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::constants::RECENTLY_ADDED_THRESHOLD;

// ── Matching functions ─────────────────────────────────────────────────────────────────

/// Returns `true` if `item` passes the given sidebar filter.
#[must_use]
pub fn item_matches_filter(item: &LibraryItem, filter: &SidebarFilter) -> bool {
    match filter {
        SidebarFilter::AllTitles => true,
        SidebarFilter::RecentlyAdded => item.added_order <= RECENTLY_ADDED_THRESHOLD,
        SidebarFilter::OnDevice => item.status == ItemStatus::Downloaded,
        SidebarFilter::InCloud => item.status == ItemStatus::Cloud,
        SidebarFilter::Publisher(name) => item.publisher.as_ref() == name.as_ref(),
    }
}

/// Returns `true` if `item` contains `query` in title, publisher, or game line
/// (case-insensitive).
#[must_use]
pub fn item_matches_query(item: &LibraryItem, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let q = query.to_lowercase();
    item.title.to_lowercase().contains(&q)
        || item.publisher.to_lowercase().contains(&q)
        || item.line.to_lowercase().contains(&q)
}
