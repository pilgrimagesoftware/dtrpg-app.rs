//! Toolbar view: section title, search, sort dropdown, group toggle, layout switcher.

use gpui::prelude::*;
use gpui::{div, px, Entity, IntoElement, ParentElement, Styled};

use crate::data::{
    data::{CatalogPresentation, SidebarFilter, SortMethod},
    state::LibraryController,
    theme::ColorTokens,
};

fn section_title_for(filter: &SidebarFilter) -> &str {
    match filter {
        SidebarFilter::AllTitles => "All Titles",
        SidebarFilter::RecentlyAdded => "Recently Added",
        SidebarFilter::OnDevice => "On This Device",
        SidebarFilter::InCloud => "In the Cloud",
        SidebarFilter::Publisher(_) => "Publisher",
    }
}

/// Renders the toolbar row above the catalog.
#[allow(clippy::too_many_arguments)]
pub fn render_toolbar(
    filter: &SidebarFilter,
    matched_count: usize,
    search_query: &str,
    sort: SortMethod,
    grouped: bool,
    presentation: CatalogPresentation,
    entity: Entity<LibraryController>,
    colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let surface = colors.surface;
    let border = colors.border;
    let border_strong = colors.border_strong;
    let text_primary = colors.text_primary;
    let text_tertiary = colors.text_tertiary;
    let bg = colors.surface_alt;
    let accent = colors.accent;
    let accent_soft = colors.accent_soft;

    let title = section_title_for(filter).to_string();
    let search_query = search_query.to_string();

    div()
        .h(px(53.0))
        .flex_none()
        .flex()
        .items_center()
        .gap(px(16.0))
        .px(px(18.0))
        .border_b_1()
        .border_color(border)
        .bg(surface)
        // ── Title + count ─────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .items_baseline()
                .gap(px(11.0))
                .min_w_0()
                .child(
                    div()
                        .text_color(text_primary)
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_lg()
                        .truncate()
                        .child(title),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(text_tertiary)
                        .whitespace_nowrap()
                        .child(matched_count.to_string()),
                ),
        )
        // ── Spacer ────────────────────────────────────────────────────────
        .child(div().flex_1())
        // ── Controls ──────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(render_search(
                    search_query,
                    entity.clone(),
                    bg,
                    border_strong,
                    text_primary,
                    text_tertiary,
                ))
                .child(render_sort_selector(
                    sort,
                    entity.clone(),
                    bg,
                    border_strong,
                    text_primary,
                    text_tertiary,
                ))
                .child(render_group_toggle(
                    grouped,
                    entity.clone(),
                    bg,
                    border_strong,
                    text_primary,
                    accent,
                    accent_soft,
                ))
                .child(render_layout_switcher(
                    presentation,
                    entity,
                    bg,
                    border_strong,
                    text_primary,
                    accent,
                    accent_soft,
                )),
        )
}

// ── Search ────────────────────────────────────────────────────────────────────

fn render_search(
    query: String,
    entity: Entity<LibraryController>,
    bg: gpui::Hsla,
    border: gpui::Hsla,
    text_primary: gpui::Hsla,
    text_tertiary: gpui::Hsla,
) -> impl IntoElement + 'static {
    let has_query = !query.is_empty();
    let entity_clear = entity.clone();

    div()
        .flex()
        .items_center()
        .gap(px(7.0))
        .h(px(30.0))
        .px(px(9.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .bg(bg)
        .w(px(188.0))
        .child(div().text_xs().text_color(text_tertiary).child("⌕"))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .text_color(if has_query { text_primary } else { text_tertiary })
                .truncate()
                .child(if has_query { query } else { "Search…".into() }),
        )
        .when(has_query, |el| {
            el.child(
                div()
                    .id("search-clear")
                    .text_xs()
                    .text_color(text_tertiary)
                    .cursor_pointer()
                    .on_click(move |_, _, cx| {
                        entity_clear.update(cx, |ctrl, cx| {
                            ctrl.clear_search_query(cx);
                        });
                    })
                    .child("✕"),
            )
        })
}

// ── Sort selector ─────────────────────────────────────────────────────────────

fn render_sort_selector(
    current: SortMethod,
    entity: Entity<LibraryController>,
    bg: gpui::Hsla,
    border: gpui::Hsla,
    text_primary: gpui::Hsla,
    text_tertiary: gpui::Hsla,
) -> impl IntoElement + 'static {
    let label = match current {
        SortMethod::Title => "Title",
        SortMethod::Publisher => "Publisher",
        SortMethod::DateAdded => "Date Added",
        SortMethod::PageCount => "Pages",
    };
    let next_sort = match current {
        SortMethod::Title => SortMethod::Publisher,
        SortMethod::Publisher => SortMethod::DateAdded,
        SortMethod::DateAdded => SortMethod::PageCount,
        SortMethod::PageCount => SortMethod::Title,
    };

    div()
        .id("sort-selector")
        .h(px(30.0))
        .px(px(11.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .bg(bg)
        .flex()
        .items_center()
        .gap(px(4.0))
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            entity.update(cx, |ctrl, cx| ctrl.set_sort(next_sort, cx));
        })
        .child(div().text_sm().text_color(text_primary).child(label))
        .child(div().text_xs().text_color(text_tertiary).child("↕"))
}

// ── Group toggle ──────────────────────────────────────────────────────────────

fn render_group_toggle(
    grouped: bool,
    entity: Entity<LibraryController>,
    bg: gpui::Hsla,
    border: gpui::Hsla,
    text_primary: gpui::Hsla,
    accent: gpui::Hsla,
    accent_soft: gpui::Hsla,
) -> impl IntoElement + 'static {
    let btn_bg = if grouped { accent_soft } else { bg };
    let text_color = if grouped { accent } else { text_primary };

    div()
        .id("group-toggle")
        .h(px(30.0))
        .px(px(11.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .bg(btn_bg)
        .flex()
        .items_center()
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            entity.update(cx, |ctrl, cx| ctrl.set_grouped(!grouped, cx));
        })
        .child(div().text_sm().text_color(text_color).child("Group"))
}

// ── Layout switcher ───────────────────────────────────────────────────────────

fn render_layout_switcher(
    current: CatalogPresentation,
    entity: Entity<LibraryController>,
    bg: gpui::Hsla,
    border: gpui::Hsla,
    text_primary: gpui::Hsla,
    accent: gpui::Hsla,
    accent_soft: gpui::Hsla,
) -> impl IntoElement + 'static {
    let modes = [
        (CatalogPresentation::List, "layout-list"),
        (CatalogPresentation::Thumbs, "layout-thumbs"),
        (CatalogPresentation::Grid, "layout-grid"),
    ];
    let labels = ["List", "Thumbs", "Grid"];

    let mut row = div()
        .flex()
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .bg(bg)
        .overflow_hidden();

    for ((mode, id_str), label) in modes.into_iter().zip(labels.into_iter()) {
        let is_active = current == mode;
        let btn_bg = if is_active {
            accent_soft
        } else {
            gpui::hsla(0.0, 0.0, 0.0, 0.0)
        };
        let text = if is_active { accent } else { text_primary };
        let e = entity.clone();

        row = row.child(
            div()
                .id(id_str)
                .h(px(28.0))
                .px(px(10.0))
                .bg(btn_bg)
                .flex()
                .items_center()
                .cursor_pointer()
                .on_click(move |_, _, cx| {
                    e.update(cx, |ctrl, cx| ctrl.set_presentation(mode, cx));
                })
                .child(div().text_sm().text_color(text).child(label)),
        );
    }

    row
}
