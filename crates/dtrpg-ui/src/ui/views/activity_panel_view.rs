//! Activity panel overlay: lists in-progress and recently-completed background operations.

use gpui::prelude::*;
use gpui::{div, px, AnyElement, Entity, IntoElement, ParentElement, Styled};

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
            render_empty(text_tertiary)
        } else {
            div()
                .flex()
                .flex_col()
                .max_h(px(300.0))
                .overflow_y_hidden()
                .children(snap.items.iter().map(|item| {
                    render_item_row(item.label.as_ref(), &item.status, text_secondary, text_tertiary, accent)
                }))
                .into_any_element()
        })
        .into_any_element()
}

fn render_empty(text_color: gpui::Hsla) -> AnyElement {
    div()
        .px(px(14.0))
        .py(px(12.0))
        .text_xs()
        .text_color(text_color)
        .child("No recent activity.")
        .into_any_element()
}

fn render_item_row(
    label: &str,
    status: &ActivityStatus,
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

    div()
        .flex()
        .items_start()
        .gap(px(8.0))
        .px(px(14.0))
        .py(px(6.0))
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
                .child(
                    div()
                        .text_xs()
                        .text_color(text_color)
                        .truncate()
                        .child(label),
                )
                .children(error_msg.map(|msg| {
                    div()
                        .text_xs()
                        .text_color(text_tertiary)
                        .truncate()
                        .child(msg)
                })),
        )
}
