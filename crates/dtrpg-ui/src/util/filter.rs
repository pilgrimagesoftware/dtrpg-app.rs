//! Data model, filtering, sorting, and stub catalog for the Libri library view.

use std::sync::Arc;

// ── Filtering ─────────────────────────────────────────────────────────────────

/// Active filter applied in the sidebar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarFilter {
    AllTitles,
    RecentlyAdded,
    OnDevice,
    InCloud,
    Publisher(Arc<str>),
}

impl Default for SidebarFilter {
    fn default() -> Self {
        Self::AllTitles
    }
}
