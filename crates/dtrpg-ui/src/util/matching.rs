//! Item filtering and text matching predicates.

use std::collections::HashSet;

use crate::data::constants::RECENTLY_ADDED_THRESHOLD;
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::util::filter::SidebarFilter;

// ── Matching functions ─────────────────────────────────────────────────────────────────

/// Returns `true` if `item` passes the given sidebar filter.
///
/// For `Collection` filters, `collection_members` contains `productId` values from the
/// product-list-items API. The item passes if either `order_product_id` or `product_id` matches,
/// since the API returns `productId` while catalog items carry both IDs.
#[must_use]
pub fn item_matches_filter(
    item: &LibraryItem,
    filter: &SidebarFilter,
    collection_members: &HashSet<u64>,
) -> bool {
    match filter {
        SidebarFilter::AllTitles => true,
        SidebarFilter::RecentlyAdded => item.added_order <= RECENTLY_ADDED_THRESHOLD,
        SidebarFilter::OnDevice => item.status == ItemStatus::Downloaded,
        SidebarFilter::InCloud => item.status == ItemStatus::Cloud,
        SidebarFilter::Publisher(name) => item.publisher.as_ref() == name.as_ref(),
        SidebarFilter::Collection(_, _) => {
            (item.order_product_id > 0 && collection_members.contains(&item.order_product_id))
                || (item.product_id > 0 && collection_members.contains(&item.product_id))
        }
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
