//! Activity controller: tracks in-progress and recently-completed background operations.

use std::collections::VecDeque;
use std::sync::Arc;

use gpui::Context;

use crate::data::activity::{ActivityItem, ActivitySnapshot, ActivityStatus};
use crate::data::events::ActivityChanged;

const RECENT_CAP: usize = 25;

/// Owns the activity item list and panel open/close state.
pub struct ActivityController {
    next_id: u64,
    in_progress: Vec<ActivityItem>,
    recent: VecDeque<ActivityItem>,
    panel_open: bool,
}

impl ActivityController {
    /// Creates a new controller with no items and the panel closed.
    pub fn new() -> Self {
        Self {
            next_id: 0,
            in_progress: Vec::new(),
            recent: VecDeque::with_capacity(RECENT_CAP),
            panel_open: false,
        }
    }

    /// Registers a new in-progress operation with the given label.
    ///
    /// Returns the assigned activity id, which the caller must pass to [`complete`] or [`error`].
    pub fn start(&mut self, label: &str, cx: &mut Context<Self>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.in_progress.push(ActivityItem {
            id,
            label: Arc::from(label),
            status: ActivityStatus::InProgress,
        });
        cx.emit(ActivityChanged);
        id
    }

    /// Resolves an in-progress item as successfully completed.
    ///
    /// No-op if `id` is not found in the in-progress list.
    pub fn complete(&mut self, id: u64, cx: &mut Context<Self>) {
        if let Some(pos) = self.in_progress.iter().position(|i| i.id == id) {
            let mut item = self.in_progress.remove(pos);
            item.status = ActivityStatus::Complete;
            self.push_recent(item);
            cx.emit(ActivityChanged);
        }
    }

    /// Resolves an in-progress item as failed with an error message.
    ///
    /// No-op if `id` is not found in the in-progress list.
    pub fn error(&mut self, id: u64, message: String, cx: &mut Context<Self>) {
        if let Some(pos) = self.in_progress.iter().position(|i| i.id == id) {
            let mut item = self.in_progress.remove(pos);
            item.status = ActivityStatus::Error(message);
            self.push_recent(item);
            cx.emit(ActivityChanged);
        }
    }

    /// Toggles the activity panel overlay open or closed.
    pub fn toggle_panel(&mut self, cx: &mut Context<Self>) {
        self.panel_open = !self.panel_open;
        cx.emit(ActivityChanged);
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
            panel_open: self.panel_open,
            items,
        }
    }

    fn push_recent(&mut self, item: ActivityItem) {
        if self.recent.len() == RECENT_CAP {
            self.recent.pop_back();
        }
        self.recent.push_front(item);
    }
}

impl Default for ActivityController {
    fn default() -> Self {
        Self::new()
    }
}
