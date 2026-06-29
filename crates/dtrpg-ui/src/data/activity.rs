//! Activity data model for tracking background operations.

use std::sync::Arc;
use std::time::Instant;

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
#[derive(Clone)]
pub struct ActivityItem {
    /// Unique monotonically-increasing identifier assigned by [`ActivityController`].
    pub id: u64,
    /// Human-readable label shown in the activity panel.
    pub label: Arc<str>,
    /// Current lifecycle state.
    pub status: ActivityStatus,
    /// Monotonic timestamp captured when the item was started.
    pub started_at: Instant,
    /// Total duration in seconds, frozen when the item leaves InProgress; None while running.
    pub elapsed_secs: Option<u64>,
    /// Reported progress in [0.0, 1.0]; None means indeterminate.
    pub progress: Option<f32>,
    /// Optional callback invoked when the user cancels this item.
    pub cancel_fn: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
}

impl std::fmt::Debug for ActivityItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActivityItem")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("status", &self.status)
            .field("started_at", &self.started_at)
            .field("elapsed_secs", &self.elapsed_secs)
            .field("progress", &self.progress)
            .field("has_cancel_fn", &self.cancel_fn.is_some())
            .finish()
    }
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
