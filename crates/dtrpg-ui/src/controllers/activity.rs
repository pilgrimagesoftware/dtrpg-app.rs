//! Activity controller: tracks in-progress and recently-completed background operations.

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use gpui::Context;

use crate::data::activity::{
    ActivityItem, ActivitySnapshot, ActivityStatus, AlertEntry, AlertHistorySnapshot,
};
use crate::data::constants::{ALERT_LOG_CAP, ERROR_EXPIRY_SECS, EXPIRY_SECS, RECENT_CAP};
use crate::data::events::{ActivityChanged, DownloadComplete, DownloadError};

/// Owns the activity item list and panel open/close state.
pub struct ActivityController {
    next_id: u64,
    in_progress: Vec<ActivityItem>,
    recent: VecDeque<ActivityItem>,
    panel_open: bool,
    /// The id of the item row currently expanded in the panel, if any.
    selected_id: Option<u64>,
    /// Durable, non-expiring log of error-status activity items.
    ///
    /// Never cleared by [`clear`](Self::clear) — this is a session-long record
    /// independent of the active service, capped at [`ALERT_LOG_CAP`].
    alert_log: VecDeque<AlertEntry>,
    /// Whether the alert history panel overlay is open.
    alert_panel_open: bool,
}

impl ActivityController {
    /// Creates a new controller with no items and the panel closed.
    pub fn new() -> Self {
        Self {
            next_id: 0,
            in_progress: Vec::new(),
            recent: VecDeque::with_capacity(RECENT_CAP),
            panel_open: false,
            selected_id: None,
            alert_log: VecDeque::with_capacity(ALERT_LOG_CAP),
            alert_panel_open: false,
        }
    }

    /// Clears all activity items (in-progress and recent) and closes the panel.
    ///
    /// Used when replacing the library service so stale error messages don't persist.
    /// The durable alert history log is intentionally left intact.
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.in_progress.clear();
        self.recent.clear();
        self.selected_id = None;
        cx.emit(ActivityChanged);
    }

    /// Registers a new in-progress operation with the given label.
    ///
    /// `cancel_fn` is an optional callback the UI invokes when the user clicks the cancel button.
    /// Pass `None` for operations that do not support cancellation.
    ///
    /// Returns the assigned activity id, which the caller must pass to [`complete`] or [`error`].
    pub fn start(
        &mut self,
        label: &str,
        cancel_fn: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
        cx: &mut Context<Self>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.in_progress.push(ActivityItem {
            id,
            label: Arc::from(label),
            status: ActivityStatus::InProgress,
            started_at: Instant::now(),
            elapsed_secs: None,
            progress: None,
            cancel_fn,
        });
        cx.emit(ActivityChanged);
        id
    }

    /// Updates the display label of an in-progress item.
    ///
    /// No-op if `id` is not found in the in-progress list.
    pub fn update_label(&mut self, id: u64, label: impl Into<String>, cx: &mut Context<Self>) {
        if let Some(item) = self.in_progress.iter_mut().find(|i| i.id == id) {
            let s: String = label.into();
            item.label = Arc::from(s.as_str());
            cx.emit(ActivityChanged);
        }
    }

    /// Resolves an in-progress item as successfully completed and schedules its 15-second expiry.
    ///
    /// No-op if `id` is not found in the in-progress list.
    pub fn complete(&mut self, id: u64, cx: &mut Context<Self>) {
        if let Some(pos) = self.in_progress.iter().position(|i| i.id == id) {
            let mut item = self.in_progress.remove(pos);
            item.elapsed_secs = Some(item.started_at.elapsed().as_secs());
            item.status = ActivityStatus::Complete;
            let title = item.label.clone();
            self.push_recent(item);
            cx.emit(ActivityChanged);
            cx.emit(DownloadComplete { title });
            cx.spawn(async move |this, async_cx| {
                async_cx
                    .background_executor()
                    .timer(Duration::from_secs(EXPIRY_SECS))
                    .await;
                this.update(async_cx, |a, cx| a.expire_item(id, cx)).ok();
            })
            .detach();
        }
    }

    /// Resolves an in-progress item as failed and schedules its 2-minute expiry.
    ///
    /// No-op if `id` is not found in the in-progress list.
    pub fn error(&mut self, id: u64, message: String, cx: &mut Context<Self>) {
        if let Some(pos) = self.in_progress.iter().position(|i| i.id == id) {
            let mut item = self.in_progress.remove(pos);
            item.elapsed_secs = Some(item.started_at.elapsed().as_secs());
            let title = item.label.clone();
            item.status = ActivityStatus::Error(message.clone());
            self.push_alert(AlertEntry {
                id: item.id,
                label: title.clone(),
                message: message.clone(),
                occurred_at: SystemTime::now(),
            });
            self.push_recent(item);
            cx.emit(ActivityChanged);
            cx.emit(DownloadError { title, message });
            cx.spawn(async move |this, async_cx| {
                async_cx
                    .background_executor()
                    .timer(Duration::from_secs(ERROR_EXPIRY_SECS))
                    .await;
                this.update(async_cx, |a, cx| a.expire_item(id, cx)).ok();
            })
            .detach();
        }
    }

    /// Removes an expired item from the recent list by id.
    ///
    /// No-op if the item was already evicted by the cap or a prior expiry. Emits
    /// [`ActivityChanged`] only when an item is actually removed.
    pub fn expire_item(&mut self, id: u64, cx: &mut Context<Self>) {
        let before = self.recent.len();
        self.recent.retain(|i| i.id != id);
        if self.recent.len() != before {
            if self.selected_id == Some(id) {
                self.selected_id = None;
            }
            cx.emit(ActivityChanged);
        }
    }

    /// Toggles the expanded row in the activity panel.
    ///
    /// Selects `id` if it differs from the current selection, or collapses if the same id.
    pub fn select_activity(&mut self, id: u64, cx: &mut Context<Self>) {
        self.selected_id = if self.selected_id == Some(id) {
            None
        } else {
            Some(id)
        };
        cx.emit(ActivityChanged);
    }

    /// Updates the progress value for an in-progress item, clamped to [0.0, 1.0].
    ///
    /// No-op if `id` is not found in the in-progress list.
    pub fn update_progress(&mut self, id: u64, progress: f32, cx: &mut Context<Self>) {
        if let Some(item) = self.in_progress.iter_mut().find(|i| i.id == id) {
            item.progress = Some(progress.clamp(0.0, 1.0));
            cx.emit(ActivityChanged);
        }
    }

    /// Calls the stored cancel function (if any) and transitions the item to Error("Cancelled").
    ///
    /// No-op if `id` is not found in the in-progress list.
    pub fn cancel_activity(&mut self, id: u64, cx: &mut Context<Self>) {
        if let Some(item) = self.in_progress.iter().find(|i| i.id == id)
            && let Some(cancel_fn) = item.cancel_fn.clone()
        {
            cancel_fn();
        }
        self.error(id, "Cancelled".to_string(), cx);
    }

    /// Toggles the activity panel overlay open or closed.
    pub fn toggle_panel(&mut self, cx: &mut Context<Self>) {
        self.panel_open = !self.panel_open;
        cx.emit(ActivityChanged);
    }

    /// Sets the activity panel overlay's open state directly.
    ///
    /// Used by the status bar's anchored `Popover`, which reports its own resolved
    /// open state on each toggle rather than expecting the caller to flip a bool.
    pub fn set_panel_open(&mut self, open: bool, cx: &mut Context<Self>) {
        if self.panel_open != open {
            self.panel_open = open;
            cx.emit(ActivityChanged);
        }
    }

    /// Toggles the alert history panel overlay open or closed.
    pub fn toggle_alert_panel(&mut self, cx: &mut Context<Self>) {
        self.alert_panel_open = !self.alert_panel_open;
        cx.emit(ActivityChanged);
    }

    /// Sets the alert history panel overlay's open state directly.
    ///
    /// Used by the status bar's anchored `Popover`, which reports its own resolved
    /// open state on each toggle rather than expecting the caller to flip a bool.
    pub fn set_alert_panel_open(&mut self, open: bool, cx: &mut Context<Self>) {
        if self.alert_panel_open != open {
            self.alert_panel_open = open;
            cx.emit(ActivityChanged);
        }
    }

    /// Removes all entries from the durable alert history log.
    pub fn clear_alert_log(&mut self, cx: &mut Context<Self>) {
        self.alert_log.clear();
        cx.emit(ActivityChanged);
    }

    /// Returns a snapshot of the alert history log for rendering.
    #[must_use]
    pub fn alert_snapshot(&self) -> AlertHistorySnapshot {
        AlertHistorySnapshot {
            open: self.alert_panel_open,
            entries: self.alert_log.iter().cloned().collect(),
        }
    }

    /// Returns a snapshot of current activity state for rendering.
    pub fn snapshot(&self) -> ActivitySnapshot {
        let recent_error_count = self
            .recent
            .iter()
            .filter(|i| matches!(i.status, ActivityStatus::Error(_)))
            .count();

        let mut items: Vec<ActivityItem> = self.in_progress.clone();
        items.extend(self.recent.iter().cloned());

        ActivitySnapshot {
            in_progress_count: self.in_progress.len(),
            recent_error_count,
            recent_count: self.recent.len(),
            panel_open: self.panel_open,
            items,
            selected_id: self.selected_id,
            aggregate_progress: self.aggregate_progress(),
        }
    }

    /// Computes the mean of known `progress` values among in-progress items.
    ///
    /// Returns `None` (indeterminate) when there are no in-progress items, or when
    /// none of them report a known progress value.
    fn aggregate_progress(&self) -> Option<f32> {
        let known: Vec<f32> = self.in_progress.iter().filter_map(|i| i.progress).collect();
        if known.is_empty() {
            return None;
        }
        Some(known.iter().sum::<f32>() / known.len() as f32)
    }

    fn push_recent(&mut self, item: ActivityItem) {
        if self.recent.len() == RECENT_CAP {
            self.recent.pop_back();
        }
        self.recent.push_front(item);
    }

    /// Appends an entry to the durable alert log, evicting the oldest entry if full.
    fn push_alert(&mut self, entry: AlertEntry) {
        if self.alert_log.len() == ALERT_LOG_CAP {
            self.alert_log.pop_back();
        }
        self.alert_log.push_front(entry);
    }
}

impl Default for ActivityController {
    fn default() -> Self {
        Self::new()
    }
}
