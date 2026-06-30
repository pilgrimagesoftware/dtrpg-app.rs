//! Data model, filtering, sorting, and stub catalog for the Libri library view.

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
    /// Filter to items belonging to a named DTRPG product list.
    Collection(Arc<str>),
}
