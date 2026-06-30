//! Activity panel overlay: lists in-progress and recently-completed background operations.

use std::time::Instant;

use gpui::prelude::*;
use gpui::{AnyElement, Entity, IntoElement, ParentElement, Styled, div, px, relative};
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
        .w(px(340.0))
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
                        .child("x")
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
                .max_h(px(400.0))
                .overflow_y_scrollbar()
                .children(snap.items.iter().map(|item| {
                    let is_in_progress = matches!(item.status, ActivityStatus::InProgress);
                    render_item_row(
                        item.id,
                        item.label.as_ref(),
                        &item.status,
                        selected_id,
                        entity.clone(),
                        item.started_at,
                        item.elapsed_secs,
                        item.progress,
                        is_in_progress && item.cancel_fn.is_some(),
                        text_secondary,
                        text_tertiary,
                        accent,
                        border,
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
        .child(div().text_2xl().text_color(text_tertiary).child("o"))
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

/// Formats a duration as "Xs" for under 60 s or "Xm Ys" for 60 s or more.
fn format_duration(secs: u64) -> String {
    if secs < 60 {
        format!("{secs}s")
    } else {
        let m = secs / 60;
        let s = secs % 60;
        format!("{m}m {s}s")
    }
}

#[allow(clippy::too_many_arguments)]
fn render_item_row(
    item_id: u64,
    label: &str,
    status: &ActivityStatus,
    selected_id: Option<u64>,
    activity_entity: Entity<ActivityController>,
    started_at: Instant,
    elapsed_secs: Option<u64>,
    progress: Option<f32>,
    has_cancel: bool,
    text_color: gpui::Hsla,
    text_tertiary: gpui::Hsla,
    accent: gpui::Hsla,
    border: gpui::Hsla,
) -> impl IntoElement + 'static + use<> {
    let is_in_progress = matches!(status, ActivityStatus::InProgress);

    let (icon, icon_color) = match status {
        ActivityStatus::InProgress => ("~", accent),
        ActivityStatus::Complete => ("+", text_color),
        ActivityStatus::Error(_) => ("!", text_tertiary),
    };
    let error_msg = if let ActivityStatus::Error(msg) = status {
        Some(msg.clone())
    } else {
        None
    };
    let label = label.to_string();
    let is_expanded = selected_id == Some(item_id);

    let elapsed_label = if is_in_progress {
        format_duration(started_at.elapsed().as_secs())
    } else {
        elapsed_secs.map(format_duration).unwrap_or_default()
    };

    // Tooltip shows the full label, plus the error message on a second line for error items.
    let tooltip_text = match &error_msg {
        Some(err) => format!("{label}\n{err}"),
        None => label.clone(),
    };

    let select_entity = activity_entity.clone();
    let cancel_entity = activity_entity;

    div()
        .id(format!("activity-row-{item_id}"))
        .flex()
        .flex_col()
        .px(px(14.0))
        .py(px(6.0))
        .cursor_pointer()
        .tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
        .on_click(move |_, _, cx| {
            select_entity.update(cx, |a, cx| a.select_activity(item_id, cx));
        })
        // ── Header line: icon + label + elapsed + optional cancel ─────────
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
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
                        .flex_1()
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
                                div().text_xs().text_color(text_tertiary).child(msg)
                            } else {
                                div()
                                    .text_xs()
                                    .text_color(text_tertiary)
                                    .truncate()
                                    .child(msg)
                            }
                        })),
                )
                .child(
                    div()
                        .flex_none()
                        .text_xs()
                        .text_color(text_tertiary)
                        .child(elapsed_label),
                )
                .children(has_cancel.then(|| {
                    div()
                        .id(format!("activity-cancel-{item_id}"))
                        .flex_none()
                        .text_xs()
                        .text_color(text_tertiary)
                        .cursor_pointer()
                        .child("x")
                        .on_click(move |_, _, cx| {
                            cancel_entity.update(cx, |a, cx| a.cancel_activity(item_id, cx));
                        })
                })),
        )
        // ── Progress bar (in-progress items only) ─────────────────────────
        .children(is_in_progress.then(|| {
            let fill = progress.unwrap_or(0.3);
            div()
                .w_full()
                .h(px(3.0))
                .mt(px(4.0))
                .bg(border)
                .rounded(px(1.5))
                .child(div().h_full().w(relative(fill)).bg(accent).rounded(px(1.5)))
        }))
}
