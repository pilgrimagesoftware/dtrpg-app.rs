//! Activity panel overlay: lists in-progress and recently-completed background operations.

use gpui::prelude::*;
use gpui::{div, px, AnyElement, Entity, IntoElement, ParentElement, Styled};
use gpui_component::scroll::ScrollableElement;
use gpui_component::tooltip::Tooltip;

use crate::controllers::activity::ActivityController;
use crate::data::activity::{ActivitySnapshot, ActivityStatus};
use crate::data::theme::ColorTokens;

/// Renders the activity panel overlay anchored at the bottom of the sidebar column.
pub fn render_activity_panel(
    snap: &ActivitySnapshot,
    entity: Entity<ActivityController>,
    colors: &ColorTokens,
) -> AnyElement {
    let surface_alt = colors.surface_alt;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let accent = colors.accent;

    let close_entity = entity.clone();
    let selected_id = snap.selected_id;

    div()
        .absolute()
        .bottom(px(44.0))
        .left_0()
        .w(px(250.0))
        .bg(surface_alt)
        .border_1()
        .border_color(border)
        .rounded_t(px(8.0))
        .flex()
        .flex_col()
        // ── Header ────────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .px(px(14.0))
                .py(px(8.0))
                .border_b_1()
                .border_color(border)
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child("Activity"),
                )
                .child(
                    div()
                        .id("activity-close")
                        .text_xs()
                        .text_color(text_tertiary)
                        .cursor_pointer()
                        .child("✕")
                        .on_click(move |_, _, cx| {
                            close_entity.update(cx, |a, cx| a.toggle_panel(cx));
                        }),
                ),
        )
        // ── Item list ─────────────────────────────────────────────────────
        .child(if snap.items.is_empty() {
            render_empty(text_primary, text_tertiary)
        } else {
            div()
                .flex()
                .flex_col()
                .max_h(px(300.0))
                .overflow_y_scrollbar()
                .children(snap.items.iter().map(|item| {
                    render_item_row(
                        item.id,
                        item.label.as_ref(),
                        &item.status,
                        selected_id,
                        entity.clone(),
                        text_secondary,
                        text_tertiary,
                        accent,
                    )
                }))
                .into_any_element()
        })
        .into_any_element()
}

fn render_empty(text_primary: gpui::Hsla, text_tertiary: gpui::Hsla) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .py(px(24.0))
        .gap(px(6.0))
        .child(
            div()
                .text_2xl()
                .text_color(text_tertiary)
                .child("○"),
        )
        .child(
            div()
                .text_sm()
                .text_color(text_primary)
                .child("No recent activity."),
        )
        .child(
            div()
                .text_xs()
                .text_color(text_tertiary)
                .child("Activity will appear here as operations run."),
        )
        .into_any_element()
}

#[allow(clippy::too_many_arguments)]
fn render_item_row(
    item_id: u64,
    label: &str,
    status: &ActivityStatus,
    selected_id: Option<u64>,
    activity_entity: Entity<ActivityController>,
    text_color: gpui::Hsla,
    text_tertiary: gpui::Hsla,
    accent: gpui::Hsla,
) -> impl IntoElement + 'static + use<> {
    let (icon, icon_color) = match status {
        ActivityStatus::InProgress => ("↻", accent),
        ActivityStatus::Complete => ("✓", text_color),
        ActivityStatus::Error(_) => ("⚠", text_tertiary),
    };
    let error_msg = if let ActivityStatus::Error(msg) = status {
        Some(msg.clone())
    } else {
        None
    };
    let label = label.to_string();
    let is_expanded = selected_id == Some(item_id);

    // Tooltip shows the full label, plus the error message on a second line for error items.
    let tooltip_text = match &error_msg {
        Some(err) => format!("{label}\n{err}"),
        None => label.clone(),
    };

    div()
        .id(format!("activity-row-{item_id}"))
        .flex()
        .items_start()
        .gap(px(8.0))
        .px(px(14.0))
        .py(px(6.0))
        .cursor_pointer()
        .tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
        .on_click(move |_, _, cx| {
            activity_entity.update(cx, |a, cx| a.select_activity(item_id, cx));
        })
        .child(
            div()
                .flex_none()
                .text_xs()
                .text_color(icon_color)
                .child(icon),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .min_w_0()
                .child(if is_expanded {
                    div()
                        .text_xs()
                        .text_color(text_color)
                        .child(label)
                        .into_any_element()
                } else {
                    div()
                        .text_xs()
                        .text_color(text_color)
                        .truncate()
                        .child(label)
                        .into_any_element()
                })
                .children(error_msg.map(|msg| {
                    if is_expanded {
                        div()
                            .text_xs()
                            .text_color(text_tertiary)
                            .child(msg)
                    } else {
                        div()
                            .text_xs()
                            .text_color(text_tertiary)
                            .truncate()
                            .child(msg)
                    }
                })),
        )
}
