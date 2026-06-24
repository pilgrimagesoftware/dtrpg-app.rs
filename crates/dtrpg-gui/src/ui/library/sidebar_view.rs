//! Sidebar view: wordmark, smart nav, publisher nav, storage footer.

use std::sync::Arc;

use gpui::{div, px, Entity, IntoElement, ParentElement, Styled};

use crate::ui::library::{
    data::{format_total_size, PublisherEntry, SectionCounts, SidebarFilter},
    state::LibraryController,
    theme::{ColorTokens, LibriTheme},
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
) -> impl IntoElement {
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
                        .gap(px(8.0))
                        .child(div().size(px(12.0)).rounded_full().bg(gpui::rgb(0xFF5F57)))
                        .child(div().size(px(12.0)).rounded_full().bg(gpui::rgb(0xFEBC2E)))
                        .child(div().size(px(12.0)).rounded_full().bg(gpui::rgb(0x28C840))),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .size(px(13.0))
                                .bg(accent)
                                .rotate(gpui::degrees(45.0))
                                .rounded(px(2.0)),
                        )
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
                .overflow_y_auto()
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
                            {
                                let e = entity.clone();
                                move |cx| {
                                    e.update(cx, |ctrl, cx| {
                                        ctrl.set_filter(SidebarFilter::AllTitles, cx);
                                    });
                                }
                            },
                            accent, accent_soft, hover, text_primary, text_secondary, text_tertiary,
                        ))
                        .child(render_nav_row(
                            "Recently Added",
                            Some(counts.recently_added),
                            active_filter == SidebarFilter::RecentlyAdded,
                            {
                                let e = entity.clone();
                                move |cx| {
                                    e.update(cx, |ctrl, cx| {
                                        ctrl.set_filter(SidebarFilter::RecentlyAdded, cx);
                                    });
                                }
                            },
                            accent, accent_soft, hover, text_primary, text_secondary, text_tertiary,
                        ))
                        .child(render_nav_row(
                            "On This Device",
                            Some(counts.on_device),
                            active_filter == SidebarFilter::OnDevice,
                            {
                                let e = entity.clone();
                                move |cx| {
                                    e.update(cx, |ctrl, cx| {
                                        ctrl.set_filter(SidebarFilter::OnDevice, cx);
                                    });
                                }
                            },
                            accent, accent_soft, hover, text_primary, text_secondary, text_tertiary,
                        ))
                        .child(render_nav_row(
                            "In the Cloud",
                            Some(counts.in_cloud),
                            active_filter == SidebarFilter::InCloud,
                            {
                                let e = entity.clone();
                                move |cx| {
                                    e.update(cx, |ctrl, cx| {
                                        ctrl.set_filter(SidebarFilter::InCloud, cx);
                                    });
                                }
                            },
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
                            let e = entity.clone();
                            let name = Arc::clone(&entry.name);
                            let count = entry.count;
                            render_nav_row(
                                entry.name.as_ref(),
                                Some(count),
                                is_active,
                                move |cx| {
                                    let n = Arc::clone(&name);
                                    e.update(cx, |ctrl, cx| {
                                        ctrl.set_filter(SidebarFilter::Publisher(n), cx);
                                    });
                                },
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
    on_click: impl Fn(&mut gpui::WindowContext) + 'static,
    accent: gpui::Hsla,
    accent_soft: gpui::Hsla,
    hover_color: gpui::Hsla,
    text_primary: gpui::Hsla,
    text_secondary: gpui::Hsla,
    text_tertiary: gpui::Hsla,
) -> impl IntoElement {
    let label_color = if is_active { accent } else { text_secondary };
    let count_color = if is_active { accent } else { text_tertiary };
    let bg = if is_active {
        accent_soft
    } else {
        gpui::hsla(0.0, 0.0, 0.0, 0.0)
    };
    let label = label.to_string();

    div()
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
        .on_click(move |_, cx| on_click(cx))
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
