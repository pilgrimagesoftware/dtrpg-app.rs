//! Tabs controller: tracks which main-window tabs are open and active.
//!
//! The catalog tab is always open and non-closable. Expanded detail tabs are
//! opened by double-clicking a catalog item and closed via their tab's close
//! button — see the `main-window-tabs` capability.

use std::collections::HashMap;
use std::sync::Arc;

use gpui::Context;

use crate::data::events::TabsChanged;

/// Identifies a tab in the main window's tab strip.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TabTarget {
    /// The always-open, non-closable catalog tab.
    Catalog,
    /// A closable expanded detail tab for the catalog item with this id.
    Detail(Arc<str>),
}

/// Read-only snapshot of tab state for rendering.
#[derive(Clone)]
pub struct TabsSnapshot {
    /// Open tabs, catalog first, in tab-strip order.
    pub open_tabs: Vec<TabTarget>,
    /// The currently active tab.
    pub active: TabTarget,
    /// Display titles for open detail tabs, keyed by item id.
    pub titles: HashMap<Arc<str>, String>,
}

/// Owns the set of open tabs and which one is active.
pub struct TabsController {
    open_tabs: Vec<TabTarget>,
    active: TabTarget,
    titles: HashMap<Arc<str>, String>,
}

impl TabsController {
    /// Creates a new controller with only the catalog tab open and active.
    #[must_use]
    pub fn new() -> Self {
        Self {
            open_tabs: vec![TabTarget::Catalog],
            active: TabTarget::Catalog,
            titles: HashMap::new(),
        }
    }

    /// Returns a snapshot of the current tab state.
    #[must_use]
    pub fn snapshot(&self) -> TabsSnapshot {
        TabsSnapshot {
            open_tabs: self.open_tabs.clone(),
            active: self.active.clone(),
            titles: self.titles.clone(),
        }
    }

    /// Opens (or activates, if already open) an expanded detail tab for `id`.
    pub fn open_detail_tab(&mut self, id: Arc<str>, title: String, cx: &mut Context<Self>) {
        let target = TabTarget::Detail(id.clone());
        if !self.open_tabs.contains(&target) {
            self.open_tabs.push(target.clone());
        }
        self.titles.insert(id, title);
        self.active = target;
        cx.emit(TabsChanged);
    }

    /// Activates an already-open tab; no-op if `target` is not currently open.
    pub fn activate(&mut self, target: TabTarget, cx: &mut Context<Self>) {
        if self.open_tabs.contains(&target) {
            self.active = target;
            cx.emit(TabsChanged);
        }
    }

    /// Closes the detail tab for `id`, falling back to the catalog tab if it
    /// was active. No-op if no such tab is open.
    pub fn close_detail_tab(&mut self, id: &str, cx: &mut Context<Self>) {
        let before = self.open_tabs.len();
        self.open_tabs.retain(|t| match t {
            TabTarget::Detail(open_id) => open_id.as_ref() != id,
            TabTarget::Catalog => true,
        });
        if self.open_tabs.len() != before {
            self.titles.remove(id);
            if matches!(&self.active, TabTarget::Detail(active_id) if active_id.as_ref() == id) {
                self.active = TabTarget::Catalog;
            }
            cx.emit(TabsChanged);
        }
    }
}

impl Default for TabsController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_controller_starts_with_only_catalog_tab_active() {
        let ctrl = TabsController::new();
        let snap = ctrl.snapshot();
        assert_eq!(snap.open_tabs, vec![TabTarget::Catalog]);
        assert!(matches!(snap.active, TabTarget::Catalog));
    }
}
