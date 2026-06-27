//! Activity data model for tracking background operations.

use std::sync::Arc;

/// The lifecycle state of a single background operation.
#[derive(Debug, Clone)]
pub enum ActivityStatus {
    /// The operation is still running.
    InProgress,
    /// The operation finished successfully.
    Complete,
    /// The operation finished with an error.
    Error(String),
}

/// A single tracked background operation.
#[derive(Debug, Clone)]
pub struct ActivityItem {
    /// Unique monotonically-increasing identifier assigned by [`super::super::controllers::activity::ActivityController`].
    pub id: u64,
    /// Human-readable label shown in the activity panel.
    pub label: Arc<str>,
    /// Current lifecycle state.
    pub status: ActivityStatus,
}

/// Snapshot of all activity state needed by the root view for one render pass.
pub struct ActivitySnapshot {
    /// Number of currently in-progress operations.
    pub in_progress_count: usize,
    /// Number of recently-completed operations that ended with an error.
    pub recent_error_count: usize,
    /// Number of items in the recent list (excludes expired items).
    pub recent_count: usize,
    /// Whether the activity panel overlay is open.
    pub panel_open: bool,
    /// Combined item list — in-progress items first, then recent (newest first).
    pub items: Vec<ActivityItem>,
    /// The id of the currently expanded item row, if any.
    pub selected_id: Option<u64>,
}
