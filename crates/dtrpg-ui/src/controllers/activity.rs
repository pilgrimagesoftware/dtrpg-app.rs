//! Activity controller: tracks in-progress and recently-completed background operations.

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use gpui::Context;

use crate::data::activity::{ActivityItem, ActivitySnapshot, ActivityStatus};
use crate::data::events::ActivityChanged;

const RECENT_CAP: usize = 25;
const EXPIRY_SECS: u64 = 15;
const ERROR_EXPIRY_SECS: u64 = 120;

/// Owns the activity item list and panel open/close state.
pub struct ActivityController {
    next_id: u64,
    in_progress: Vec<ActivityItem>,
    recent: VecDeque<ActivityItem>,
    panel_open: bool,
    /// The id of the item row currently expanded in the panel, if any.
    selected_id: Option<u64>,
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
            item.status = ActivityStatus::Complete;
            self.push_recent(item);
            cx.emit(ActivityChanged);
            cx.spawn(async move |this, async_cx| {
                async_cx.background_executor().timer(Duration::from_secs(EXPIRY_SECS)).await;
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
            item.status = ActivityStatus::Error(message);
            self.push_recent(item);
            cx.emit(ActivityChanged);
            cx.spawn(async move |this, async_cx| {
                async_cx.background_executor().timer(Duration::from_secs(ERROR_EXPIRY_SECS)).await;
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
            recent_count: self.recent.len(),
            panel_open: self.panel_open,
            items,
            selected_id: self.selected_id,
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
