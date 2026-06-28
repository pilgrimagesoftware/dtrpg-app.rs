//! Catalog view: list, thumbs, and grid layouts with grouping and empty state.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    div, px, uniform_list, AnyElement, Context, Entity, IntoElement, ParentElement,
    Render, Styled, UniformListScrollHandle, Window,
};
use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::data::enums::*;
use crate::data::enums::CatalogPresentation;
use crate::data::library::LibraryItem;
use crate::data::theme::{ColorTokens, DensityConstants, LibriTheme};
use crate::ui::library::cover::render_generative_cover;
use crate::util::publisher::group_by_publisher;
use crate::util::reveal::reveal_in_file_manager;

// ── CatalogView ───────────────────────────────────────────────────────────────

/// GPUI view for the catalog area. Holds scroll state and delegates to
/// `uniform_list` for list and thumbs layouts, keeping frame layout cost
/// O(visible rows) rather than O(total items).
pub struct CatalogView {
    controller: Entity<LibraryController>,
    settings: Entity<SettingsController>,
    scroll_handle: UniformListScrollHandle,
    /// Cached items-per-row for the grid layout; updated each render from
    /// the window viewport width. Initialized to 4 as a safe default.
    items_per_row: usize,
}

impl CatalogView {
    /// Creates a new `CatalogView` connected to the given controller and settings.
    pub fn new(
        controller: Entity<LibraryController>,
        settings: Entity<SettingsController>,
    ) -> Self {
        Self {
            controller,
            settings,
            scroll_handle: UniformListScrollHandle::default(),
            items_per_row: 4,
        }
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

        // Update items_per_row estimate for grid layout using the viewport width.
        // Subtract a rough sidebar width (220px) and both side pads.
        let viewport_w = window.viewport_size().width.as_f32();
        let usable_w = (viewport_w - 220.0 - pad_side.as_f32() * 2.0).max(0.0);
        let card_pitch = density.card_min_width + density.card_gap_x.as_f32();
        self.items_per_row = ((usable_w / card_pitch) as usize).max(1);

        let items_per_row = self.items_per_row;
        let scroll_handle = self.scroll_handle.clone();
        let ctrl = self.controller.clone();

        let root = div()
            .flex_1()
            .min_h_0()
            .flex()
            .flex_col()
            .overflow_y_hidden()
            .pt(pad_top)
            .pb(pad_bottom);

        if item_count == 0 {
            return root
                .child(render_empty_state(colors.text_tertiary))
                .into_any_element();
        }

        match (snap.presentation, snap.grouped) {
            // ── List, ungrouped — virtualized ──────────────────────────────
            (CatalogPresentation::List, false) => {
                let c = colors.clone();
                let d = density.clone();
                let s = storage_root.clone();
                root.px(pad_side)
                    .child(render_list_header(&colors))
                    .child(
                        uniform_list("catalog-list", item_count, move |range, _window, cx| {
                            let items = ctrl.read(cx).visible_items_slice(range);
                            items.iter().map(|item| {
                                render_list_row(item, &c, &d, ctrl.clone(), s.clone())
                                    .into_any_element()
                            }).collect()
                        })
                        .track_scroll(&scroll_handle)
                        .flex_1()
                        .min_h_0(),
                    )
                    .into_any_element()
            }

            // ── List, grouped — non-virtualized (group headers break uniform height) ──
            (CatalogPresentation::List, true) => {
                let items = self.controller.read(cx).visible_items();
                let groups = group_by_publisher(items);
                root.px(pad_side)
                    .children(groups.into_iter().map(|g| {
                        let c = colors.clone();
                        let d = density.clone();
                        let e = self.controller.clone();
                        let s = storage_root.clone();
                        div()
                            .child(render_group_header(&g.publisher, g.items.len(), &c))
                            .child(render_list_header(&c))
                            .children(g.items.into_iter().map(move |item| {
                                render_list_row(&item, &c, &d, e.clone(), s.clone())
                            }))
                    }))
                    .into_any_element()
            }

            // ── Thumbs, ungrouped — virtualized ───────────────────────────
            (CatalogPresentation::Thumbs, false) => {
                let c = colors.clone();
                let d = density.clone();
                root.px(pad_side)
                    .child(
                        uniform_list("catalog-thumbs", item_count, move |range, _window, cx| {
                            let items = ctrl.read(cx).visible_items_slice(range);
                            items.iter().map(|item| {
                                render_thumb_row(item, &c, &d, ctrl.clone())
                                    .into_any_element()
                            }).collect()
                        })
                        .track_scroll(&scroll_handle)
                        .flex_1()
                        .min_h_0(),
                    )
                    .into_any_element()
            }

            // ── Thumbs, grouped — non-virtualized ─────────────────────────
            (CatalogPresentation::Thumbs, true) => {
                let items = self.controller.read(cx).visible_items();
                let groups = group_by_publisher(items);
                root.px(pad_side)
                    .children(groups.into_iter().map(|g| {
                        let c = colors.clone();
                        let d = density.clone();
                        let e = self.controller.clone();
                        div()
                            .child(render_group_header(&g.publisher, g.items.len(), &c))
                            .children(g.items.into_iter().map(move |item| {
                                render_thumb_row(&item, &c, &d, e.clone())
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
                            row_range.map(|row| {
                                let offset = (row - range_start) * items_per_row;
                                let row_end = (offset + items_per_row).min(items.len());
                                let row_items = &items[offset..row_end];
                                div()
                                    .flex()
                                    .gap(d.card_gap_x)
                                    .mb(d.card_gap_y)
                                    .children(row_items.iter().map(|item| {
                                        render_grid_card(
                                            item, &c, d.card_min_width,
                                            ctrl.clone(), s.clone(),
                                        )
                                    }))
                                    .into_any_element()
                            }).collect()
                        })
                        .track_scroll(&scroll_handle)
                        .flex_1()
                        .min_h_0(),
                    )
                    .into_any_element()
            }

            // ── Grid, grouped — non-virtualized ───────────────────────────
            (CatalogPresentation::Grid, true) => {
                let items = self.controller.read(cx).visible_items();
                let groups = group_by_publisher(items);
                root.px(pad_side)
                    .children(groups.into_iter().map(|g| {
                        let c = colors.clone();
                        let d = density.clone();
                        let e = self.controller.clone();
                        let s = storage_root.clone();
                        div()
                            .child(render_group_header(&g.publisher, g.items.len(), &c))
                            .child(render_grid(g.items, c, d, e, s))
                    }))
                    .into_any_element()
            }
        }
    }
}

// ── Empty state ───────────────────────────────────────────────────────────────

fn render_empty_state(text_color: gpui::Hsla) -> impl IntoElement + 'static {
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
                .child("No titles match."),
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

// ── Reveal action ─────────────────────────────────────────────────────────────

fn platform_reveal_label() -> &'static str {
    #[cfg(target_os = "macos")]
    return "Show in Finder";
    #[cfg(target_os = "windows")]
    return "Show in Explorer";
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return "Show in Files";
}

// ── List layout ───────────────────────────────────────────────────────────────

fn render_list_header(colors: &ColorTokens) -> impl IntoElement + 'static + use<> {
    let border = colors.border;
    let text_tertiary = colors.text_tertiary;
    div()
        .flex()
        .items_center()
        .h(px(28.0))
        .border_b_1()
        .border_color(border)
        .text_xs()
        .text_color(text_tertiary)
        .child(div().flex_1().child("Title / Kind"))
        .child(div().w(px(130.0)).child("Publisher"))
        .child(div().w(px(110.0)).child("System"))
        .child(div().w(px(60.0)).child("Pages"))
        .child(div().w(px(60.0)).child("Size"))
        .child(div().w(px(80.0)).child("Added"))
        .child(div().w(px(24.0)).child(""))
        .child(div().w(px(28.0)).child(""))
}

fn render_list_row(
    item: &LibraryItem,
    colors: &ColorTokens,
    density: &DensityConstants,
    entity: Entity<LibraryController>,
    storage_root_path: PathBuf,
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

    let reveal_col: AnyElement = if status == ItemStatus::Downloaded {
        let item_reveal_path = storage_root_path.join("items").join(&*reveal_item_id);
        let reveal_elem_id: Arc<str> = Arc::from(format!("reveal-row-{}", &*reveal_item_id));
        div()
            .id(reveal_elem_id)
            .w(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .on_click(move |_, _, _cx| {
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
        div().w(px(28.0)).into_any_element()
    };

    div()
        .id(Arc::clone(&id))
        .flex()
        .items_center()
        .h(h)
        .border_b_1()
        .border_color(colors.border)
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            entity.update(cx, |ctrl, cx| ctrl.select_item(Arc::clone(&id), cx));
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
                        .text_xs()
                        .text_color(colors.text_tertiary)
                        .whitespace_nowrap()
                        .child(kind),
                ),
        )
        .child(
            div()
                .w(px(130.0))
                .text_sm()
                .text_color(colors.text_secondary)
                .truncate()
                .child(publisher),
        )
        .child(
            div()
                .w(px(110.0))
                .text_sm()
                .text_color(colors.text_secondary)
                .truncate()
                .child(line),
        )
        .child(
            div()
                .w(px(60.0))
                .text_sm()
                .text_color(colors.text_secondary)
                .child(pages.to_string()),
        )
        .child(
            div()
                .w(px(60.0))
                .text_sm()
                .text_color(colors.text_secondary)
                .child(format!("{size_mb:.0} MB")),
        )
        .child(
            div()
                .w(px(80.0))
                .text_sm()
                .text_color(colors.text_secondary)
                .child(year.to_string()),
        )
        .child(render_status(status, &colors))
        .child(reveal_col)
}

// ── Thumbs layout ─────────────────────────────────────────────────────────────

fn render_thumb_row(
    item: &LibraryItem,
    colors: &ColorTokens,
    density: &DensityConstants,
    entity: Entity<LibraryController>,
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
    let row_h = density.row_text_height + px(6.0);
    let colors = colors.clone();

    let cover = render_generative_cover(item, thumb_w, thumb_h);

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
        .child(
            div()
                .w(px(thumb_w))
                .h(px(thumb_h))
                .rounded(px(3.0))
                .overflow_hidden()
                .flex_none()
                .child(cover),
        )
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
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors.text_tertiary)
                                .child(kind),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors.text_tertiary)
                                .child(format!("{pages} pp · {size_mb:.0} MB · {year}")),
                        ),
                ),
        )
        .child(render_status(status, &colors))
}

// ── Grid layout ───────────────────────────────────────────────────────────────

fn render_grid(
    items: Vec<LibraryItem>,
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
            render_grid_card(&item, &colors, min_w, entity.clone(), storage_root_path.clone())
        }))
}

fn render_grid_card(
    item: &LibraryItem,
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

    let cover = render_generative_cover(item, card_w, cover_h);
    let colors = colors.clone();

    let reveal_row: AnyElement = if status == ItemStatus::Downloaded {
        let item_reveal_path = storage_root_path.join("items").join(&*reveal_item_id);
        let reveal_elem_id: Arc<str> = Arc::from(format!("reveal-grid-{}", &*reveal_item_id));
        div()
            .id(reveal_elem_id)
            .mt(px(2.0))
            .cursor_pointer()
            .on_click(move |_, _, _cx| {
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
        .child(div().w(px(card_w)).h(px(cover_h)).child(cover))
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
                .child(reveal_row),
        )
}
