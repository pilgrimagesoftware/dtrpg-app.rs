//! Status bar view: library totals, current-section summary, theme picker,
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

use std::sync::Arc;

use dtrpg_core::data::activity::{ActivitySnapshot, AlertHistorySnapshot};
use dtrpg_core::data::mcp_access::McpTool;
use dtrpg_core::library::insights::RootUsage;
use gpui::prelude::*;
use gpui::{Anchor, Entity, IntoElement, Styled, div, px};
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_kit::component::popover::Popover;
use gpui_kit::component::progress::Progress;
use gpui_kit::component::separator::Separator;
use gpui_kit::component::status_bar::StatusBar;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::component::{Sizable as _, Size};
use rust_i18n::t;

use crate::controllers::activity::ActivityController;
use crate::controllers::library::LibraryController;
use crate::data::theme::ColorTokens;
use crate::data::theme_catalog::{ThemeCatalog, ThemeId, theme_label};
use crate::i18n::locale::Locale;
use crate::ui::views::activity_panel::render_activity_panel;
use crate::ui::views::alert_history::render_alert_history_panel;
use crate::util::pluralize::pluralize;
use crate::util::size::size_format;
use crate::view_models::storage_display::volume_size_label;

/// The color of the Agent dot when agents may use Libri: a green that stays the
/// same in every theme, so "on" never looks like "off".
const AGENT_ON_COLOR: u32 = 0x56_8B_5A;

/// Data the status bar needs to render, decoupled from where it's sourced.
pub struct StatusBarSnapshot {
    /// Total number of items in the library.
    pub total_count:    usize,
    /// Total size of all library items, in megabytes.
    pub total_mb:       f64,
    /// Title of the catalog section being shown.
    pub section_label:  String,
    /// Number of items matching in that section.
    pub section_count:  usize,
    /// Currently active theme.
    pub theme_key:      ThemeId,
    /// Currently active locale.
    pub current_locale: Locale,
    /// Whether agents may use Libri.
    pub agent_enabled:  bool,
    /// How many tools an agent can call right now.
    pub agent_exposed:  usize,
    /// What Libri uses on each storage location.
    pub roots:          Arc<Vec<RootUsage>>,
    /// How many titles are being downloaded now.
    pub downloading:    usize,
}

/// Activity and alert history state needed to render the status bar's
/// activity/notification buttons and their anchored panels.
pub struct ActivityBarData<'a> {
    /// Controller entity, used to toggle the panels and act on their contents.
    pub entity:     Entity<ActivityController>,
    /// Activity panel snapshot for the current render pass.
    pub snap:       &'a ActivitySnapshot,
    /// Alert history panel snapshot for the current render pass.
    pub alert_snap: &'a AlertHistorySnapshot,
}

/// Builds a `Button` tooltip with a title line above a descriptive body line.
///
/// `Button`'s tooltip only accepts plain text, so the title and body are
/// joined with a newline rather than rendered as separate styled elements.
fn status_bar_tooltip(title: &str, body: &str) -> String {
    format!("{title}\n{body}")
}

/// Renders the status bar row below the main content area.
pub fn render_status_bar(snap: StatusBarSnapshot, entity: Entity<LibraryController>,
                         activity_data: ActivityBarData<'_>, colors: &ColorTokens)
                         -> impl IntoElement + 'static {
    // "18 of 18 titles": how many are on show of how many there are.
    let titles_summary = div().text_xs().text_color(colors.text_secondary).child(
        t!(
            "status_bar.titles_of",
            shown = snap.section_count,
            total = pluralize(snap.total_count, "count.title", "count.titles")
        )
        .to_string(),
    );
    let size_summary = div().text_xs()
                            .text_color(colors.text_secondary)
                            .child(size_format(snap.total_mb));

    let language_entity = entity.clone();
    let language_picker = Button::new("status-bar-language")
        .ghost()
        .compact()
        .icon(Icon::empty().path("icons/globe.svg"))
        .label(snap.current_locale.code().to_uppercase())
        .tooltip(
            t!(
                "status_bar.language_tooltip",
                language = snap.current_locale.label()
            )
            .to_string(),
        )
        .dropdown_menu(move |menu, _, _| {
            let mut m = menu;
            for locale in Locale::ALL {
                let e = language_entity.clone();
                m = m.item(
                    PopupMenuItem::new(locale.label())
                        .checked(locale == snap.current_locale)
                        .on_click(move |_, _, cx| {
                            e.update(cx, |ctrl, cx| ctrl.set_locale(locale, cx));
                        }),
                );
            }
            m
        });

    let theme_picker =
        Button::new("status-bar-theme")
            .ghost()
            .compact()
            .label(theme_label(&snap.theme_key))
            .tooltip(
                t!(
                    "status_bar.theme_tooltip",
                    theme = theme_label(&snap.theme_key)
                )
                .to_string(),
            )
            .dropdown_menu(move |menu, _, _| {
                let mut m = menu;
                for id in ThemeCatalog::all_ids() {
                    let e = entity.clone();
                    let lbl = theme_label(&id);
                    let checked = id == snap.theme_key;
                    m = m.item(PopupMenuItem::new(lbl).checked(checked).on_click(
                        move |_, _, cx| {
                            e.update(cx, |ctrl, cx| ctrl.set_theme(id.clone(), cx));
                        },
                    ));
                }
                m
            });

    let ActivityBarData { entity: activity,
                          snap: activity_snap,
                          alert_snap, } = activity_data;

    // Only mention completed items in the tooltip when there are any this
    // session — otherwise it always reads "0 completed", which is noise.
    let activity_tooltip_body = if activity_snap.recent_count > 0 {
        t!("status_bar.activity_tooltip",
           in_progress = activity_snap.in_progress_count,
           completed = activity_snap.recent_count)
    }
    else {
        t!("status_bar.activity_tooltip_in_progress_only",
           in_progress = activity_snap.in_progress_count)
    };
    // The activity button shows a download arrow and, while titles are being
    // downloaded, how many. It opens the activity panel, which lists every
    // task in progress, not only downloads.
    let downloading_label = (snap.downloading > 0).then(|| {
                                                      t!("status_bar.downloading",
                                                         n = snap.downloading).to_string()
                                                  });
    let activity_indicator =
        Button::new("status-bar-activity").ghost()
                                          .compact()
                                          .icon(Icon::empty().path("icons/download.svg"))
                                          .when_some(downloading_label, |button, label| {
                                              button.label(label)
                                          })
                                          .tooltip(status_bar_tooltip(&t!("activity.title"),
                                                                      &activity_tooltip_body));

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

    let has_unread_alert = alert_snap.has_unread;
    let notification_button = Button::new("status-bar-notifications").ghost()
                                                                     .compact()
                                                                     .icon(IconName::Bell)
                                                                     .tooltip(status_bar_tooltip(
        &t!("status_bar.notifications_tooltip_title"),
        &t!(
            "status_bar.notifications_tooltip",
            n = activity_snap.alert_count
        ),
    ));

    let alert_for_open_change = activity.clone();
    // `Popover::trigger` requires `Selectable`, which `Badge` doesn't
    // implement, so the unread-alert dot is layered on as a sibling of the
    // popover rather than wrapping the trigger in a `Badge`.
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
        .when(has_unread_alert, |this| {
            this.child(
                div()
                    .absolute()
                    .top_0()
                    .right_0()
                    .size(px(6.0))
                    .rounded_full()
                    .bg(gpui::red()),
            )
        });

    // Whether agents may use Libri: a green dot when they may, a grey one when
    // not. A coloured dot alone says nothing, so the word beside it and the
    // tooltip say what it means.
    let agent_dot = if snap.agent_enabled {
        gpui::rgb(AGENT_ON_COLOR).into()
    }
    else {
        colors.text_tertiary
    };
    let agent_tooltip = if snap.agent_enabled {
        t!("status_bar.agent_on_tooltip",
           exposed = snap.agent_exposed,
           all = McpTool::ALL.len()).to_string()
    }
    else {
        t!("status_bar.agent_off_tooltip").to_string()
    };
    let agent_indicator =
        div().id("status-bar-agent")
             .flex()
             .items_center()
             .gap(px(6.0))
             .px(px(8.0))
             .tooltip(move |window, cx| Tooltip::new(agent_tooltip.clone()).build(window, cx))
             .child(div().size(px(8.0)).rounded_full().bg(agent_dot))
             .child(Icon::empty().path("icons/bot.svg")
                                 .with_size(Size::Small)
                                 .text_color(colors.text_secondary))
             .child(div().text_xs()
                         .text_color(colors.text_secondary)
                         .child(t!("status_bar.agent").to_string()));

    let mut bar = StatusBar::new().left(titles_summary)
                                  .left(Separator::vertical().h_5())
                                  .left(size_summary);
    for (index, root) in snap.roots.iter().enumerate() {
        bar = bar.left(Separator::vertical().h_5())
                 .left(root_meter(index, root, colors));
    }
    bar.right(agent_indicator)
       .right(Separator::vertical().h_5())
       .right(activity_panel)
       .right(Separator::vertical().h_5())
       .right(language_picker)
       .right(Separator::vertical().h_5())
       .right(theme_picker)
       .right(Separator::vertical().h_5())
       .right(notification_panel)
       .w_full()
       .h_full()
}

/// The width of a storage location's meter, in pixels.
const ROOT_METER_WIDTH: f32 = 48.0;

/// A storage location's name and a small meter of how much of its volume Libri
/// fills. The tooltip gives the figures. A location whose volume could not be
/// measured shows its name alone.
fn root_meter(index: usize, root: &RootUsage, colors: &ColorTokens) -> impl IntoElement {
    let fraction = match (root.used_mb, root.total_mb) {
        (Some(used), Some(total)) if total > 0.0 => Some(used / total),
        _ => None,
    };
    let tooltip = match (root.used_mb, root.available_mb) {
        (Some(used), Some(free)) => t!("status_bar.root_tooltip",
                                       name = root.name,
                                       used = volume_size_label(used),
                                       free = volume_size_label(free)).to_string(),
        _ => root.name.clone(),
    };
    div().id(("status-bar-root", index))
         .flex()
         .items_center()
         .gap(px(6.0))
         .px(px(8.0))
         .tooltip(move |window, cx| Tooltip::new(tooltip.clone()).build(window, cx))
         .child(Icon::empty().path("icons/hard-drive.svg")
                             .with_size(Size::Small)
                             .text_color(colors.text_secondary))
         .child(div().text_xs()
                     .text_color(colors.text_secondary)
                     .child(root.name.clone()))
         .children(fraction.map(|fraction| {
                               div().w(px(ROOT_METER_WIDTH)).child(
                Progress::new(("status-bar-root-meter", index))
                    .value((fraction * 100.0).clamp(1.0, 100.0) as f32)
                    .color(colors.accent),
            )
                           }))
}
