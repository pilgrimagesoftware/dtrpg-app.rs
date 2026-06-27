//! Library UI state and interaction controller.

use std::sync::Arc;
use gpui::{Context, Entity};
use crate::controllers::activity::ActivityController;
use crate::data::theme::LibriTheme;
use crate::data::enums::*;
use crate::data::theme::*;
use crate::util::filter::*;
use crate::data::selection::Selection;
use crate::data::library::*;
use crate::util::sort::*;
use crate::util::publisher::*;
use crate::util::matching::*;
use crate::data::events::*;
use crate::services::{LibraryService, LibraryServiceError};
use crate::view_models::library::{LibraryPaneState, LibraryViewModel};

// ── LibraryController ─────────────────────────────────────────────────────────

/// Snapshot of all data needed by the root view for a single render pass.
pub struct LibrarySnapshot {
    pub filter: SidebarFilter,
    pub counts: SectionCounts,
    pub publishers: Vec<PublisherEntry>,
    pub total_count: usize,
    pub total_mb: f64,
    pub matched_count: usize,
    pub search_query: String,
    pub sort: SortMethod,
    pub grouped: bool,
    pub presentation: CatalogPresentation,
    pub selected_item: Option<LibraryItem>,
    pub items: Vec<LibraryItem>,
}

/// Owns all mutable state for the library view.
pub struct LibraryController {
    /// View model that owns the service and pane state.
    vm: LibraryViewModel,
    /// Keeps the `ActivityController` entity alive so the weak reference in
    /// background task closures remains valid for the lifetime of this controller.
    #[allow(dead_code)]
    activity: Entity<ActivityController>,
    /// Full catalog — never filtered.
    catalog: Vec<LibraryItem>,
    /// Active sidebar filter.
    pub filter: SidebarFilter,
    /// Text search query.
    pub search_query: String,
    /// Current sort method.
    pub sort: SortMethod,
    /// Whether the catalog is grouped by publisher.
    pub grouped: bool,
    /// Active catalog presentation mode.
    pub presentation: CatalogPresentation,
    /// The currently selected item id (for the detail panel).
    pub selection: Selection,
    /// Smart section counts derived from the full catalog.
    pub section_counts: SectionCounts,
    /// Publisher list derived from the full catalog (count desc, name asc).
    pub publishers: Vec<PublisherEntry>,
}

impl LibraryController {
    /// Creates a controller and immediately schedules catalog loading on a background thread.
    ///
    /// The controller starts in the `Loading` pane state with an empty catalog. When the
    /// background fetch completes, [`apply_load_result`] is called and [`LibraryChanged`] emitted.
    ///
    /// # Panics
    ///
    /// Does not panic; service errors are reflected in [`pane_state`].
    pub fn new(service: Box<dyn LibraryService>, activity: Entity<ActivityController>, cx: &mut Context<Self>) -> Self {
        let vm = LibraryViewModel::new(service);
        let service_arc = vm.service_arc();
        let weak_activity = activity.downgrade();

        // Load catalog off the main thread so the UI remains responsive during the
        // potentially multi-page HTTP fetch.
        cx.spawn(async move |this, async_cx| {
            let activity_id = weak_activity
                .update(async_cx, |a, cx| a.start("Loading catalog\u{2026}", cx))
                .unwrap_or(0);

            let result = async_cx
                .background_executor()
                .spawn(async move { service_arc.list_items() })
                .await;

            match &result {
                Ok(_) => {
                    weak_activity.update(async_cx, |a, cx| a.complete(activity_id, cx)).ok();
                }
                Err(e) => {
                    let detail = e.panel_detail();
                    tracing::error!(error = %e, backtrace = %app_backtrace(), "catalog load failed");
                    weak_activity.update(async_cx, |a, cx| a.error(activity_id, detail, cx)).ok();
                }
            }

            this.update(async_cx, |ctrl, cx| ctrl.apply_load_result(result, cx)).ok();
        })
        .detach();

        Self {
            vm,
            activity,
            catalog: Vec::new(),
            filter: SidebarFilter::default(),
            search_query: String::new(),
            sort: SortMethod::default(),
            grouped: false,
            presentation: CatalogPresentation::default(),
            selection: Selection::default(),
            section_counts: SectionCounts::default(),
            publishers: Vec::new(),
        }
    }

    /// Applies a completed load result from the background task.
    fn apply_load_result(
        &mut self,
        result: Result<Vec<LibraryItem>, LibraryServiceError>,
        cx: &mut Context<Self>,
    ) {
        self.vm.apply_list_result(result);
        self.catalog = self.vm.items().to_vec();
        self.section_counts = section_counts(&self.catalog);
        self.publishers = publisher_entries(&self.catalog);
        cx.emit(LibraryChanged);
    }

    /// Returns the current pane state from the service layer.
    pub fn pane_state(&self) -> &LibraryPaneState {
        self.vm.pane_state()
    }

    /// Reloads catalog from the service and resets selection.
    pub fn reload(&mut self, cx: &mut Context<Self>) {
        self.vm.refresh();
        self.catalog = self.vm.items().to_vec();
        self.section_counts = section_counts(&self.catalog);
        self.publishers = publisher_entries(&self.catalog);
        self.selection = Selection::default();
        cx.emit(LibraryChanged);
    }

    // ── Snapshot ──────────────────────────────────────────────────────────────

    /// Returns all data needed by the root view for one render pass.
    pub fn snapshot(&self) -> LibrarySnapshot {
        let items = self.visible_items();
        let matched_count = items.len();
        let selected_item = self.selected_item().cloned();
        LibrarySnapshot {
            filter: self.filter.clone(),
            counts: self.section_counts,
            publishers: self.publishers.clone(),
            total_count: self.section_counts.all,
            total_mb: self.total_size_mb(),
            matched_count,
            search_query: self.search_query.clone(),
            sort: self.sort,
            grouped: self.grouped,
            presentation: self.presentation,
            selected_item,
            items,
        }
    }

    // ── Filtered result set ───────────────────────────────────────────────────

    /// Returns the filtered, sorted result set for the current state.
    #[must_use]
    pub fn visible_items(&self) -> Vec<LibraryItem> {
        let mut items: Vec<LibraryItem> = self
            .catalog
            .iter()
            .filter(|i| {
                item_matches_filter(i, &self.filter)
                    && item_matches_query(i, &self.search_query)
            })
            .cloned()
            .collect();
        sort_items(&mut items, self.sort);
        items
    }

    // ── Sidebar filter mutations ──────────────────────────────────────────────

    /// Sets the active sidebar filter.
    pub fn set_filter(&mut self, filter: SidebarFilter, cx: &mut Context<Self>) {
        self.filter = filter;
        self.selection = Selection::None;
        cx.emit(LibraryChanged);
    }

    // ── Search mutations ──────────────────────────────────────────────────────

    /// Updates the text search query.
    pub fn set_search_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.search_query = query;
        cx.emit(LibraryChanged);
    }

    /// Clears the text search query.
    pub fn clear_search_query(&mut self, cx: &mut Context<Self>) {
        self.search_query.clear();
        cx.emit(LibraryChanged);
    }

    // ── Sort mutations ────────────────────────────────────────────────────────

    /// Sets the sort method.
    pub fn set_sort(&mut self, sort: SortMethod, cx: &mut Context<Self>) {
        self.sort = sort;
        cx.emit(LibraryChanged);
    }

    // ── Grouping / presentation mutations ────────────────────────────────────

    /// Toggles publisher grouping on or off.
    pub fn set_grouped(&mut self, grouped: bool, cx: &mut Context<Self>) {
        self.grouped = grouped;
        cx.emit(LibraryChanged);
    }

    /// Switches the catalog presentation mode.
    pub fn set_presentation(&mut self, mode: CatalogPresentation, cx: &mut Context<Self>) {
        self.presentation = mode;
        cx.emit(LibraryChanged);
    }

    // ── Selection mutations ───────────────────────────────────────────────────

    /// Selects an item by id (opens the detail panel).
    pub fn select_item(&mut self, id: Arc<str>, cx: &mut Context<Self>) {
        self.selection = Selection::Item(id);
        cx.emit(LibraryChanged);
    }

    /// Clears the selection (closes the detail panel).
    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selection = Selection::None;
        cx.emit(LibraryChanged);
    }

    // ── Download toggle ───────────────────────────────────────────────────────

    /// Toggles the download status of the item with the given id.
    pub fn toggle_download(&mut self, id: &str, cx: &mut Context<Self>) {
        use crate::data::enums::ItemStatus;
        if let Some(item) = self.catalog.iter_mut().find(|i| i.id.as_ref() == id) {
            item.status = match item.status {
                ItemStatus::Downloaded => ItemStatus::Cloud,
                ItemStatus::Cloud => ItemStatus::Downloaded,
            };
            self.section_counts = section_counts(&self.catalog);
        }
        cx.emit(LibraryChanged);
    }

    // ── Theme / density mutations (dispatched via callbacks) ──────────────────

    /// Applies a new theme key (updates the GPUI global).
    pub fn set_theme(&self, key: ThemeKey, cx: &mut Context<Self>) {
        let current = cx.global::<LibriTheme>();
        let new_theme = LibriTheme::new(key, current.density);
        cx.set_global(new_theme);
        cx.notify();
    }

    /// Applies a new density (updates the GPUI global).
    pub fn set_density(&self, density: Density, cx: &mut Context<Self>) {
        let current = cx.global::<LibriTheme>();
        let new_theme = LibriTheme::new(current.key, density);
        cx.set_global(new_theme);
        cx.notify();
    }

    // ── Helper accessors ──────────────────────────────────────────────────────

    /// Returns the selected `LibraryItem`, if any.
    #[must_use]
    pub fn selected_item(&self) -> Option<&LibraryItem> {
        match &self.selection {
            Selection::Item(id) => self.catalog.iter().find(|i| &i.id == id),
            Selection::None => None,
        }
    }

    /// Total file size of all items in the catalog, in MB.
    #[must_use]
    pub fn total_size_mb(&self) -> f64 {
        self.catalog.iter().map(|i| i.size_mb).sum()
    }
}

/// Captures a backtrace and returns only the frames from app crates (`dtrpg_*`).
///
/// Each retained frame is one symbol line followed by its `at file:line:col` line.
/// Returns a hint string when `RUST_BACKTRACE` is not set or no app frames are found.
fn app_backtrace() -> String {
    let bt = std::backtrace::Backtrace::capture();
    if bt.status() != std::backtrace::BacktraceStatus::Captured {
        return "<set RUST_BACKTRACE=1 to capture backtraces>".to_string();
    }
    let full = format!("{bt}");
    let mut out: Vec<&str> = Vec::new();
    let mut take_location = false;
    for line in full.lines() {
        if line.trim_start().starts_with("at ") {
            if take_location {
                out.push(line);
                take_location = false;
            }
        } else if line.contains("dtrpg_") {
            out.push(line);
            take_location = true;
        } else {
            take_location = false;
        }
    }
    if out.is_empty() {
        "<no app frames found in backtrace>".to_string()
    } else {
        out.join("\n")
    }
}



// //! Library UI state and interaction controller.

// use crate::app::shell::{AppCommand, AppShell, AppShellState, SessionPresentationState};
// use crate::services::LibraryItem;
// use crate::services::sdk::RustSdkLibraryService;
// use crate::view_models::library::{LibraryPaneState, LibraryViewModel};

// use crate::ui::library::model::library_data::{
//     FilterScope, LibraryViewMode, MatchPresentation, SortMethod, TreeNode, filter_presets,
//     grouped_items, item_matches, mode_is_grid, mode_label, next_sort, root_matches, sort_label,
//     sorted_flat_items,
// };

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum Selection {
//     Publisher(String),
//     ProductType(String),
//     Item(u64),
// }

// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub enum SortPopup {
//     Flat,
//     Outer,
//     Inner,
// }

// /// UI state for the compact DriveThruRPG account menu.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct AccountMenuState {
//     /// User-facing account label shown in the account button/menu.
//     pub display_name: String,
//     /// Human-readable connection or token status.
//     pub connection_status: String,
//     /// Whether an access token is currently configured.
//     pub token_configured: bool,
//     /// Whether the compact account menu is visible.
//     pub menu_open: bool,
// }

// impl AccountMenuState {
//     fn signed_out() -> Self {
//         Self {
//             display_name: "DriveThruRPG account".to_string(),
//             connection_status: "Access token required".to_string(),
//             token_configured: std::env::var("DTRPG_ACCESS_TOKEN").is_ok(),
//             menu_open: false,
//         }
//     }
// }

// /// UI state for low-profile library sync/update reporting.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct SyncStatus {
//     /// Whether a sync or refresh operation is currently active.
//     pub active: bool,
//     /// Human-readable progress summary.
//     pub progress_label: String,
//     /// Human-readable network latency summary.
//     pub latency_label: String,
//     /// Human-readable last-update summary.
//     pub last_update_label: String,
// }

// impl SyncStatus {
//     fn idle() -> Self {
//         Self {
//             active: false,
//             progress_label: "Idle".to_string(),
//             latency_label: "Latency unavailable".to_string(),
//             last_update_label: "Not synced this session".to_string(),
//         }
//     }
// }

// pub struct LibraryController {
//     pub shell: AppShell,
//     pub view_mode: LibraryViewMode,
//     pub filter_scope: FilterScope,
//     pub match_presentation: MatchPresentation,
//     pub flat_sort: SortMethod,
//     pub outer_sort: SortMethod,
//     pub inner_sort: SortMethod,
//     pub filter_query: String,
//     pub search_editing: bool,
//     pub controls_disclosed: bool,
//     pub open_sort_popup: Option<SortPopup>,
//     pub selection: Option<Selection>,
//     pub account: AccountMenuState,
//     pub sync_status: SyncStatus,
// }

// impl LibraryController {
//     pub fn new() -> Self {
//         let service = RustSdkLibraryService::from_environment();
//         let library = LibraryViewModel::new(Box::new(service));

//         let mut shell = AppShell::new(
//             AppShellState {
//                 session: SessionPresentationState::Restoring,
//                 library: LibraryPaneState::Loading,
//                 selected_item_id: None,
//                 status_message: "Loading your library…".to_string(),
//             },
//             library,
//         );

//         shell.dispatch(AppCommand::LoadLibrary);

//         let selection = shell.first_item_id().map(Selection::Item);
//         if let Some(Selection::Item(first)) = selection {
//             shell.dispatch(AppCommand::SelectLibraryItem(first));
//         }

//         Self {
//             shell,
//             view_mode: LibraryViewMode::TreeByPublisher,
//             filter_scope: FilterScope::ChildOnly,
//             match_presentation: MatchPresentation::HideNonMatching,
//             flat_sort: SortMethod::AtoZ,
//             outer_sort: SortMethod::AtoZ,
//             inner_sort: SortMethod::AtoZ,
//             filter_query: String::new(),
//             search_editing: false,
//             open_sort_popup: None,
//             selection,
//             controls_disclosed: true,
//             account: AccountMenuState::signed_out(),
//             sync_status: SyncStatus::idle(),
//         }
//     }

//     pub fn cycle_view_mode(&mut self) {
//         self.view_mode = match self.view_mode {
//             LibraryViewMode::FlatList => LibraryViewMode::TreeByPublisher,
//             LibraryViewMode::TreeByPublisher => LibraryViewMode::TreeByProductType,
//             LibraryViewMode::TreeByProductType => LibraryViewMode::GridByPublisher,
//             LibraryViewMode::GridByPublisher => LibraryViewMode::GridByProductType,
//             LibraryViewMode::GridByProductType => LibraryViewMode::FlatList,
//         };
//         self.selection = None;
//         self.shell.dispatch(AppCommand::ClearSelection);
//     }

//     pub fn set_view_mode(&mut self, mode: LibraryViewMode) {
//         if self.view_mode != mode {
//             self.view_mode = mode;
//             self.selection = None;
//             self.shell.dispatch(AppCommand::ClearSelection);
//         }
//     }

//     pub fn cycle_filter_scope(&mut self) {
//         self.filter_scope = match self.filter_scope {
//             FilterScope::ChildOnly => FilterScope::RootAndChild,
//             FilterScope::RootAndChild => FilterScope::RootOnly,
//             FilterScope::RootOnly => FilterScope::ChildOnly,
//         };
//     }

//     pub fn set_filter_scope(&mut self, scope: FilterScope) {
//         self.filter_scope = scope;
//     }

//     pub fn set_match_presentation(&mut self, mode: MatchPresentation) {
//         self.match_presentation = mode;
//     }

//     pub fn toggle_match_presentation(&mut self) {
//         self.match_presentation = match self.match_presentation {
//             MatchPresentation::HideNonMatching => MatchPresentation::HighlightMatching,
//             MatchPresentation::HighlightMatching => MatchPresentation::HideNonMatching,
//         };
//     }

//     pub fn cycle_flat_sort(&mut self) {
//         self.flat_sort = next_sort(self.flat_sort);
//     }

//     pub fn set_flat_sort(&mut self, sort: SortMethod) {
//         self.flat_sort = sort;
//         self.open_sort_popup = None;
//     }

//     pub fn cycle_outer_sort(&mut self) {
//         self.outer_sort = next_sort(self.outer_sort);
//     }

//     pub fn set_outer_sort(&mut self, sort: SortMethod) {
//         self.outer_sort = sort;
//         self.open_sort_popup = None;
//     }

//     pub fn cycle_inner_sort(&mut self) {
//         self.inner_sort = next_sort(self.inner_sort);
//     }

//     pub fn set_inner_sort(&mut self, sort: SortMethod) {
//         self.inner_sort = sort;
//         self.open_sort_popup = None;
//     }

//     pub fn toggle_sort_popup(&mut self, popup: SortPopup) {
//         self.open_sort_popup = match self.open_sort_popup {
//             Some(current) if current == popup => None,
//             _ => Some(popup),
//         };
//     }

//     pub fn close_sort_popup(&mut self) {
//         self.open_sort_popup = None;
//     }

//     pub fn toggle_controls_disclosure(&mut self) {
//         self.controls_disclosed = !self.controls_disclosed;
//     }

//     pub fn toggle_account_menu(&mut self) {
//         self.account.menu_open = !self.account.menu_open;
//     }

//     pub fn mark_token_set_action(&mut self) {
//         self.account.token_configured = true;
//         self.account.connection_status = "Access token action selected".to_string();
//         self.account.menu_open = false;
//     }

//     pub fn mark_token_reset_action(&mut self) {
//         self.account.token_configured = false;
//         self.account.connection_status = "Access token reset requested".to_string();
//         self.account.menu_open = false;
//     }

//     pub fn open_settings_action(&mut self) {
//         self.account.connection_status = "Settings action selected".to_string();
//         self.account.menu_open = false;
//     }

//     pub fn cycle_filter_query(&mut self) {
//         let presets = filter_presets();
//         let current = presets
//             .iter()
//             .position(|preset| *preset == self.filter_query)
//             .unwrap_or(0);
//         let next = (current + 1) % presets.len();
//         self.filter_query = presets[next].to_string();
//     }

//     pub fn set_filter_query(&mut self, query: impl Into<String>) {
//         self.filter_query = query.into();
//     }

//     pub fn begin_search_editing(&mut self) {
//         self.search_editing = true;
//     }

//     pub fn end_search_editing(&mut self) {
//         self.search_editing = false;
//     }

//     pub fn append_query_char(&mut self, ch: char) {
//         if !ch.is_control() {
//             self.filter_query.push(ch);
//         }
//     }

//     pub fn backspace_query(&mut self) {
//         self.filter_query.pop();
//     }

//     pub fn clear_filter_query(&mut self) {
//         self.filter_query.clear();
//     }

//     pub fn handle_global_key(&mut self, key: &str, modifiers: &gpui::Modifiers) {
//         if modifiers.secondary() && key.eq_ignore_ascii_case("f") {
//             self.begin_search_editing();
//             return;
//         }

//         if modifiers.secondary() && key.eq_ignore_ascii_case("l") {
//             self.clear_filter_query();
//             self.begin_search_editing();
//             return;
//         }

//         if key == "/" {
//             self.begin_search_editing();
//             return;
//         }

//         if self.search_editing {
//             if key == "escape" {
//                 self.end_search_editing();
//             } else if key == "backspace" {
//                 self.backspace_query();
//             } else if key.chars().count() == 1
//                 && !modifiers.control
//                 && !modifiers.alt
//                 && !modifiers.platform
//                 && !modifiers.function
//             {
//                 if let Some(ch) = key.chars().next() {
//                     self.append_query_char(ch);
//                 }
//             }
//         }
//     }

//     pub fn refresh(&mut self) {
//         self.sync_status = SyncStatus {
//             active: true,
//             progress_label: "Refreshing library metadata".to_string(),
//             latency_label: "Last request pending".to_string(),
//             last_update_label: "Refresh in progress".to_string(),
//         };

//         self.shell.dispatch(AppCommand::RefreshLibrary);

//         if let Some(Selection::Item(item_id)) = self.selection {
//             self.shell.dispatch(AppCommand::SelectLibraryItem(item_id));
//         }

//         self.sync_status = SyncStatus {
//             active: false,
//             progress_label: "Library metadata current".to_string(),
//             latency_label: "Last request completed".to_string(),
//             last_update_label: "Updated this session".to_string(),
//         };
//     }

//     pub fn set_item_selection(&mut self, item_id: u64) {
//         self.selection = Some(Selection::Item(item_id));
//         self.shell.dispatch(AppCommand::SelectLibraryItem(item_id));
//     }

//     pub fn set_publisher_selection(&mut self, publisher: String) {
//         self.selection = Some(Selection::Publisher(publisher));
//         self.shell.dispatch(AppCommand::ClearSelection);
//     }

//     pub fn set_product_type_selection(&mut self, product_type: String) {
//         self.selection = Some(Selection::ProductType(product_type));
//         self.shell.dispatch(AppCommand::ClearSelection);
//     }

//     pub fn mode_label(&self) -> &'static str {
//         mode_label(self.view_mode)
//     }

//     pub fn flat_sort_label(&self) -> &'static str {
//         sort_label(self.flat_sort)
//     }

//     pub fn outer_sort_label(&self) -> &'static str {
//         sort_label(self.outer_sort)
//     }

//     pub fn inner_sort_label(&self) -> &'static str {
//         sort_label(self.inner_sort)
//     }

//     pub fn controls_summary(&self) -> String {
//         format!(
//             "{} | query: {} | {} | sections: {}",
//             self.mode_label(),
//             self.active_query_label(),
//             self.active_sort_summary(),
//             self.section_count()
//         )
//     }

//     pub fn active_sort_summary(&self) -> String {
//         match self.view_mode {
//             LibraryViewMode::FlatList => format!("sort: {}", self.flat_sort_label()),
//             _ => format!(
//                 "outer: {}, inner: {}",
//                 self.outer_sort_label(),
//                 self.inner_sort_label()
//             ),
//         }
//     }

//     pub fn account_summary(&self) -> String {
//         let token_status = if self.account.token_configured {
//             "token set"
//         } else {
//             "token missing"
//         };

//         format!("{} ({token_status})", self.account.display_name)
//     }

//     pub fn sync_status_summary(&self) -> String {
//         if self.sync_status.active {
//             format!("Syncing: {}", self.sync_status.progress_label)
//         } else {
//             format!("Sync: {}", self.sync_status.progress_label)
//         }
//     }

//     pub fn sync_status_detail(&self) -> String {
//         format!(
//             "{} | {} | {}",
//             self.sync_status.progress_label,
//             self.sync_status.latency_label,
//             self.sync_status.last_update_label
//         )
//     }

//     pub fn view_summary(&self) -> String {
//         format!(
//             "{} total | {} matched | {} sections",
//             self.shell.items().len(),
//             self.filtered_item_count(),
//             self.section_count()
//         )
//     }

//     pub fn match_presentation_label(&self) -> &'static str {
//         match self.match_presentation {
//             MatchPresentation::HideNonMatching => "Search mode: hide non-matching",
//             MatchPresentation::HighlightMatching => "Search mode: highlight matches",
//         }
//     }

//     pub fn active_query_label(&self) -> String {
//         if self.filter_query.is_empty() {
//             "(none)".to_string()
//         } else {
//             self.filter_query.clone()
//         }
//     }

//     pub fn flat_items(&self) -> Vec<LibraryItem> {
//         let mut items = sorted_flat_items(self.shell.items(), self.flat_sort);

//         if matches!(self.match_presentation, MatchPresentation::HideNonMatching)
//             && !self.filter_query.is_empty()
//         {
//             items.retain(|item| item_matches(item, &self.filter_query));
//         }

//         items
//     }

//     pub fn tree_items(&self) -> Vec<TreeNode> {
//         let mut nodes = grouped_items(
//             self.shell.items(),
//             self.view_mode,
//             self.outer_sort,
//             self.inner_sort,
//         );

//         if self.filter_query.is_empty() {
//             return nodes;
//         }

//         if matches!(
//             self.match_presentation,
//             MatchPresentation::HighlightMatching
//         ) {
//             return nodes;
//         }

//         let query = self.filter_query.clone();

//         nodes.retain_mut(|node| {
//             let root_hit = root_matches(&node.root_label, &query);

//             match self.filter_scope {
//                 FilterScope::ChildOnly => {
//                     node.children.retain(|item| item_matches(item, &query));
//                 }
//                 FilterScope::RootAndChild => {
//                     node.children
//                         .retain(|item| root_hit || item_matches(item, &query));
//                 }
//                 FilterScope::RootOnly => {
//                     if !root_hit {
//                         node.children.clear();
//                     }
//                 }
//             }

//             !node.children.is_empty()
//         });

//         nodes
//     }

//     pub fn grid_sections(&self) -> Vec<TreeNode> {
//         self.tree_items()
//     }

//     pub fn is_item_match(&self, item: &LibraryItem) -> bool {
//         item_matches(item, &self.filter_query)
//     }

//     pub fn is_root_match(&self, root_label: &str) -> bool {
//         root_matches(root_label, &self.filter_query)
//     }

//     pub fn filtered_item_count(&self) -> usize {
//         match self.view_mode {
//             LibraryViewMode::FlatList => self.flat_items().len(),
//             _ => self
//                 .tree_items()
//                 .into_iter()
//                 .map(|node| node.children.len())
//                 .sum(),
//         }
//     }

//     pub fn section_count(&self) -> usize {
//         match self.view_mode {
//             LibraryViewMode::FlatList => 0,
//             _ => self.tree_items().len(),
//         }
//     }

//     pub fn renders_grid(&self) -> bool {
//         mode_is_grid(self.view_mode)
//     }

//     pub fn detail_lines(&self) -> Vec<String> {
//         match &self.selection {
//             Some(Selection::Item(item_id)) => {
//                 if let Some(item) = self.shell.items().iter().find(|item| item.id == *item_id) {
//                     return vec![
//                         "Catalog item detail".to_string(),
//                         format!("Title: {}", item.title),
//                         format!("Publisher: {}", item.publisher),
//                         format!("Type: {}", item.product_type),
//                         format!("Added order: {}", item.added_order),
//                         format!("Updated order: {}", item.updated_order),
//                         format!("Summary: {}", item.summary),
//                     ];
//                 }

//                 vec!["Catalog item detail unavailable.".to_string()]
//             }
//             Some(Selection::Publisher(publisher)) => {
//                 let count = self
//                     .shell
//                     .items()
//                     .iter()
//                     .filter(|item| &item.publisher == publisher)
//                     .count();

//                 vec![
//                     "Publisher detail".to_string(),
//                     format!("Publisher: {}", publisher),
//                     format!("Items in library: {}", count),
//                     "Publisher metadata is derived from SDK library responses.".to_string(),
//                 ]
//             }
//             Some(Selection::ProductType(product_type)) => {
//                 let count = self
//                     .shell
//                     .items()
//                     .iter()
//                     .filter(|item| &item.product_type == product_type)
//                     .count();

//                 vec![
//                     "Product type detail".to_string(),
//                     format!("Type: {}", product_type),
//                     format!("Items in library: {}", count),
//                     "Suggested arrangement enabled: tree grouped by product type.".to_string(),
//                 ]
//             }
//             None => vec!["Select a publisher or catalog item to view details.".to_string()],
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::services::stub::{StubLibraryService, StubMode};

//     fn make_controller() -> LibraryController {
//         let library = LibraryViewModel::new(Box::new(StubLibraryService::new(StubMode::Seeded)));
//         let mut shell = AppShell::new(
//             AppShellState {
//                 session: SessionPresentationState::SignedIn,
//                 library: LibraryPaneState::Loading,
//                 selected_item_id: None,
//                 status_message: "Loading your library…".to_string(),
//             },
//             library,
//         );
//         shell.dispatch(AppCommand::LoadLibrary);

//         LibraryController {
//             shell,
//             view_mode: LibraryViewMode::TreeByPublisher,
//             filter_scope: FilterScope::ChildOnly,
//             match_presentation: MatchPresentation::HideNonMatching,
//             flat_sort: SortMethod::AtoZ,
//             outer_sort: SortMethod::AtoZ,
//             inner_sort: SortMethod::AtoZ,
//             filter_query: String::new(),
//             search_editing: false,
//             controls_disclosed: true,
//             open_sort_popup: None,
//             selection: None,
//             account: AccountMenuState::signed_out(),
//             sync_status: SyncStatus::idle(),
//         }
//     }

//     #[test]
//     fn controls_disclosure_preserves_browsing_summary() {
//         let mut controller = make_controller();
//         controller.set_filter_query("atlas");
//         controller.set_view_mode(LibraryViewMode::GridByPublisher);

//         let expanded_summary = controller.controls_summary();
//         controller.toggle_controls_disclosure();

//         assert!(!controller.controls_disclosed);
//         assert_eq!(controller.filter_query, "atlas");
//         assert_eq!(controller.controls_summary(), expanded_summary);
//         assert!(controller.controls_summary().contains("Grid by publisher"));
//     }

//     #[test]
//     fn grid_and_tree_presentations_share_filtered_result_state() {
//         let mut controller = make_controller();
//         controller.set_filter_query("atlas");

//         controller.set_view_mode(LibraryViewMode::TreeByPublisher);
//         let tree_count = controller.filtered_item_count();
//         let tree_sections = controller.section_count();

//         controller.set_view_mode(LibraryViewMode::GridByPublisher);

//         assert!(controller.renders_grid());
//         assert_eq!(controller.filtered_item_count(), tree_count);
//         assert_eq!(controller.section_count(), tree_sections);
//     }

//     #[test]
//     fn account_actions_do_not_store_raw_token_values() {
//         let mut controller = make_controller();

//         controller.toggle_account_menu();
//         assert!(controller.account.menu_open);

//         controller.mark_token_set_action();
//         assert!(controller.account.token_configured);
//         assert!(!controller.account.menu_open);
//         assert!(!controller.account_summary().contains("DTRPG_ACCESS_TOKEN"));

//         controller.mark_token_reset_action();
//         assert!(!controller.account.token_configured);
//         assert!(!controller.account_summary().contains("DTRPG_ACCESS_TOKEN"));
//     }

//     #[test]
//     fn refresh_updates_sync_status_without_changing_browsing_state() {
//         let mut controller = make_controller();
//         controller.set_filter_query("atlas");
//         controller.set_view_mode(LibraryViewMode::GridByProductType);
//         let summary = controller.controls_summary();

//         controller.refresh();

//         assert!(!controller.sync_status.active);
//         assert_eq!(controller.filter_query, "atlas");
//         assert_eq!(controller.view_mode, LibraryViewMode::GridByProductType);
//         assert_eq!(controller.controls_summary(), summary);
//         assert!(
//             controller
//                 .sync_status_summary()
//                 .contains("Library metadata")
//         );
//     }
// }
