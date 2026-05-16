//! Library UI state and interaction controller.

use crate::app::shell::{AppCommand, AppShell, AppShellState, SessionPresentationState};
use crate::services::LibraryItem;
use crate::services::sdk::RustSdkLibraryService;
use crate::view_models::library::{LibraryPaneState, LibraryViewModel};

use crate::ui::library::model::library_data::{
    FilterScope, LibraryViewMode, MatchPresentation, SortMethod, TreeNode, filter_presets,
    grouped_items, item_matches, mode_label, next_sort, root_matches, sort_label,
    sorted_flat_items,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selection {
    Publisher(String),
    ProductType(String),
    Item(u64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortPopup {
    Flat,
    Outer,
    Inner,
}

pub struct LibraryController {
    pub shell: AppShell,
    pub view_mode: LibraryViewMode,
    pub filter_scope: FilterScope,
    pub match_presentation: MatchPresentation,
    pub flat_sort: SortMethod,
    pub outer_sort: SortMethod,
    pub inner_sort: SortMethod,
    pub filter_query: String,
    pub search_editing: bool,
    pub open_sort_popup: Option<SortPopup>,
    pub selection: Option<Selection>,
}

impl LibraryController {
    pub fn new() -> Self {
        let service = RustSdkLibraryService::from_environment();
        let library = LibraryViewModel::new(Box::new(service));

        let mut shell = AppShell::new(
            AppShellState {
                session: SessionPresentationState::Restoring,
                library: LibraryPaneState::Loading,
                selected_item_id: None,
                status_message: "Loading your library…".to_string(),
            },
            library,
        );

        shell.dispatch(AppCommand::LoadLibrary);

        let selection = shell.first_item_id().map(Selection::Item);
        if let Some(Selection::Item(first)) = selection {
            shell.dispatch(AppCommand::SelectLibraryItem(first));
        }

        Self {
            shell,
            view_mode: LibraryViewMode::TreeByPublisher,
            filter_scope: FilterScope::ChildOnly,
            match_presentation: MatchPresentation::HideNonMatching,
            flat_sort: SortMethod::AtoZ,
            outer_sort: SortMethod::AtoZ,
            inner_sort: SortMethod::AtoZ,
            filter_query: String::new(),
            search_editing: false,
            open_sort_popup: None,
            selection,
        }
    }

    pub fn cycle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            LibraryViewMode::FlatList => LibraryViewMode::TreeByPublisher,
            LibraryViewMode::TreeByPublisher => LibraryViewMode::TreeByProductType,
            LibraryViewMode::TreeByProductType => LibraryViewMode::FlatList,
        };
        self.selection = None;
        self.shell.dispatch(AppCommand::ClearSelection);
    }

    pub fn set_view_mode(&mut self, mode: LibraryViewMode) {
        if self.view_mode != mode {
            self.view_mode = mode;
            self.selection = None;
            self.shell.dispatch(AppCommand::ClearSelection);
        }
    }

    pub fn cycle_filter_scope(&mut self) {
        self.filter_scope = match self.filter_scope {
            FilterScope::ChildOnly => FilterScope::RootAndChild,
            FilterScope::RootAndChild => FilterScope::RootOnly,
            FilterScope::RootOnly => FilterScope::ChildOnly,
        };
    }

    pub fn set_filter_scope(&mut self, scope: FilterScope) {
        self.filter_scope = scope;
    }

    pub fn set_match_presentation(&mut self, mode: MatchPresentation) {
        self.match_presentation = mode;
    }

    pub fn toggle_match_presentation(&mut self) {
        self.match_presentation = match self.match_presentation {
            MatchPresentation::HideNonMatching => MatchPresentation::HighlightMatching,
            MatchPresentation::HighlightMatching => MatchPresentation::HideNonMatching,
        };
    }

    pub fn cycle_flat_sort(&mut self) {
        self.flat_sort = next_sort(self.flat_sort);
    }

    pub fn set_flat_sort(&mut self, sort: SortMethod) {
        self.flat_sort = sort;
        self.open_sort_popup = None;
    }

    pub fn cycle_outer_sort(&mut self) {
        self.outer_sort = next_sort(self.outer_sort);
    }

    pub fn set_outer_sort(&mut self, sort: SortMethod) {
        self.outer_sort = sort;
        self.open_sort_popup = None;
    }

    pub fn cycle_inner_sort(&mut self) {
        self.inner_sort = next_sort(self.inner_sort);
    }

    pub fn set_inner_sort(&mut self, sort: SortMethod) {
        self.inner_sort = sort;
        self.open_sort_popup = None;
    }

    pub fn toggle_sort_popup(&mut self, popup: SortPopup) {
        self.open_sort_popup = match self.open_sort_popup {
            Some(current) if current == popup => None,
            _ => Some(popup),
        };
    }

    pub fn close_sort_popup(&mut self) {
        self.open_sort_popup = None;
    }

    pub fn cycle_filter_query(&mut self) {
        let presets = filter_presets();
        let current = presets
            .iter()
            .position(|preset| *preset == self.filter_query)
            .unwrap_or(0);
        let next = (current + 1) % presets.len();
        self.filter_query = presets[next].to_string();
    }

    pub fn set_filter_query(&mut self, query: impl Into<String>) {
        self.filter_query = query.into();
    }

    pub fn begin_search_editing(&mut self) {
        self.search_editing = true;
    }

    pub fn end_search_editing(&mut self) {
        self.search_editing = false;
    }

    pub fn append_query_char(&mut self, ch: char) {
        if !ch.is_control() {
            self.filter_query.push(ch);
        }
    }

    pub fn backspace_query(&mut self) {
        self.filter_query.pop();
    }

    pub fn clear_filter_query(&mut self) {
        self.filter_query.clear();
    }

    pub fn handle_global_key(&mut self, key: &str, modifiers: &gpui::Modifiers) {
        if modifiers.secondary() && key.eq_ignore_ascii_case("f") {
            self.begin_search_editing();
            return;
        }

        if modifiers.secondary() && key.eq_ignore_ascii_case("l") {
            self.clear_filter_query();
            self.begin_search_editing();
            return;
        }

        if key == "/" {
            self.begin_search_editing();
            return;
        }

        if self.search_editing {
            if key == "escape" {
                self.end_search_editing();
            } else if key == "backspace" {
                self.backspace_query();
            } else if key.chars().count() == 1
                && !modifiers.control
                && !modifiers.alt
                && !modifiers.platform
                && !modifiers.function
            {
                if let Some(ch) = key.chars().next() {
                    self.append_query_char(ch);
                }
            }
        }
    }

    pub fn refresh(&mut self) {
        self.shell.dispatch(AppCommand::RefreshLibrary);

        if let Some(Selection::Item(item_id)) = self.selection {
            self.shell.dispatch(AppCommand::SelectLibraryItem(item_id));
        }
    }

    pub fn set_item_selection(&mut self, item_id: u64) {
        self.selection = Some(Selection::Item(item_id));
        self.shell.dispatch(AppCommand::SelectLibraryItem(item_id));
    }

    pub fn set_publisher_selection(&mut self, publisher: String) {
        self.selection = Some(Selection::Publisher(publisher));
        self.shell.dispatch(AppCommand::ClearSelection);
    }

    pub fn set_product_type_selection(&mut self, product_type: String) {
        self.selection = Some(Selection::ProductType(product_type));
        self.shell.dispatch(AppCommand::ClearSelection);
    }

    pub fn mode_label(&self) -> &'static str {
        mode_label(self.view_mode)
    }

    pub fn flat_sort_label(&self) -> &'static str {
        sort_label(self.flat_sort)
    }

    pub fn outer_sort_label(&self) -> &'static str {
        sort_label(self.outer_sort)
    }

    pub fn inner_sort_label(&self) -> &'static str {
        sort_label(self.inner_sort)
    }

    pub fn match_presentation_label(&self) -> &'static str {
        match self.match_presentation {
            MatchPresentation::HideNonMatching => "Search mode: hide non-matching",
            MatchPresentation::HighlightMatching => "Search mode: highlight matches",
        }
    }

    pub fn active_query_label(&self) -> String {
        if self.filter_query.is_empty() {
            "(none)".to_string()
        } else {
            self.filter_query.clone()
        }
    }

    pub fn flat_items(&self) -> Vec<LibraryItem> {
        let mut items = sorted_flat_items(self.shell.items(), self.flat_sort);

        if matches!(self.match_presentation, MatchPresentation::HideNonMatching)
            && !self.filter_query.is_empty()
        {
            items.retain(|item| item_matches(item, &self.filter_query));
        }

        items
    }

    pub fn tree_items(&self) -> Vec<TreeNode> {
        let mut nodes = grouped_items(
            self.shell.items(),
            self.view_mode,
            self.outer_sort,
            self.inner_sort,
        );

        if self.filter_query.is_empty() {
            return nodes;
        }

        if matches!(
            self.match_presentation,
            MatchPresentation::HighlightMatching
        ) {
            return nodes;
        }

        let query = self.filter_query.clone();

        nodes.retain_mut(|node| {
            let root_hit = root_matches(&node.root_label, &query);

            match self.filter_scope {
                FilterScope::ChildOnly => {
                    node.children.retain(|item| item_matches(item, &query));
                }
                FilterScope::RootAndChild => {
                    node.children
                        .retain(|item| root_hit || item_matches(item, &query));
                }
                FilterScope::RootOnly => {
                    if !root_hit {
                        node.children.clear();
                    }
                }
            }

            !node.children.is_empty()
        });

        nodes
    }

    pub fn is_item_match(&self, item: &LibraryItem) -> bool {
        item_matches(item, &self.filter_query)
    }

    pub fn is_root_match(&self, root_label: &str) -> bool {
        root_matches(root_label, &self.filter_query)
    }

    pub fn filtered_item_count(&self) -> usize {
        match self.view_mode {
            LibraryViewMode::FlatList => self.flat_items().len(),
            _ => self
                .tree_items()
                .into_iter()
                .map(|node| node.children.len())
                .sum(),
        }
    }

    pub fn detail_lines(&self) -> Vec<String> {
        match &self.selection {
            Some(Selection::Item(item_id)) => {
                if let Some(item) = self.shell.items().iter().find(|item| item.id == *item_id) {
                    return vec![
                        "Catalog item detail".to_string(),
                        format!("Title: {}", item.title),
                        format!("Publisher: {}", item.publisher),
                        format!("Type: {}", item.product_type),
                        format!("Added order: {}", item.added_order),
                        format!("Updated order: {}", item.updated_order),
                        format!("Summary: {}", item.summary),
                    ];
                }

                vec!["Catalog item detail unavailable.".to_string()]
            }
            Some(Selection::Publisher(publisher)) => {
                let count = self
                    .shell
                    .items()
                    .iter()
                    .filter(|item| &item.publisher == publisher)
                    .count();

                vec![
                    "Publisher detail".to_string(),
                    format!("Publisher: {}", publisher),
                    format!("Items in library: {}", count),
                    "Publisher metadata is derived from SDK library responses.".to_string(),
                ]
            }
            Some(Selection::ProductType(product_type)) => {
                let count = self
                    .shell
                    .items()
                    .iter()
                    .filter(|item| &item.product_type == product_type)
                    .count();

                vec![
                    "Product type detail".to_string(),
                    format!("Type: {}", product_type),
                    format!("Items in library: {}", count),
                    "Suggested arrangement enabled: tree grouped by product type.".to_string(),
                ]
            }
            None => vec!["Select a publisher or catalog item to view details.".to_string()],
        }
    }
}
