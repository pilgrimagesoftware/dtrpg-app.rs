//! Catalog view: list, thumbs, and grid layouts with grouping and empty state.

use std::sync::Arc;

use gpui::prelude::*;
use gpui::{div, px, AnyElement, Entity, IntoElement, ParentElement, Styled};

use crate::ui::library::{
    cover::render_generative_cover,
};
use crate::data::data::{group_by_publisher, CatalogPresentation, ItemStatus, LibraryItem};
use crate::data::state::LibraryController;
use crate::data::theme::{ColorTokens, DensityConstants};

// ── Public entry point ────────────────────────────────────────────────────────

/// Renders the catalog area in the active presentation mode.
pub fn render_catalog(
    items: Vec<LibraryItem>,
    presentation: CatalogPresentation,
    grouped: bool,
    entity: Entity<LibraryController>,
    colors: &ColorTokens,
    density: &DensityConstants,
) -> AnyElement {
    let pad_top = density.catalog_pad_top;
    let pad_side = density.catalog_pad_side;
    let pad_bottom = density.catalog_pad_bottom;

    let root = div()
        .flex_1()
        .min_h_0()
        .overflow_y_hidden()
        .pt(pad_top)
        .pb(pad_bottom);

    if items.is_empty() {
        return root
            .child(render_empty_state(colors.text_tertiary))
            .into_any_element();
    }

    let colors = colors.clone();
    let density = density.clone();

    match (presentation, grouped) {
        (CatalogPresentation::List, false) => root
            .px(pad_side)
            .child(render_list_header(&colors))
            .children(items.iter().map(|item| {
                render_list_row(item, &colors, &density, entity.clone())
            }))
            .into_any_element(),

        (CatalogPresentation::List, true) => {
            let groups = group_by_publisher(items);
            root.px(pad_side)
                .children(groups.into_iter().map(|g| {
                    let c = colors.clone();
                    let d = density.clone();
                    let e = entity.clone();
                    div()
                        .child(render_group_header(&g.publisher, g.items.len(), &c))
                        .child(render_list_header(&c))
                        .children(g.items.into_iter().map(move |item| {
                            render_list_row(&item, &c, &d, e.clone())
                        }))
                }))
                .into_any_element()
        }

        (CatalogPresentation::Thumbs, false) => root
            .px(pad_side)
            .children(items.iter().map(|item| {
                render_thumb_row(item, &colors, &density, entity.clone())
            }))
            .into_any_element(),

        (CatalogPresentation::Thumbs, true) => {
            let groups = group_by_publisher(items);
            root.px(pad_side)
                .children(groups.into_iter().map(|g| {
                    let c = colors.clone();
                    let d = density.clone();
                    let e = entity.clone();
                    div()
                        .child(render_group_header(&g.publisher, g.items.len(), &c))
                        .children(g.items.into_iter().map(move |item| {
                            render_thumb_row(&item, &c, &d, e.clone())
                        }))
                }))
                .into_any_element()
        }

        (CatalogPresentation::Grid, false) => root
            .px(pad_side)
            .child(render_grid(items, colors, density, entity))
            .into_any_element(),

        (CatalogPresentation::Grid, true) => {
            let groups = group_by_publisher(items);
            root.px(pad_side)
                .children(groups.into_iter().map(|g| {
                    let c = colors.clone();
                    let d = density.clone();
                    let e = entity.clone();
                    div()
                        .child(render_group_header(&g.publisher, g.items.len(), &c))
                        .child(render_grid(g.items, c, d, e))
                }))
                .into_any_element()
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
}

fn render_list_row(
    item: &LibraryItem,
    colors: &ColorTokens,
    density: &DensityConstants,
    entity: Entity<LibraryController>,
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
            render_grid_card(&item, &colors, min_w, entity.clone())
        }))
}

fn render_grid_card(
    item: &LibraryItem,
    colors: &ColorTokens,
    card_w: f32,
    entity: Entity<LibraryController>,
) -> impl IntoElement + 'static + use<> {
    let id = Arc::clone(&item.id);
    let title = item.title.to_string();
    let publisher = item.publisher.to_string();
    let status = item.status;
    let cover_h = card_w * 10.0 / 7.0;

    let cover = render_generative_cover(item, card_w, cover_h);
    let colors = colors.clone();

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
                ),
        )
}
