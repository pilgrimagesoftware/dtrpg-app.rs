//! Catalog view: list, thumbs, and grid layouts with grouping and empty state.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    AnyElement, App, Context, Entity, Image, IntoElement, ObjectFit, ParentElement, Render, Styled,
    StyledImage, UniformListScrollHandle, Window, div, img, px, uniform_list,
};
use gpui_component::badge::Badge;
use gpui_component::pagination::Pagination;
use gpui_component::scroll::ScrollableElement;
use gpui_component::spinner::Spinner;
use gpui_component::table::{Column, ColumnSort, DataTable, TableDelegate, TableEvent, TableState};
use gpui_component::{Disableable, Sizable, Size};

use gpui_component::button::{Button, ButtonVariants};
use gpui_component::menu::{ContextMenuExt, DropdownMenu, PopupMenu, PopupMenuItem};

use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::data::enums::CatalogPresentation;
use crate::data::enums::*;
use crate::data::events::LibraryChanged;
use crate::data::library::{LibraryItem, thumbnail_cooldown_elapsed};
use crate::data::theme::{ColorTokens, DensityConstants, LibriTheme};
use crate::ui::library::cover::{CoverCache, render_generative_cover};
use crate::util::publisher::{PublisherGroup, group_by_publisher};
use crate::util::reveal::reveal_in_file_manager;
use crate::util::sort::{SortDirection, SortMethod};
use rust_i18n::t;

#[derive(Clone, Copy)]
enum EmptyReason {
    LibraryEmpty,
    NoMatches,
}

// ── Shared column definitions ─────────────────────────────────────────────────

/// Returns the column definitions used for the list view.
///
/// Used by both the `DataTable` (ungrouped) and the grouped-list header/rows
/// to ensure column widths are always in sync between headers and cells.
fn list_columns() -> Vec<Column> {
    vec![
        Column::new("title", t!("catalog.col_title"))
            .width(300.)
            .min_width(150.)
            .resizable(true),
        Column::new("publisher", t!("catalog.col_publisher"))
            .width(130.)
            .resizable(true),
        Column::new("system", t!("catalog.col_system"))
            .width(110.)
            .resizable(true),
        Column::new("pages", t!("catalog.col_pages"))
            .width(60.)
            .resizable(true),
        Column::new("size", t!("catalog.col_size"))
            .width(60.)
            .resizable(true),
        Column::new("added", t!("catalog.col_added"))
            .width(80.)
            .resizable(true),
        Column::new("status", "")
            .width(24.)
            .resizable(false)
            .selectable(false),
        Column::new("open", "")
            .width(28.)
            .resizable(false)
            .selectable(false),
        Column::new("ctx", "")
            .width(28.)
            .resizable(false)
            .selectable(false),
    ]
}

/// Returns a 2–3 character badge abbreviation for an item kind string.
fn kind_badge(kind: &str) -> &'static str {
    if kind.contains("Core") {
        "CR"
    } else if kind.contains("Supplement") {
        "SUP"
    } else if kind.contains("Adventure") {
        "ADV"
    } else if kind.contains("Map") {
        "MAP"
    } else if kind.contains("Token") {
        "TOK"
    } else if kind.contains("Bundle") || kind.contains("PDF") {
        "PDF"
    } else {
        "OTH"
    }
}

// ── CatalogListDelegate ───────────────────────────────────────────────────────

/// `TableDelegate` for the ungrouped list view. Backed by `LibraryController`.
struct CatalogListDelegate {
    controller: Entity<LibraryController>,
    storage_root: PathBuf,
    columns: Vec<Column>,
    /// User-adjusted column widths captured from `TableEvent::ColumnWidthsChanged`.
    /// `None` means use the static default from `list_columns()`.
    user_widths: Vec<Option<gpui::Pixels>>,
}

impl TableDelegate for CatalogListDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, cx: &App) -> usize {
        self.controller.read(cx).visible_items_count()
    }

    fn column(&self, col_ix: usize, cx: &App) -> Column {
        let snap = self.controller.read(cx).snapshot();
        let active_col = match snap.sort {
            SortMethod::Title => Some(0usize),
            SortMethod::Publisher => Some(1),
            SortMethod::PageCount => Some(3),
            SortMethod::DateAdded => Some(5),
            SortMethod::Custom {
                col_key: "publisher",
            } => Some(1),
            SortMethod::Custom { col_key: "system" } => Some(2),
            SortMethod::Custom { col_key: "pages" } => Some(3),
            SortMethod::Custom { col_key: "size" } => Some(4),
            SortMethod::Custom { col_key: "added" } => Some(5),
            SortMethod::Custom { .. } => None,
        };
        let mut col = self.columns[col_ix].clone();
        if let Some(Some(w)) = self.user_widths.get(col_ix) {
            col = col.width(w.as_f32());
        }
        if active_col == Some(col_ix) {
            let sort = match snap.sort_direction {
                SortDirection::Ascending => ColumnSort::Ascending,
                SortDirection::Descending => ColumnSort::Descending,
            };
            col.sort(sort)
        } else if col_ix < 6 {
            col.sort(ColumnSort::Default)
        } else {
            col
        }
    }

    fn perform_sort(
        &mut self,
        col_ix: usize,
        sort: ColumnSort,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) {
        if col_ix >= 6 {
            return;
        }
        let method = match col_ix {
            0 => SortMethod::Title,
            1 => SortMethod::Custom {
                col_key: "publisher",
            },
            2 => SortMethod::Custom { col_key: "system" },
            3 => SortMethod::Custom { col_key: "pages" },
            4 => SortMethod::Custom { col_key: "size" },
            5 => SortMethod::Custom { col_key: "added" },
            _ => return,
        };
        let (method, direction) = match sort {
            ColumnSort::Ascending => (method, SortDirection::Ascending),
            ColumnSort::Descending => (method, SortDirection::Descending),
            ColumnSort::Default => (SortMethod::Title, SortDirection::Ascending),
        };
        self.controller.update(cx, |ctrl, cx| {
            ctrl.set_sort(method, cx);
            ctrl.set_sort_direction(direction, cx);
        });
    }

    fn context_menu(
        &mut self,
        row_ix: usize,
        menu: PopupMenu,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> PopupMenu {
        let items = self
            .controller
            .read(cx)
            .visible_items_slice(row_ix..row_ix + 1);
        let Some(item) = items.into_iter().next() else {
            return menu;
        };
        let id = Arc::clone(&item.id);
        let status = item.status;
        let entity = self.controller.clone();
        match status {
            ItemStatus::Downloaded => {
                let item_path = self.storage_root.join("items").join(&*id);
                let entity_remove = entity.clone();
                let remove_id = Arc::clone(&id);
                let item_path_open = item_path.clone();
                let item_path_reveal = item_path.clone();
                menu.item(
                    PopupMenuItem::new(t!("catalog.action_open")).on_click(move |_, _, _| {
                        use crate::util::item_opener::{ItemOpener, OpenError};

                        if !item_path_open.exists() {
                            tracing::warn!(
                                path = %item_path_open.display(),
                                "open: file not found"
                            );
                            return;
                        }
                        if let Err(e) = ItemOpener::open(&item_path_open) {
                            match e {
                                OpenError::FileNotFound(path) => {
                                    tracing::warn!("open: file not found: {path}");
                                }
                                OpenError::NoDefaultApp => {
                                    tracing::warn!("open: no default application configured");
                                }
                                OpenError::OsFailed(msg) => {
                                    tracing::warn!("open: OS failed: {msg}");
                                }
                                OpenError::MultipleFilesRequireSelection => {
                                    tracing::warn!("open: multiple files require selection");
                                }
                            }
                        }
                    }),
                )
                .item(
                    PopupMenuItem::new(platform_reveal_label()).on_click(move |_, _, _| {
                        if !item_path_reveal.exists() {
                            tracing::warn!(
                                path = %item_path_reveal.display(),
                                "reveal: file not found"
                            );
                            return;
                        }
                        if let Err(e) = reveal_in_file_manager(&item_path_reveal) {
                            tracing::warn!("reveal_in_file_manager failed: {e}");
                        }
                    }),
                )
                .item(
                    PopupMenuItem::new(t!("catalog.action_remove_download")).on_click(
                        move |_, _, cx| {
                            entity_remove
                                .update(cx, |ctrl, cx| ctrl.toggle_download(&remove_id, cx));
                        },
                    ),
                )
            }
            ItemStatus::Cloud => menu.item(
                PopupMenuItem::new(t!("catalog.action_download")).on_click(move |_, _, cx| {
                    entity.update(cx, |ctrl, cx| ctrl.toggle_download(&id, cx));
                }),
            ),
        }
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let items = self
            .controller
            .read(cx)
            .visible_items_slice(row_ix..row_ix + 1);
        let Some(item) = items.into_iter().next() else {
            return div().into_any_element();
        };
        let colors = cx.global::<LibriTheme>().colors.clone();

        match col_ix {
            0 => div()
                .h_full()
                .flex()
                .items_center()
                .gap(px(6.0))
                .min_w_0()
                .child(
                    div()
                        .text_sm()
                        .text_color(colors.text_primary)
                        .truncate()
                        .child(item.title.to_string()),
                )
                .child(
                    div()
                        .flex_none()
                        .text_xs()
                        .text_color(colors.text_tertiary)
                        .px(px(4.0))
                        .py(px(1.0))
                        .rounded(px(3.0))
                        .bg(colors.hover)
                        .child(kind_badge(&item.kind)),
                )
                .into_any_element(),

            1 => div()
                .h_full()
                .flex()
                .items_center()
                .text_sm()
                .text_color(colors.text_secondary)
                .truncate()
                .child(item.publisher.to_string())
                .into_any_element(),

            2 => div()
                .h_full()
                .flex()
                .items_center()
                .text_sm()
                .text_color(colors.text_secondary)
                .truncate()
                .child(item.line.to_string())
                .into_any_element(),

            3 => div()
                .h_full()
                .flex()
                .items_center()
                .text_sm()
                .text_color(colors.text_secondary)
                .child(item.pages.to_string())
                .into_any_element(),

            4 => div()
                .h_full()
                .flex()
                .items_center()
                .text_sm()
                .text_color(colors.text_secondary)
                .child(format!("{:.0} MB", item.size_mb))
                .into_any_element(),

            5 => div()
                .h_full()
                .flex()
                .items_center()
                .text_sm()
                .text_color(colors.text_secondary)
                .child(item.year.to_string())
                .into_any_element(),

            6 => div()
                .h_full()
                .flex()
                .items_center()
                .justify_center()
                .child(render_status(item.status, &colors))
                .into_any_element(),

            7 => {
                if item.status == ItemStatus::Downloaded {
                    let item_open_path = self.storage_root.join("items").join(&*item.id);
                    let open_elem_id: Arc<str> = Arc::from(format!("open-row-{}", &*item.id));
                    div()
                        .id(open_elem_id)
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .on_click(move |_, _, _| {
                            use crate::util::item_opener::{ItemOpener, OpenError};

                            if !item_open_path.exists() {
                                tracing::warn!(
                                    path = %item_open_path.display(),
                                    "open: file not found — item may need re-download"
                                );
                                return;
                            }
                            if let Err(e) = ItemOpener::open(&item_open_path) {
                                match e {
                                    OpenError::FileNotFound(path) => {
                                        tracing::warn!("open: file not found: {path}");
                                    }
                                    OpenError::NoDefaultApp => {
                                        tracing::warn!("open: no default application configured");
                                    }
                                    OpenError::OsFailed(msg) => {
                                        tracing::warn!("open: OS failed: {msg}");
                                    }
                                    OpenError::MultipleFilesRequireSelection => {
                                        tracing::warn!("open: multiple files require selection");
                                    }
                                }
                            }
                        })
                        .child(div().text_xs().text_color(colors.text_tertiary).child("▶"))
                        .into_any_element()
                } else {
                    div().h_full().into_any_element()
                }
            }

            8 => {
                let ctrl = self.controller.clone();
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(render_thumbnail_menu(&item, ctrl))
                    .into_any_element()
            }

            _ => div().into_any_element(),
        }
    }
}

// ── CatalogView ───────────────────────────────────────────────────────────────

/// GPUI view for the catalog area. Holds scroll state and delegates to
/// `DataTable` for list layout and `uniform_list` for thumbs/grid layouts.
pub struct CatalogView {
    controller: Entity<LibraryController>,
    settings: Entity<SettingsController>,
    scroll_handle: UniformListScrollHandle,
    catalog_list_table: Entity<TableState<CatalogListDelegate>>,
    /// Cached items-per-row for the grid layout; updated each render from
    /// the window viewport width. Initialized to 4 as a safe default.
    items_per_row: usize,
    /// Cached publisher grouping of the controller's visible items.
    /// Invalidated (set to `None`) on `LibraryChanged`; repopulated lazily
    /// during `render()` so grouped presentation modes avoid re-grouping on
    /// every hover-triggered re-render.
    grouped_cache: Option<Vec<PublisherGroup>>,
}

impl CatalogView {
    /// Creates a new `CatalogView` connected to the given controller and settings.
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        controller: Entity<LibraryController>,
        settings: Entity<SettingsController>,
    ) -> Self {
        let storage_root = settings.read(cx).snapshot().storage_root_path;
        let cols = list_columns();
        let col_count = cols.len();
        let delegate = CatalogListDelegate {
            controller: controller.clone(),
            storage_root,
            columns: cols,
            user_widths: vec![None; col_count],
        };
        let catalog_list_table = cx.new(|cx| {
            TableState::new(delegate, window, cx)
                .row_selectable(true)
                .col_selectable(false)
                .col_movable(false)
                .col_resizable(true)
                .sortable(true)
        });

        cx.subscribe(&controller, {
            let table = catalog_list_table.clone();
            move |this, _ctrl, _event: &LibraryChanged, cx| {
                table.update(cx, |state, cx| state.refresh(cx));
                this.grouped_cache = None;
            }
        })
        .detach();

        cx.subscribe(
            &catalog_list_table,
            |this, table, event: &TableEvent, cx| match event {
                TableEvent::SelectRow(row_ix) => {
                    let row_ix = *row_ix;
                    let items = this
                        .controller
                        .read(cx)
                        .visible_items_slice(row_ix..row_ix + 1);
                    if let Some(item) = items.first() {
                        let id = Arc::clone(&item.id);
                        this.controller
                            .update(cx, |ctrl, cx| ctrl.select_item(id, cx));
                    }
                }
                TableEvent::ColumnWidthsChanged(widths) => {
                    let widths = widths.clone();
                    table.update(cx, |state, _cx| {
                        let delegate = state.delegate_mut();
                        if widths.len() == delegate.user_widths.len() {
                            for (slot, &w) in delegate.user_widths.iter_mut().zip(widths.iter()) {
                                *slot = Some(w);
                            }
                        }
                    });
                }
                _ => {}
            },
        )
        .detach();

        Self {
            controller,
            settings,
            scroll_handle: UniformListScrollHandle::default(),
            catalog_list_table,
            items_per_row: 4,
            grouped_cache: None,
        }
    }

    /// Returns the cached publisher grouping, computing and storing it first if stale.
    fn grouped_items(&mut self, cx: &App) -> Vec<PublisherGroup> {
        if self.grouped_cache.is_none() {
            let items = self.controller.read(cx).visible_items();
            self.grouped_cache = Some(group_by_publisher(items));
        }
        self.grouped_cache.clone().unwrap_or_default()
    }
}

impl Render for CatalogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snap = self.controller.read(cx).snapshot();
        let item_count = self.controller.read(cx).visible_items_count();
        let theme = cx.global::<LibriTheme>().clone();
        let colors = theme.colors;
        let density = theme.density_constants;
        let storage_root = self.settings.read(cx).snapshot().storage_root_path;

        let pad_top = density.catalog_pad_top;
        let pad_side = density.catalog_pad_side;
        let pad_bottom = density.catalog_pad_bottom;
        let empty_reason = if item_count == 0 {
            Some(if snap.total_count == 0 {
                EmptyReason::LibraryEmpty
            } else {
                EmptyReason::NoMatches
            })
        } else {
            None
        };

        // Update items_per_row estimate for grid layout using the viewport width.
        // Subtract a rough sidebar width (220px) and both side pads.
        let viewport_w = window.viewport_size().width.as_f32();
        let usable_w = (viewport_w - 220.0 - pad_side.as_f32() * 2.0).max(0.0);
        let card_pitch = density.card_min_width + density.card_gap_x.as_f32();
        self.items_per_row = ((usable_w / card_pitch) as usize).max(1);

        let items_per_row = self.items_per_row;
        let scroll_handle = self.scroll_handle.clone();
        let ctrl = self.controller.clone();

        let outer = div().flex_1().min_h_0().flex().flex_col();

        let root = div()
            .flex_1()
            .min_h_0()
            .flex()
            .flex_col()
            .overflow_y_scrollbar()
            .pt(pad_top)
            .pb(pad_bottom);

        if snap.catalog_loading && item_count == 0 {
            return outer
                .child(
                    root.justify_center()
                        .items_center()
                        .child(Spinner::new().with_size(Size::Large)),
                )
                .into_any_element();
        }

        if let Some(reason) = empty_reason {
            let empty_state = match reason {
                EmptyReason::LibraryEmpty => {
                    render_library_empty_state(colors.text_tertiary).into_any_element()
                }
                EmptyReason::NoMatches => {
                    render_no_matches_state(&snap.search_query, colors.text_tertiary)
                        .into_any_element()
                }
            };
            return outer.child(root.child(empty_state)).into_any_element();
        }

        let current_page = snap.current_page;
        let total_pages = snap.total_pages;
        let page_size = snap.page_size;
        let ctrl_for_page = self.controller.clone();
        let ctrl_for_first = self.controller.clone();
        let ctrl_for_last = self.controller.clone();
        let ctrl_for_page_size = self.controller.clone();

        let content: AnyElement = match (snap.presentation, snap.grouped) {
            // ── List, ungrouped — DataTable (handles header/row alignment) ──
            (CatalogPresentation::List, false) => {
                use gpui_component::Size;
                root.px(pad_side)
                    .child(
                        DataTable::new(&self.catalog_list_table)
                            .with_size(Size::Size(density.row_text_height))
                            .bordered(false)
                            .scrollbar_visible(true, false),
                    )
                    .into_any_element()
            }

            // ── List, grouped — non-virtualized; raw flex rows with shared widths ──
            (CatalogPresentation::List, true) => {
                let groups = self.grouped_items(cx);
                let cols = list_columns();
                root.px(pad_side)
                    .children(groups.into_iter().map(|g| {
                        let c = colors.clone();
                        let d = density.clone();
                        let e = self.controller.clone();
                        let s = storage_root.clone();
                        let header_cols = cols.clone();
                        let row_cols = cols.clone();
                        div()
                            .child(render_group_header(&g.publisher, g.items.len(), &c))
                            .child(render_grouped_list_header(&c, &header_cols))
                            .children(g.items.into_iter().map(move |item| {
                                render_grouped_list_row(
                                    &item,
                                    &c,
                                    &d,
                                    e.clone(),
                                    s.clone(),
                                    &row_cols,
                                )
                            }))
                    }))
                    .into_any_element()
            }

            // ── Thumbs, ungrouped — virtualized ───────────────────────────
            (CatalogPresentation::Thumbs, false) => {
                let c = colors.clone();
                let d = density.clone();
                let s = storage_root.clone();
                root.px(pad_side)
                    .child(
                        uniform_list("catalog-thumbs", item_count, move |range, _window, cx| {
                            let items = ctrl.read(cx).visible_items_slice(range);
                            let covers: Vec<Option<Arc<Image>>> = {
                                let cache = cx.global::<CoverCache>();
                                items.iter().map(|item| cache.get(&item.id)).collect()
                            };
                            items
                                .iter()
                                .zip(covers)
                                .map(|(item, cover)| {
                                    render_thumb_row(item, cover, &c, &d, ctrl.clone(), s.clone())
                                        .into_any_element()
                                })
                                .collect()
                        })
                        .track_scroll(&scroll_handle)
                        .flex_1()
                        .min_h_0(),
                    )
                    .into_any_element()
            }

            // ── Thumbs, grouped — non-virtualized ─────────────────────────
            (CatalogPresentation::Thumbs, true) => {
                let groups = self.grouped_items(cx);
                let cover_cache = {
                    let cache = cx.global::<CoverCache>();
                    cache.images.clone()
                };
                root.px(pad_side)
                    .children(groups.into_iter().map(|g| {
                        let c = colors.clone();
                        let d = density.clone();
                        let e = self.controller.clone();
                        let s = storage_root.clone();
                        let cc = cover_cache.clone();
                        div()
                            .child(render_group_header(&g.publisher, g.items.len(), &c))
                            .children(g.items.into_iter().map(move |item| {
                                let cover = cc.get(&item.id).cloned();
                                render_thumb_row(&item, cover, &c, &d, e.clone(), s.clone())
                            }))
                    }))
                    .into_any_element()
            }

            // ── Grid, ungrouped — row-virtualized ─────────────────────────
            (CatalogPresentation::Grid, false) => {
                let row_count = item_count.div_ceil(items_per_row);
                let c = colors.clone();
                let d = density.clone();
                let s = storage_root.clone();
                root.px(pad_side)
                    .child(
                        uniform_list("catalog-grid", row_count, move |row_range, _window, cx| {
                            let range_start = row_range.start;
                            let item_start = range_start * items_per_row;
                            let item_end = (row_range.end * items_per_row).min(item_count);
                            let items = ctrl.read(cx).visible_items_slice(item_start..item_end);
                            let covers: Vec<Option<Arc<Image>>> = {
                                let cache = cx.global::<CoverCache>();
                                items.iter().map(|item| cache.get(&item.id)).collect()
                            };
                            row_range
                                .map(|row| {
                                    let offset = (row - range_start) * items_per_row;
                                    let row_end = (offset + items_per_row).min(items.len());
                                    let row_items = &items[offset..row_end];
                                    let row_covers = &covers[offset..row_end];
                                    div()
                                        .flex()
                                        .gap(d.card_gap_x)
                                        .mb(d.card_gap_y)
                                        .children(row_items.iter().zip(row_covers.iter()).map(
                                            |(item, cover)| {
                                                render_grid_card(
                                                    item,
                                                    cover.clone(),
                                                    &c,
                                                    d.card_min_width,
                                                    ctrl.clone(),
                                                    s.clone(),
                                                )
                                            },
                                        ))
                                        .into_any_element()
                                })
                                .collect()
                        })
                        .track_scroll(&scroll_handle)
                        .flex_1()
                        .min_h_0(),
                    )
                    .into_any_element()
            }

            // ── Grid, grouped — non-virtualized ───────────────────────────
            (CatalogPresentation::Grid, true) => {
                let groups = self.grouped_items(cx);
                let cover_cache = {
                    let cache = cx.global::<CoverCache>();
                    cache.images.clone()
                };
                root.px(pad_side)
                    .children(groups.into_iter().map(|g| {
                        let c = colors.clone();
                        let d = density.clone();
                        let e = self.controller.clone();
                        let s = storage_root.clone();
                        let cc = cover_cache.clone();
                        div()
                            .child(render_group_header(&g.publisher, g.items.len(), &c))
                            .child(render_grid(g.items, cc, c, d, e, s))
                    }))
                    .into_any_element()
            }
        };

        let mut result = outer.child(content);

        if item_count > 0 {
            let mut bar = div()
                .flex_none()
                .px(pad_side)
                .py(px(8.0))
                .flex()
                .items_center()
                .justify_center()
                .gap(px(16.0));

            if total_pages > 1 {
                bar = bar
                    .child(
                        Button::new("page-first-btn")
                            .ghost()
                            .label(format!("\u{00ab} {}", t!("catalog.pagination_first")))
                            .disabled(current_page == 1)
                            .on_click(move |_, _, cx| {
                                ctrl_for_first.update(cx, |ctrl, cx| ctrl.set_page(1, cx));
                            }),
                    )
                    .child(
                        Pagination::new("catalog-pagination")
                            .current_page(current_page)
                            .total_pages(total_pages)
                            .on_click(move |page, _, cx| {
                                ctrl_for_page.update(cx, |ctrl, cx| ctrl.set_page(*page, cx));
                            }),
                    )
                    .child(
                        Button::new("page-last-btn")
                            .ghost()
                            .label(format!("{} \u{00bb}", t!("catalog.pagination_last")))
                            .disabled(current_page == total_pages)
                            .on_click(move |_, _, cx| {
                                ctrl_for_last.update(cx, |ctrl, cx| ctrl.set_page(total_pages, cx));
                            }),
                    );
            }

            bar = bar.child(render_page_size_selector(page_size, ctrl_for_page_size));

            result = result.child(bar);
        }

        result.into_any_element()
    }
}

// ── Page size selector ──────────────────────────────────────────────────────────

/// Page size options offered in the pagination area, mirroring
/// [`LibraryController::set_page_size`](crate::controllers::library::LibraryController::set_page_size).
const PAGE_SIZE_OPTIONS: [usize; 5] = [10, 25, 50, 100, 200];

/// Renders the "N / page" dropdown control in the pagination area.
fn render_page_size_selector(
    current: usize,
    entity: Entity<LibraryController>,
) -> impl IntoElement + 'static {
    let label = t!("toolbar.page_size_label", n = current).to_string();

    Button::new("page-size-selector")
        .ghost()
        .label(label)
        .dropdown_caret(true)
        .dropdown_menu(move |menu, _, _| {
            let mut m = menu;
            for size in PAGE_SIZE_OPTIONS {
                let e = entity.clone();
                let item_label = t!("toolbar.page_size_label", n = size).to_string();
                m = m.item(
                    PopupMenuItem::new(item_label)
                        .checked(size == current)
                        .on_click(move |_, _, cx| {
                            e.update(cx, |ctrl, cx| ctrl.set_page_size(size, cx));
                        }),
                );
            }
            m
        })
}

// ── Empty state ───────────────────────────────────────────────────────────────

fn render_no_matches_state(
    search_query: &str,
    text_color: gpui::Hsla,
) -> impl IntoElement + 'static {
    let hint = if search_query.is_empty() {
        t!("catalog.try_different_section")
    } else {
        t!("catalog.try_clear_search")
    };

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .h_full()
        .gap(px(12.0))
        .child(div().text_2xl().text_color(text_color).child("⊘"))
        .child(
            div()
                .text_sm()
                .text_color(text_color)
                .child(t!("catalog.no_titles_match")),
        )
        .child(div().text_xs().text_color(text_color).child(hint))
}

fn render_library_empty_state(text_color: gpui::Hsla) -> impl IntoElement + 'static {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .h_full()
        .gap(px(12.0))
        .child(div().text_2xl().text_color(text_color).child("⊘"))
        .child(
            div()
                .text_sm()
                .text_color(text_color)
                .child(t!("catalog.library_empty")),
        )
}

// ── Group header ──────────────────────────────────────────────────────────────

fn render_group_header(
    publisher: &str,
    count: usize,
    colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_tertiary = colors.text_tertiary;
    let publisher = publisher.to_string();
    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .py(px(10.0))
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(publisher),
        )
        .child(
            div()
                .text_xs()
                .text_color(text_tertiary)
                .child(count.to_string()),
        )
}

// ── Status glyph ──────────────────────────────────────────────────────────────

fn render_status(status: ItemStatus, colors: &ColorTokens) -> AnyElement {
    let accent = colors.accent;
    let text_tertiary = colors.text_tertiary;
    match status {
        ItemStatus::Downloaded => div()
            .size(px(7.0))
            .rounded_full()
            .bg(accent)
            .flex_none()
            .into_any_element(),
        ItemStatus::Cloud => div()
            .text_xs()
            .text_color(text_tertiary)
            .flex_none()
            .child("☁")
            .into_any_element(),
    }
}

// ── Thumbnail context menu ─────────────────────────────────────────────────────

/// Renders a small "⋯" button that opens a dropdown with a "Load Thumbnail"
/// action.  The action is disabled when no `cover_url` is present or when the
/// cooldown guard is active.
fn render_thumbnail_menu(
    item: &LibraryItem,
    entity: Entity<LibraryController>,
) -> impl IntoElement + 'static + use<> {
    let can_load = item.cover_url.is_some() && thumbnail_cooldown_elapsed(item);
    let cover_url = item.cover_url.clone();
    let btn_id: Arc<str> = Arc::from(format!("ctx-thumb-{}", &*item.id));

    Button::new(btn_id)
        .ghost()
        .small()
        .label("\u{22ef}")
        .dropdown_menu(move |menu, _, _| {
            let eu = entity.clone();
            let url = cover_url.clone();
            menu.item(
                PopupMenuItem::new(t!("catalog.action_load_thumbnail"))
                    .disabled(!can_load)
                    .on_click(move |_, _, cx| {
                        if let Some(ref u) = url {
                            eu.update(cx, |ctrl, cx| ctrl.load_thumbnail(Arc::clone(u), cx));
                        }
                    }),
            )
        })
}

// ── Reveal action ─────────────────────────────────────────────────────────────

fn platform_reveal_label() -> std::borrow::Cow<'static, str> {
    #[cfg(target_os = "macos")]
    return t!("catalog.action_show_in_finder");
    #[cfg(target_os = "windows")]
    return t!("catalog.action_show_in_explorer");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return t!("catalog.action_show_in_files");
}

// ── Grouped list layout ───────────────────────────────────────────────────────

/// Header row for the grouped list. Column widths come from `cols` so they
/// always match the data rows rendered by `render_grouped_list_row`.
fn render_grouped_list_header(
    colors: &ColorTokens,
    cols: &[Column],
) -> impl IntoElement + 'static + use<> {
    let border = colors.border;
    let text_tertiary = colors.text_tertiary;
    let mut row = div()
        .flex()
        .items_center()
        .h(px(28.0))
        .border_b_1()
        .border_color(border)
        .text_xs()
        .text_color(text_tertiary)
        .child(div().flex_1().child(cols[0].name.to_string()));
    for col in &cols[1..] {
        let name = col.name.to_string();
        let width = col.width;
        row = row.child(div().w(width).child(name));
    }
    row
}

/// A single data row for the grouped list. Column widths come from `cols`.
fn render_grouped_list_row(
    item: &LibraryItem,
    colors: &ColorTokens,
    density: &DensityConstants,
    entity: Entity<LibraryController>,
    storage_root_path: PathBuf,
    cols: &[Column],
) -> impl IntoElement + 'static + use<> {
    let id = Arc::clone(&item.id);
    let title = item.title.to_string();
    let kind = item.kind.to_string();
    let publisher = item.publisher.to_string();
    let line = item.line.to_string();
    let pages = item.pages;
    let size_mb = item.size_mb;
    let year = item.year;
    let status = item.status;
    let h = density.row_text_height;
    let colors = colors.clone();
    let reveal_item_id = Arc::clone(&item.id);
    let ctx_cover_url = item.cover_url.clone();
    let ctx_can_load = item.cover_url.is_some() && thumbnail_cooldown_elapsed(item);
    let ctx_btn_id: Arc<str> = Arc::from(format!("ctx-list-{}", &*item.id));

    // Fixed-width columns 1–8 use widths from the shared column definitions.
    let [_, pub_w, sys_w, pgs_w, sz_w, yr_w, st_w, rv_w, ctx_w] =
        std::array::from_fn::<_, 9, _>(|i| cols.get(i).map_or(px(0.), |c| c.width));

    let reveal_col: AnyElement = if status == ItemStatus::Downloaded {
        let item_reveal_path = storage_root_path.join("items").join(&*reveal_item_id);
        let reveal_elem_id: Arc<str> = Arc::from(format!("reveal-row-{}", &*reveal_item_id));
        div()
            .id(reveal_elem_id)
            .w(rv_w)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .on_click(move |_, _, _| {
                if !item_reveal_path.exists() {
                    tracing::warn!(
                        path = %item_reveal_path.display(),
                        "reveal: file not found — item may need re-download"
                    );
                    return;
                }
                if let Err(e) = reveal_in_file_manager(&item_reveal_path) {
                    tracing::warn!("reveal_in_file_manager failed: {e}");
                }
            })
            .child(div().text_xs().text_color(colors.text_tertiary).child("↗"))
            .into_any_element()
    } else {
        div().w(rv_w).into_any_element()
    };

    let ctx_id = Arc::clone(&id);
    let ctx_entity = entity.clone();
    let ctx_path = storage_root_path.join("items").join(&*id);
    let entity_click = entity.clone();
    div()
        .id(Arc::clone(&id))
        .flex()
        .items_center()
        .h(h)
        .border_b_1()
        .border_color(colors.border)
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            entity_click.update(cx, |ctrl, cx| ctrl.select_item(Arc::clone(&id), cx));
        })
        .child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .gap(px(6.0))
                .min_w_0()
                .child(
                    div()
                        .text_sm()
                        .text_color(colors.text_primary)
                        .truncate()
                        .child(title),
                )
                .child(
                    div()
                        .flex_none()
                        .text_xs()
                        .text_color(colors.text_tertiary)
                        .px(px(4.0))
                        .py(px(1.0))
                        .rounded(px(3.0))
                        .bg(colors.hover)
                        .child(kind_badge(&kind)),
                ),
        )
        .child(
            div()
                .w(pub_w)
                .text_sm()
                .text_color(colors.text_secondary)
                .truncate()
                .child(publisher),
        )
        .child(
            div()
                .w(sys_w)
                .text_sm()
                .text_color(colors.text_secondary)
                .truncate()
                .child(line),
        )
        .child(
            div()
                .w(pgs_w)
                .text_sm()
                .text_color(colors.text_secondary)
                .child(pages.to_string()),
        )
        .child(
            div()
                .w(sz_w)
                .text_sm()
                .text_color(colors.text_secondary)
                .child(format!("{size_mb:.0} MB")),
        )
        .child(
            div()
                .w(yr_w)
                .text_sm()
                .text_color(colors.text_secondary)
                .child(year.to_string()),
        )
        .child(
            div()
                .w(st_w)
                .flex()
                .items_center()
                .justify_center()
                .child(render_status(status, &colors)),
        )
        .child(reveal_col)
        .child(
            div()
                .w(ctx_w)
                .flex()
                .items_center()
                .justify_center()
                .child({
                    let eu = entity.clone();
                    Button::new(ctx_btn_id)
                        .ghost()
                        .small()
                        .label("\u{22ef}")
                        .dropdown_menu(move |menu, _, _| {
                            let eu2 = eu.clone();
                            let url = ctx_cover_url.clone();
                            menu.item(
                                PopupMenuItem::new(t!("catalog.action_load_thumbnail"))
                                    .disabled(!ctx_can_load)
                                    .on_click(move |_, _, cx| {
                                        if let Some(ref u) = url {
                                            eu2.update(cx, |ctrl, cx| {
                                                ctrl.load_thumbnail(Arc::clone(u), cx);
                                            });
                                        }
                                    }),
                            )
                        })
                }),
        )
        .context_menu(move |menu, _, _| match status {
            ItemStatus::Downloaded => {
                let open_path = ctx_path.clone();
                let reveal_path = ctx_path.clone();
                let remove_id = Arc::clone(&ctx_id);
                let entity_remove = ctx_entity.clone();
                menu.item(
                    PopupMenuItem::new(t!("catalog.action_open")).on_click(move |_, _, _| {
                        use crate::util::item_opener::{ItemOpener, OpenError};

                        if !open_path.exists() {
                            tracing::warn!(
                                path = %open_path.display(),
                                "open: file not found"
                            );
                            return;
                        }
                        if let Err(e) = ItemOpener::open(&open_path) {
                            match e {
                                OpenError::FileNotFound(path) => {
                                    tracing::warn!("open: file not found: {path}");
                                }
                                OpenError::NoDefaultApp => {
                                    tracing::warn!("open: no default application configured");
                                }
                                OpenError::OsFailed(msg) => {
                                    tracing::warn!("open: OS failed: {msg}");
                                }
                                OpenError::MultipleFilesRequireSelection => {
                                    tracing::warn!("open: multiple files require selection");
                                }
                            }
                        }
                    }),
                )
                .item(
                    PopupMenuItem::new(platform_reveal_label()).on_click(move |_, _, _| {
                        if !reveal_path.exists() {
                            tracing::warn!(
                                path = %reveal_path.display(),
                                "reveal: file not found"
                            );
                            return;
                        }
                        if let Err(e) = reveal_in_file_manager(&reveal_path) {
                            tracing::warn!("reveal_in_file_manager failed: {e}");
                        }
                    }),
                )
                .item(
                    PopupMenuItem::new(t!("catalog.action_remove_download")).on_click(
                        move |_, _, cx| {
                            entity_remove
                                .update(cx, |ctrl, cx| ctrl.toggle_download(&remove_id, cx));
                        },
                    ),
                )
            }
            ItemStatus::Cloud => {
                let dl_id = Arc::clone(&ctx_id);
                let entity_dl = ctx_entity.clone();
                menu.item(PopupMenuItem::new(t!("catalog.action_download")).on_click(
                    move |_, _, cx| {
                        entity_dl.update(cx, |ctrl, cx| ctrl.toggle_download(&dl_id, cx));
                    },
                ))
            }
        })
}

// ── Thumbs layout ─────────────────────────────────────────────────────────────

fn render_thumb_row(
    item: &LibraryItem,
    cover_image: Option<Arc<Image>>,
    colors: &ColorTokens,
    density: &DensityConstants,
    entity: Entity<LibraryController>,
    storage_root_path: PathBuf,
) -> impl IntoElement + 'static + use<> {
    let id = Arc::clone(&item.id);
    let title = item.title.to_string();
    let publisher = item.publisher.to_string();
    let line = item.line.to_string();
    let kind = item.kind.to_string();
    let pages = item.pages;
    let size_mb = item.size_mb;
    let year = item.year;
    let status = item.status;
    let thumb_w = density.thumb_width;
    let thumb_h = thumb_w * 10.0 / 7.0;
    let row_h = density.thumb_row_height;
    let colors = colors.clone();
    let ctx_menu = render_thumbnail_menu(item, entity.clone());
    let ctx_id = Arc::clone(&id);
    let ctx_entity = entity.clone();
    let ctx_path = storage_root_path.join("items").join(&*id);

    let cover: AnyElement = if let Some(image) = cover_image {
        img(image)
            .w(px(thumb_w))
            .h(px(thumb_h))
            .object_fit(ObjectFit::Cover)
            .into_any_element()
    } else {
        render_generative_cover(item, thumb_w, thumb_h, false).into_any_element()
    };

    let cover_cell: AnyElement = {
        let inner = div()
            .rounded(px(3.0))
            .overflow_hidden()
            .flex_none()
            .child(cover);
        if status == ItemStatus::Downloaded {
            Badge::new()
                .dot()
                .color(gpui::green())
                .child(inner)
                .into_any_element()
        } else {
            inner.into_any_element()
        }
    };

    div()
        .id(Arc::clone(&id))
        .flex()
        .items_center()
        .gap(px(12.0))
        .h(row_h)
        .border_b_1()
        .border_color(colors.border)
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            entity.update(cx, |ctrl, cx| ctrl.select_item(Arc::clone(&id), cx));
        })
        .child(cover_cell)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .justify_center()
                .gap(px(2.0))
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(colors.text_primary)
                        .truncate()
                        .child(title),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(colors.text_secondary)
                        .truncate()
                        .child(format!("{publisher} · {line}")),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(div().text_xs().text_color(colors.text_tertiary).child(kind))
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors.text_tertiary)
                                .child(format!("{pages} pp · {size_mb:.0} MB · {year}")),
                        ),
                ),
        )
        .child(render_status(status, &colors))
        .child(ctx_menu)
        .context_menu(move |menu, _, _| match status {
            ItemStatus::Downloaded => {
                let open_path = ctx_path.clone();
                let reveal_path = ctx_path.clone();
                let remove_id = Arc::clone(&ctx_id);
                let entity_remove = ctx_entity.clone();
                menu.item(
                    PopupMenuItem::new(t!("catalog.action_open")).on_click(move |_, _, _| {
                        use crate::util::item_opener::{ItemOpener, OpenError};

                        if !open_path.exists() {
                            tracing::warn!(
                                path = %open_path.display(),
                                "open: file not found"
                            );
                            return;
                        }
                        if let Err(e) = ItemOpener::open(&open_path) {
                            match e {
                                OpenError::FileNotFound(path) => {
                                    tracing::warn!("open: file not found: {path}");
                                }
                                OpenError::NoDefaultApp => {
                                    tracing::warn!("open: no default application configured");
                                }
                                OpenError::OsFailed(msg) => {
                                    tracing::warn!("open: OS failed: {msg}");
                                }
                                OpenError::MultipleFilesRequireSelection => {
                                    tracing::warn!("open: multiple files require selection");
                                }
                            }
                        }
                    }),
                )
                .item(
                    PopupMenuItem::new(platform_reveal_label()).on_click(move |_, _, _| {
                        if !reveal_path.exists() {
                            tracing::warn!(
                                path = %reveal_path.display(),
                                "reveal: file not found"
                            );
                            return;
                        }
                        if let Err(e) = reveal_in_file_manager(&reveal_path) {
                            tracing::warn!("reveal_in_file_manager failed: {e}");
                        }
                    }),
                )
                .item(
                    PopupMenuItem::new(t!("catalog.action_remove_download")).on_click(
                        move |_, _, cx| {
                            entity_remove
                                .update(cx, |ctrl, cx| ctrl.toggle_download(&remove_id, cx));
                        },
                    ),
                )
            }
            ItemStatus::Cloud => {
                let dl_id = Arc::clone(&ctx_id);
                let entity_dl = ctx_entity.clone();
                menu.item(PopupMenuItem::new(t!("catalog.action_download")).on_click(
                    move |_, _, cx| {
                        entity_dl.update(cx, |ctrl, cx| ctrl.toggle_download(&dl_id, cx));
                    },
                ))
            }
        })
}

// ── Grid layout ───────────────────────────────────────────────────────────────

fn render_grid(
    items: Vec<LibraryItem>,
    cover_cache: HashMap<Arc<str>, Arc<Image>>,
    colors: ColorTokens,
    density: DensityConstants,
    entity: Entity<LibraryController>,
    storage_root_path: PathBuf,
) -> impl IntoElement + 'static {
    let gap_x = density.card_gap_x;
    let gap_y = density.card_gap_y;
    let min_w = density.card_min_width;

    div()
        .flex()
        .flex_wrap()
        .gap(gap_x)
        .mb(gap_y)
        .children(items.into_iter().map(move |item| {
            let cover = cover_cache.get(&item.id).cloned();
            render_grid_card(
                &item,
                cover,
                &colors,
                min_w,
                entity.clone(),
                storage_root_path.clone(),
            )
        }))
}

fn render_grid_card(
    item: &LibraryItem,
    cover_image: Option<Arc<Image>>,
    colors: &ColorTokens,
    card_w: f32,
    entity: Entity<LibraryController>,
    storage_root_path: PathBuf,
) -> impl IntoElement + 'static + use<> {
    let id = Arc::clone(&item.id);
    let title = item.title.to_string();
    let publisher = item.publisher.to_string();
    let status = item.status;
    let cover_h = card_w * 10.0 / 7.0;
    let reveal_item_id = Arc::clone(&item.id);

    let cover: AnyElement = if let Some(image) = cover_image {
        img(image)
            .w(px(card_w))
            .h(px(cover_h))
            .object_fit(ObjectFit::Cover)
            .into_any_element()
    } else {
        render_generative_cover(item, card_w, cover_h, true).into_any_element()
    };
    let colors = colors.clone();
    let ctx_menu = render_thumbnail_menu(item, entity.clone());
    let ctx_id = Arc::clone(&id);
    let ctx_entity = entity.clone();
    let ctx_path = storage_root_path.join("items").join(&*id);

    let reveal_row: AnyElement = if status == ItemStatus::Downloaded {
        let item_reveal_path = storage_root_path.join("items").join(&*reveal_item_id);
        let reveal_elem_id: Arc<str> = Arc::from(format!("reveal-grid-{}", &*reveal_item_id));
        div()
            .id(reveal_elem_id)
            .mt(px(2.0))
            .cursor_pointer()
            .on_click(move |_, _, _| {
                if !item_reveal_path.exists() {
                    tracing::warn!(
                        path = %item_reveal_path.display(),
                        "reveal: file not found — item may need re-download"
                    );
                    return;
                }
                if let Err(e) = reveal_in_file_manager(&item_reveal_path) {
                    tracing::warn!("reveal_in_file_manager failed: {e}");
                }
            })
            .child(
                div()
                    .text_xs()
                    .text_color(colors.text_tertiary)
                    .child(platform_reveal_label()),
            )
            .into_any_element()
    } else {
        div().into_any_element()
    };

    let cover_cell: AnyElement = {
        let inner = div().child(cover);
        if status == ItemStatus::Downloaded {
            Badge::new()
                .dot()
                .color(gpui::green())
                .child(inner)
                .into_any_element()
        } else {
            inner.into_any_element()
        }
    };

    div()
        .id(Arc::clone(&id))
        .w(px(card_w))
        .flex()
        .flex_col()
        .rounded(px(6.0))
        .overflow_hidden()
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            entity.update(cx, |ctrl, cx| ctrl.select_item(Arc::clone(&id), cx));
        })
        .child(cover_cell)
        .child(
            div()
                .px(px(4.0))
                .pt(px(4.0))
                .pb(px(6.0))
                .flex()
                .flex_col()
                .gap(px(1.0))
                .child(
                    div()
                        .text_xs()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(colors.text_primary)
                        .truncate()
                        .child(title),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors.text_tertiary)
                                .truncate()
                                .child(publisher),
                        )
                        .child(render_status(status, &colors)),
                )
                .child(reveal_row)
                .child(ctx_menu),
        )
        .context_menu(move |menu, _, _| match status {
            ItemStatus::Downloaded => {
                let open_path = ctx_path.clone();
                let reveal_path = ctx_path.clone();
                let remove_id = Arc::clone(&ctx_id);
                let entity_remove = ctx_entity.clone();
                menu.item(
                    PopupMenuItem::new(t!("catalog.action_open")).on_click(move |_, _, _| {
                        use crate::util::item_opener::{ItemOpener, OpenError};

                        if !open_path.exists() {
                            tracing::warn!(
                                path = %open_path.display(),
                                "open: file not found"
                            );
                            return;
                        }
                        if let Err(e) = ItemOpener::open(&open_path) {
                            match e {
                                OpenError::FileNotFound(path) => {
                                    tracing::warn!("open: file not found: {path}");
                                }
                                OpenError::NoDefaultApp => {
                                    tracing::warn!("open: no default application configured");
                                }
                                OpenError::OsFailed(msg) => {
                                    tracing::warn!("open: OS failed: {msg}");
                                }
                                OpenError::MultipleFilesRequireSelection => {
                                    tracing::warn!("open: multiple files require selection");
                                }
                            }
                        }
                    }),
                )
                .item(
                    PopupMenuItem::new(platform_reveal_label()).on_click(move |_, _, _| {
                        if !reveal_path.exists() {
                            tracing::warn!(
                                path = %reveal_path.display(),
                                "reveal: file not found"
                            );
                            return;
                        }
                        if let Err(e) = reveal_in_file_manager(&reveal_path) {
                            tracing::warn!("reveal_in_file_manager failed: {e}");
                        }
                    }),
                )
                .item(
                    PopupMenuItem::new(t!("catalog.action_remove_download")).on_click(
                        move |_, _, cx| {
                            entity_remove
                                .update(cx, |ctrl, cx| ctrl.toggle_download(&remove_id, cx));
                        },
                    ),
                )
            }
            ItemStatus::Cloud => {
                let dl_id = Arc::clone(&ctx_id);
                let entity_dl = ctx_entity.clone();
                menu.item(PopupMenuItem::new(t!("catalog.action_download")).on_click(
                    move |_, _, cx| {
                        entity_dl.update(cx, |ctrl, cx| ctrl.toggle_download(&dl_id, cx));
                    },
                ))
            }
        })
}
