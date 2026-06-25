//! Libri library controller: owns all mutable UI state for the library feature.

use std::sync::Arc;

use gpui::{App, Context, EventEmitter};

use crate::ui::library::cover::CoverCache;
use crate::data::{
    data::{
        item_matches_filter, item_matches_query, publisher_entries, section_counts, sort_items,
        CatalogPresentation, LibraryItem, PublisherEntry, SectionCounts, SidebarFilter, SortMethod,
        stub_catalog,
    },
    theme::{Density, LibriTheme, ThemeKey},
};

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

// ── LibraryChanged event ──────────────────────────────────────────────────────

/// Emitted when any state changes that should trigger a re-render.
pub struct LibraryChanged;

impl EventEmitter<LibraryChanged> for LibraryController {}

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
    /// Creates a controller loaded with the stub catalog.
    pub fn new() -> Self {
        let catalog = stub_catalog();
        let section_counts = section_counts(&catalog);
        let publishers = publisher_entries(&catalog);
        Self {
            catalog,
            filter: SidebarFilter::default(),
            search_query: String::new(),
            sort: SortMethod::default(),
            grouped: false,
            presentation: CatalogPresentation::default(),
            selection: Selection::default(),
            section_counts,
            publishers,
        }
    }

    // ── Snapshot ──────────────────────────────────────────────────────────────

    /// Returns all data needed by the root view for one render pass.
    pub fn snapshot(&self) -> (
        SidebarFilter,
        SectionCounts,
        Vec<PublisherEntry>,
        usize,
        f64,
        usize,
        String,
        SortMethod,
        bool,
        CatalogPresentation,
        Option<LibraryItem>,
        Vec<LibraryItem>,
    ) {
        let items = self.visible_items();
        let matched_count = items.len();
        let selected_item = self.selected_item().cloned();
        (
            self.filter.clone(),
            self.section_counts,
            self.publishers.clone(),
            self.section_counts.all,
            self.total_size_mb(),
            matched_count,
            self.search_query.clone(),
            self.sort,
            self.grouped,
            self.presentation,
            selected_item,
            items,
        )
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
        use crate::data::data::ItemStatus;
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

impl Default for LibraryController {
    fn default() -> Self {
        Self::new()
    }
}

// ── GPUI global initializer ───────────────────────────────────────────────────

/// Registers `LibriTheme` and `CoverCache` as GPUI app-level globals.
pub fn init_globals(cx: &mut App) {
    cx.set_global(LibriTheme::default_theme());
    cx.set_global(CoverCache::new());
}
