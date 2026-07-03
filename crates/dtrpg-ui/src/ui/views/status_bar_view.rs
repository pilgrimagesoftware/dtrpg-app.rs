//! Status bar view: library totals, active-tab summary, theme picker,
//! activity indicator, and notification indicator.
//!
//! Consolidates indicators previously scattered in the sidebar footer
//! (library totals, activity) into a single bottom row, per
//! `main-window-status-bar`. The notification indicator maps onto this
//! crate's existing alert history panel — there is no separate
//! notification-inbox capability in this app yet, so this relocates and
//! reuses that entry point rather than introducing a new one.
//!
//! The activity and alert history panels are rendered as `Popover`s anchored
//! to their trigger buttons here (rather than as root-level, hand-positioned
//! overlays) so they always appear next to the button that opens them,
//! regardless of window width or future status bar layout changes.

use gpui::prelude::*;
use gpui::{Anchor, Entity, IntoElement, Styled, div};
use gpui_component::IconName;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_component::popover::Popover;
use gpui_component::progress::ProgressCircle;
use gpui_component::separator::Separator;
use gpui_component::status_bar::StatusBar;
use gpui_component::{Sizable as _, Size};

use crate::controllers::activity::ActivityController;
use crate::controllers::library::LibraryController;
use crate::data::activity::{ActivitySnapshot, AlertHistorySnapshot};
use crate::data::theme::{ColorTokens, ThemeKey};
use crate::ui::views::activity_panel_view::render_activity_panel;
use crate::ui::views::alert_history_view::render_alert_history_panel;
use crate::util::pluralize::pluralize;
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
}

/// Activity and alert history state needed to render the status bar's
/// activity/notification buttons and their anchored panels.
pub struct ActivityBarData<'a> {
    /// Controller entity, used to toggle the panels and act on their contents.
    pub entity: Entity<ActivityController>,
    /// Activity panel snapshot for the current render pass.
    pub snap: &'a ActivitySnapshot,
    /// Alert history panel snapshot for the current render pass.
    pub alert_snap: &'a AlertHistorySnapshot,
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
    activity_data: ActivityBarData<'_>,
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
        .child(format!(
            "{} \u{2022} {total_size_str}",
            pluralize(snap.total_count, "count.total_item", "count.total_items")
        ));

    let active_tab_summary = div()
        .text_xs()
        .text_color(colors.text_secondary)
        .child(format!(
            "{} \u{2022} {}",
            snap.active_tab_label,
            pluralize(snap.active_tab_count, "count.item", "count.items")
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

    let ActivityBarData {
        entity: activity,
        snap: activity_snap,
        alert_snap,
    } = activity_data;

    let activity_total = activity_snap.in_progress_count + activity_snap.recent_count;
    let activity_glyph = if activity_snap.in_progress_count > 0 {
        "\u{21bb}"
    } else if activity_snap.recent_count > 0 {
        "\u{25cf}"
    } else {
        "\u{25cb}"
    };
    let mut activity_indicator = Button::new("status-bar-activity")
        .ghost()
        .compact()
        .label(format!("{activity_glyph} {activity_total}"))
        .tooltip(
            t!(
                "status_bar.activity_tooltip",
                in_progress = activity_snap.in_progress_count,
                completed = activity_snap.recent_count
            )
            .to_string(),
        );
    if activity_snap.in_progress_count > 0 {
        let progress_circle = match activity_snap.aggregate_progress {
            Some(fraction) => ProgressCircle::new("status-bar-activity-progress")
                .with_size(Size::Small)
                .value(fraction * 100.0),
            None => ProgressCircle::new("status-bar-activity-progress")
                .with_size(Size::Small)
                .loading(true),
        };
        activity_indicator = activity_indicator.child(progress_circle);
    }

    let activity_for_open_change = activity.clone();
    let activity_panel = Popover::new("status-bar-activity-popover")
        .anchor(Anchor::BottomLeft)
        .trigger(activity_indicator)
        .open(activity_snap.panel_open)
        .on_open_change(move |open, _window, cx| {
            activity_for_open_change.update(cx, |a, cx| a.set_panel_open(*open, cx));
        })
        .child(render_activity_panel(
            activity_snap,
            activity.clone(),
            colors,
        ));

    let has_errors = activity_snap.recent_error_count > 0;
    let notification_button = Button::new("status-bar-notifications")
        .ghost()
        .compact()
        .icon(IconName::Bell)
        .tooltip(
            t!(
                "status_bar.notifications_tooltip",
                n = activity_snap.recent_error_count
            )
            .to_string(),
        );

    let alert_for_open_change = activity.clone();
    // `Popover::trigger` requires `Selectable`, which `Badge` doesn't implement, so the
    // unread-error dot is layered on as a sibling of the popover rather than wrapping the
    // trigger in a `Badge`.
    let notification_panel = div()
        .relative()
        .child(
            Popover::new("status-bar-notifications-popover")
                .anchor(Anchor::BottomRight)
                .trigger(notification_button)
                .open(alert_snap.open)
                .on_open_change(move |open, _window, cx| {
                    alert_for_open_change.update(cx, |a, cx| a.set_alert_panel_open(*open, cx));
                })
                .child(render_alert_history_panel(alert_snap, activity, colors)),
        )
        .when(has_errors, |this| {
            this.child(
                div()
                    .absolute()
                    .top_0()
                    .right_0()
                    .size(gpui::px(6.0))
                    .rounded_full()
                    .bg(gpui::red()),
            )
        });

    StatusBar::new()
        .left(library_summary)
        .left(Separator::vertical())
        .left(active_tab_summary)
        .right(theme_picker)
        .right(activity_panel)
        .right(notification_panel)
}
