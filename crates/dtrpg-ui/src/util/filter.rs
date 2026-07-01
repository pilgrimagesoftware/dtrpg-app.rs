//! Active sidebar filter variants.

use std::sync::Arc;

// ── Filtering ─────────────────────────────────────────────────────────────────

/// Active filter applied in the sidebar.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SidebarFilter {
    #[default]
    AllTitles,
    RecentlyAdded,
    OnDevice,
    InCloud,
    Publisher(Arc<str>),
    /// Filter to items belonging to the DTRPG product list with this numeric id.
    Collection(u64),
}
