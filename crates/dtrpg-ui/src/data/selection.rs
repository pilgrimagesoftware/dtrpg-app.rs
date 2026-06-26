use std::sync::Arc;

// ── Selection ─────────────────────────────────────────────────────────────────

/// What is currently selected in the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    None,
    Item(Arc<str>),
}

impl Default for Selection {
    fn default() -> Self {
        Self::None
    }
}
