//! Sidebar view: wordmark, smart nav, publisher nav, storage footer.

use std::sync::Arc;

use gpui::prelude::*;
use gpui::{div, px, Entity, IntoElement, ParentElement, Styled};

use crate::ui::library::{
    data::{PublisherEntry, SectionCounts, SidebarFilter},
    state::LibraryController,
    theme::ColorTokens,
};

/// Renders the full sidebar column.
pub fn render_sidebar(
    filter: &SidebarFilter,
    counts: SectionCounts,
    publishers: &[PublisherEntry],
    total_count: usize,
    total_mb: f64,
    entity: Entity<LibraryController>,
    colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let surface_alt = colors.surface_alt;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let accent = colors.accent;
    let accent_soft = colors.accent_soft;
    let hover = colors.hover;

    let active_filter = filter.clone();
    let total_size_str = if total_mb >= 1024.0 {
        format!("{:.1} GB", total_mb / 1024.0)
    } else {
        format!("{:.0} MB", total_mb)
    };

    div()
        .w(px(250.0))
        .flex_none()
        .bg(surface_alt)
        .border_r_1()
        .border_color(border)
        .flex()
        .flex_col()
        .h_full()
        // ── Header (wordmark) ──────────────────────────────────────────────
        .child(
            div()
                .h(px(53.0))
                .flex_none()
                .flex()
                .items_center()
                .gap(px(16.0))
                .px(px(16.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        // Logo mark: filled square in accent color (no CSS rotate available)
                        // .child(
                        //     div()
                        //         .size(px(13.0))
                        //         .bg(accent)
                        //         .rounded(px(2.0)),
                        // )
                        .child(
                            div()
                                .text_color(text_primary)
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_lg()
                                .child("Libri"),
                        ),
                ),
        )
        // ── Scrollable body ────────────────────────────────────────────────
        .child(
            div()
                .flex_1()
                .min_h_0()
                .overflow_y_hidden()
                .py(px(6.0))
                // Library smart section
                .child(
                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .child(render_nav_row(
                            "All Titles",
                            Some(counts.all),
                            active_filter == SidebarFilter::AllTitles,
                            SidebarFilter::AllTitles,
                            entity.clone(),
                            accent, accent_soft, hover, text_primary, text_secondary, text_tertiary,
                        ))
                        .child(render_nav_row(
                            "Recently Added",
                            Some(counts.recently_added),
                            active_filter == SidebarFilter::RecentlyAdded,
                            SidebarFilter::RecentlyAdded,
                            entity.clone(),
                            accent, accent_soft, hover, text_primary, text_secondary, text_tertiary,
                        ))
                        .child(render_nav_row(
                            "On This Device",
                            Some(counts.on_device),
                            active_filter == SidebarFilter::OnDevice,
                            SidebarFilter::OnDevice,
                            entity.clone(),
                            accent, accent_soft, hover, text_primary, text_secondary, text_tertiary,
                        ))
                        .child(render_nav_row(
                            "In the Cloud",
                            Some(counts.in_cloud),
                            active_filter == SidebarFilter::InCloud,
                            SidebarFilter::InCloud,
                            entity.clone(),
                            accent, accent_soft, hover, text_primary, text_secondary, text_tertiary,
                        )),
                )
                // Publishers section
                .child(
                    div()
                        .border_t_1()
                        .border_color(border)
                        .mt(px(2.0))
                        .pt(px(8.0))
                        .child(
                            div()
                                .px(px(20.0))
                                .pb(px(7.0))
                                .text_xs()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(text_tertiary)
                                .child("PUBLISHERS"),
                        )
                        .children(publishers.iter().map(|entry| {
                            let is_active = active_filter
                                == SidebarFilter::Publisher(Arc::clone(&entry.name));
                            let filter = SidebarFilter::Publisher(Arc::clone(&entry.name));
                            render_nav_row(
                                entry.name.as_ref(),
                                Some(entry.count),
                                is_active,
                                filter,
                                entity.clone(),
                                accent, accent_soft, hover, text_primary, text_secondary, text_tertiary,
                            )
                        })),
                ),
        )
        // ── Footer ────────────────────────────────────────────────────────
        .child(
            div()
                .flex_none()
                .border_t_1()
                .border_color(border)
                .px(px(18.0))
                .py(px(11.0))
                .flex()
                .justify_between()
                .text_xs()
                .text_color(text_tertiary)
                .child(format!("{total_count} titles"))
                .child(total_size_str),
        )
}

// ── Nav row ───────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn render_nav_row(
    label: &str,
    count: Option<usize>,
    is_active: bool,
    filter: SidebarFilter,
    entity: Entity<LibraryController>,
    accent: gpui::Hsla,
    accent_soft: gpui::Hsla,
    _hover_color: gpui::Hsla,
    _text_primary: gpui::Hsla,
    text_secondary: gpui::Hsla,
    text_tertiary: gpui::Hsla,
) -> impl IntoElement + 'static + use<> {
    let label_color = if is_active { accent } else { text_secondary };
    let count_color = if is_active { accent } else { text_tertiary };
    let bg = if is_active {
        accent_soft
    } else {
        gpui::hsla(0.0, 0.0, 0.0, 0.0)
    };
    let label_id = label.to_string();
    let label = label.to_string();

    div()
        .id(label_id)
        .flex()
        .items_center()
        .justify_between()
        .gap(px(10.0))
        .w_full()
        .px(px(11.0))
        .py(px(6.0))
        .rounded(px(7.0))
        .bg(bg)
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            let f = filter.clone();
            entity.update(cx, |ctrl, cx| ctrl.set_filter(f, cx));
        })
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .text_sm()
                .text_color(label_color)
                .child(label),
        )
        .children(count.map(|c| {
            div()
                .flex_none()
                .text_xs()
                .text_color(count_color)
                .child(c.to_string())
        }))
}
