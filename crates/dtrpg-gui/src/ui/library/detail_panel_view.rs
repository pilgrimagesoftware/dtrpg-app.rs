//! Detail panel: slide-over showing full item metadata and actions.

use std::sync::Arc;

use gpui::{div, px, Entity, IntoElement, ParentElement, Styled};

use crate::ui::library::{
    cover::render_generative_cover,
    data::{ItemStatus, LibraryItem},
    state::LibraryController,
    theme::ColorTokens,
};

/// Renders the detail panel overlay if `selected_item` is `Some`; otherwise an empty div.
pub fn render_detail_panel(
    selected_item: Option<&LibraryItem>,
    entity: Entity<LibraryController>,
    colors: &ColorTokens,
) -> impl IntoElement {
    let Some(item) = selected_item else {
        return div().into_any();
    };

    let surface = colors.surface;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let accent = colors.accent;
    let accent_on = colors.accent_on;
    let hover = colors.hover;

    let item = item.clone();
    let entity_close = entity.clone();
    let entity_download = entity.clone();
    let item_id = Arc::clone(&item.id);
    let is_downloaded = item.status == ItemStatus::Downloaded;

    div()
        .absolute()
        .right_0()
        .top_0()
        .bottom_0()
        .w(px(320.0))
        .bg(surface)
        .border_l_1()
        .border_color(border)
        .flex()
        .flex_col()
        .overflow_hidden()
        // Close button
        .child(
            div()
                .absolute()
                .top(px(12.0))
                .right(px(12.0))
                .size(px(24.0))
                .rounded_full()
                .bg(hover)
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .text_sm()
                .text_color(text_secondary)
                .on_click(move |_, cx| {
                    entity_close.update(cx, |ctrl, cx| ctrl.clear_selection(cx));
                })
                .child("✕"),
        )
        // Cover
        .child({
            let cover_w = 320.0_f32;
            let cover_h = cover_w * 10.0 / 7.0;
            div()
                .w(px(cover_w))
                .h(px(cover_h))
                .flex_none()
                .child(render_generative_cover(&item, cover_w, cover_h))
        })
        // Scrollable body
        .child(
            div()
                .flex_1()
                .min_h_0()
                .overflow_y_auto()
                .p(px(20.0))
                .flex()
                .flex_col()
                .gap(px(16.0))
                // Publisher + title + line
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_tertiary)
                                .child(item.publisher.to_string()),
                        )
                        .child(
                            div()
                                .text_xl()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(text_primary)
                                .child(item.title.to_string()),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(text_secondary)
                                .child(item.line.to_string()),
                        ),
                )
                // Description
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child(item.desc.to_string()),
                )
                // Actions
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            div()
                                .h(px(36.0))
                                .px(px(16.0))
                                .rounded(px(8.0))
                                .bg(accent)
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor_pointer()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(gpui::FontWeight::MEDIUM)
                                        .text_color(accent_on)
                                        .child("Read"),
                                ),
                        )
                        .child(
                            div()
                                .h(px(36.0))
                                .px(px(16.0))
                                .rounded(px(8.0))
                                .border_1()
                                .border_color(border)
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor_pointer()
                                .on_click(move |_, cx| {
                                    let id = Arc::clone(&item_id);
                                    entity_download.update(cx, |ctrl, cx| {
                                        ctrl.toggle_download(&id, cx);
                                    });
                                })
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(text_primary)
                                        .child(if is_downloaded { "Downloaded" } else { "Download" }),
                                ),
                        ),
                )
                // Metadata table
                .child(render_metadata_table(&item, colors)),
        )
        .into_any()
}

fn render_metadata_table(item: &LibraryItem, colors: &ColorTokens) -> impl IntoElement {
    let rows: &[(&str, String)] = &[
        ("System",    item.line.to_string()),
        ("Category",  item.kind.to_string()),
        ("Format",    item.format.to_string()),
        ("Pages",     item.pages.to_string()),
        ("File size", format!("{:.0} MB", item.size_mb)),
        ("Released",  item.year.to_string()),
        ("Status", match item.status {
            ItemStatus::Downloaded => "On this device".into(),
            ItemStatus::Cloud => "In the cloud".into(),
        }),
    ];

    let border = colors.border;
    let text_tertiary = colors.text_tertiary;
    let text_secondary = colors.text_secondary;

    let mut table = div()
        .flex()
        .flex_col()
        .border_t_1()
        .border_color(border);

    for (label, value) in rows {
        let label = *label;
        let value = value.clone();
        table = table.child(
            div()
                .flex()
                .justify_between()
                .py(px(8.0))
                .border_b_1()
                .border_color(border)
                .child(div().text_xs().text_color(text_tertiary).child(label))
                .child(div().text_xs().text_color(text_secondary).child(value)),
        );
    }

    table
}
