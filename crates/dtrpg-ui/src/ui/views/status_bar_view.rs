//! Status bar view: library totals, active-tab summary, theme picker,
//! activity indicator, and notification indicator.
//!
//! Consolidates indicators previously scattered in the sidebar footer
//! (library totals, activity) into a single bottom row, per
//! `main-window-status-bar`. The notification indicator maps onto this
//! crate's existing alert history panel — there is no separate
//! notification-inbox capability in this app yet, so this relocates and
//! reuses that entry point rather than introducing a new one.

use gpui::prelude::*;
use gpui::{Entity, IntoElement, Styled, div};
use gpui_component::IconName;
use gpui_component::badge::Badge;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_component::separator::Separator;
use gpui_component::status_bar::StatusBar;

use crate::controllers::activity::ActivityController;
use crate::controllers::library::LibraryController;
use crate::data::theme::{ColorTokens, ThemeKey};
use rust_i18n::t;

/// Data the status bar needs to render, decoupled from where it's sourced.
pub struct StatusBarSnapshot {
    /// Total number of items in the library.
    pub total_count: usize,
    /// Total size of all library items, in megabytes.
    pub total_mb: f64,
    /// Title of the currently active tab (catalog section name, or an open
    /// detail tab's item title).
    pub active_tab_label: String,
    /// Item count for the active tab's content (catalog: matched item count;
    /// detail tab: always 1).
    pub active_tab_count: usize,
    /// Currently active theme.
    pub theme_key: ThemeKey,
    /// Count of in-progress background operations.
    pub activity_in_progress: usize,
    /// Count of recently completed background operations.
    pub activity_recent_count: usize,
    /// Count of recent error-status operations, used as the notification
    /// indicator's unread badge.
    pub activity_recent_error_count: usize,
}

fn theme_label(key: ThemeKey) -> &'static str {
    match key {
        ThemeKey::Parchment => "Parchment",
        ThemeKey::Slate => "Slate",
        ThemeKey::Sage => "Sage",
        ThemeKey::Ink => "Ink",
    }
}

/// Renders the status bar row below the main content area.
pub fn render_status_bar(
    snap: StatusBarSnapshot,
    entity: Entity<LibraryController>,
    activity: Entity<ActivityController>,
    colors: &ColorTokens,
) -> impl IntoElement + 'static {
    let total_size_str = if snap.total_mb >= 1024.0 {
        format!("{:.1} GB", snap.total_mb / 1024.0)
    } else {
        format!("{:.0} MB", snap.total_mb)
    };

    let library_summary = div()
        .text_xs()
        .text_color(colors.text_secondary)
        .child(format!("{} \u{2022} {total_size_str}", snap.total_count));

    let active_tab_summary = div()
        .text_xs()
        .text_color(colors.text_secondary)
        .child(format!(
            "{} \u{2022} {}",
            snap.active_tab_label, snap.active_tab_count
        ));

    let theme_picker = Button::new("status-bar-theme")
        .ghost()
        .compact()
        .label(theme_label(snap.theme_key))
        .tooltip(
            t!(
                "status_bar.theme_tooltip",
                theme = theme_label(snap.theme_key)
            )
            .to_string(),
        )
        .dropdown_menu(move |menu, _, _| {
            let mut m = menu;
            for key in [
                ThemeKey::Parchment,
                ThemeKey::Slate,
                ThemeKey::Sage,
                ThemeKey::Ink,
            ] {
                let e = entity.clone();
                m = m.item(
                    PopupMenuItem::new(theme_label(key))
                        .checked(key == snap.theme_key)
                        .on_click(move |_, _, cx| {
                            e.update(cx, |ctrl, cx| ctrl.set_theme(key, cx));
                        }),
                );
            }
            m
        });

    let activity_total = snap.activity_in_progress + snap.activity_recent_count;
    let activity_glyph = if snap.activity_in_progress > 0 {
        "\u{21bb}"
    } else if snap.activity_recent_count > 0 {
        "\u{25cf}"
    } else {
        "\u{25cb}"
    };
    let activity_for_click = activity.clone();
    let activity_indicator = Button::new("status-bar-activity")
        .ghost()
        .compact()
        .label(format!("{activity_glyph} {activity_total}"))
        .tooltip(
            t!(
                "status_bar.activity_tooltip",
                in_progress = snap.activity_in_progress,
                completed = snap.activity_recent_count
            )
            .to_string(),
        )
        .on_click(move |_, _, cx| {
            activity_for_click.update(cx, |a, cx| a.toggle_panel(cx));
        });

    let has_errors = snap.activity_recent_error_count > 0;
    let notification_button = Button::new("status-bar-notifications")
        .ghost()
        .compact()
        .icon(IconName::Bell)
        .tooltip(
            t!(
                "status_bar.notifications_tooltip",
                n = snap.activity_recent_error_count
            )
            .to_string(),
        )
        .on_click(move |_, _, cx| {
            activity.update(cx, |a, cx| a.toggle_alert_panel(cx));
        });
    let notification_indicator = if has_errors {
        Badge::new()
            .dot()
            .color(gpui::red())
            .child(notification_button)
            .into_any_element()
    } else {
        notification_button.into_any_element()
    };

    StatusBar::new()
        .left(library_summary)
        .left(Separator::vertical())
        .left(active_tab_summary)
        .right(theme_picker)
        .right(activity_indicator)
        .right(notification_indicator)
}
