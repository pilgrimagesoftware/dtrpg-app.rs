//! `LibraryIndex`'s filter/search/sort query: dimensions in `clause`, list
//! and count queries in `query`, and the equivalence bar for the SQL search
//! box in `parity_tests`.
//!
//! Split out of the original `filtered_items.rs` under
//! `openspec/changes/search-box-semantics-sql`.

mod clause;
mod facets;
mod query;

#[cfg(test)]
// Index fixtures use unwrap for known-good in-memory database operations.
#[allow(clippy::unwrap_used)]
mod facet_tests;
#[cfg(test)]
// Index fixtures use unwrap for known-good in-memory database operations.
#[allow(clippy::unwrap_used)]
mod group_parity_tests;
#[cfg(test)]
// Index fixtures use unwrap for known-good in-memory database operations.
#[allow(clippy::unwrap_used)]
mod parity_tests;

pub use clause::ItemFilter;
pub use facets::FacetCount;
pub(super) use facets::facet_counts;
pub(super) use query::{
    count_items_filtered, list_items_filtered, list_items_for_grouping_by_publisher,
    list_items_for_grouping_by_tag,
};
