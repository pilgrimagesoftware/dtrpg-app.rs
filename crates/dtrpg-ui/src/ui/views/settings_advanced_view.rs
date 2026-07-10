//! Advanced settings section: cache visibility and destructive/maintenance
//! actions (cache details, opening the cache folder, cache clearing).
//!
//! Also renders the About section, which is purely informational (app name,
//! version, description) and shares no state with Advanced beyond both living
//! in the Settings panel.

use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};
use gpui_component::WindowExt as _;
use gpui_component::button::{Button, ButtonVariants};
use rust_i18n::t;

use crate::controllers::settings::{CacheCounts, SettingsController};
use crate::data::constants::{
    FORCE_RELOAD_COOLDOWN_SECS, ITEM_CHECK_BATCH_COOLDOWN_SECS, ITEM_CHECK_BATCH_TIMER_SECS,
    ITEM_CHECK_COOLDOWN_SECS, STALE_SECS, THUMBNAIL_COOLDOWN_SECS,
};
use crate::data::theme::ColorTokens;
use crate::util::pluralize::pluralize;

/// Formats a fixed duration in seconds as a human-readable, localized string
/// ("60 seconds", "5 minutes", "7 days"), routing the noun form through
/// [`pluralize`] rather than hand-formatting English plural suffixes.
///
/// Distinct from `activity_panel_view::format_duration`, which renders
/// *elapsed* time as "Xm Ys" — a style that reads oddly for a static
/// configuration value like a 7-day staleness window ("10080m 0s"). Chooses
/// the coarsest unit that divides evenly; falls back to seconds otherwise.
fn format_static_duration(secs: u64) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * MINUTE;
    const DAY: u64 = 24 * HOUR;

    if secs >= DAY && secs.is_multiple_of(DAY) {
        pluralize((secs / DAY) as usize, "count.day", "count.days")
    }
    else if secs >= HOUR && secs.is_multiple_of(HOUR) {
        pluralize((secs / HOUR) as usize, "count.hour", "count.hours")
    }
    else if secs >= MINUTE && secs.is_multiple_of(MINUTE) {
        pluralize((secs / MINUTE) as usize, "count.minute", "count.minutes")
    }
    else {
        pluralize(secs as usize, "count.second", "count.seconds")
    }
}

/// Renders one "Cache details" data point: a bold label + value on one line,
/// with a short explanatory description beneath in tertiary text.
///
/// Shared by both the cache counts and the timing/cooldown constants so the
/// three-line layout isn't repeated at each call site.
fn stat_row(label: impl Into<SharedString>, value: impl Into<SharedString>,
            description: impl Into<SharedString>, colors: &ColorTokens)
            -> impl IntoElement + 'static {
    let label = label.into();
    let value = value.into();
    let description = description.into();
    div().flex()
         .flex_col()
         .gap(px(2.0))
         .child(div().flex()
                     .gap(px(6.0))
                     .items_baseline()
                     .child(div().text_sm()
                                 .font_weight(gpui::FontWeight::MEDIUM)
                                 .text_color(colors.text_primary)
                                 .child(label))
                     .child(div().text_sm()
                                 .text_color(colors.text_secondary)
                                 .child(value)))
         .child(div().text_xs()
                     .text_color(colors.text_tertiary)
                     .child(description))
}

/// Renders the "Cache details" subsection: per-type cache counts and the
/// cache-related timeout/cooldown constants, each with a label and
/// description.
fn render_cache_details(cache_counts: CacheCounts, colors: &ColorTokens)
                        -> impl IntoElement + 'static + use<> {
    let avatar_value = if cache_counts.avatar_cached {
        t!("settings.cache_avatar_value_cached").to_string()
    }
    else {
        t!("settings.cache_avatar_value_not_cached").to_string()
    };

    div().flex()
         .flex_col()
         .gap(px(12.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(t!("settings.cache_details_title")))
         .child(stat_row(t!("settings.cache_metadata_label"),
                         cache_counts.metadata_items.to_string(),
                         t!("settings.cache_metadata_description"),
                         colors))
         .child(stat_row(t!("settings.cache_covers_label"),
                         cache_counts.cover_thumbnails.to_string(),
                         t!("settings.cache_covers_description"),
                         colors))
         .child(stat_row(t!("settings.cache_avatar_label"),
                         avatar_value,
                         t!("settings.cache_avatar_description"),
                         colors))
         .child(stat_row(t!("settings.cache_staleness_label"),
                         format_static_duration(STALE_SECS),
                         t!("settings.cache_staleness_description"),
                         colors))
         .child(stat_row(t!("settings.cache_reload_cooldown_label"),
                         format_static_duration(FORCE_RELOAD_COOLDOWN_SECS),
                         t!("settings.cache_reload_cooldown_description"),
                         colors))
         .child(stat_row(t!("settings.cache_item_check_cooldown_label"),
                         format_static_duration(ITEM_CHECK_COOLDOWN_SECS),
                         t!("settings.cache_item_check_cooldown_description"),
                         colors))
         .child(stat_row(t!("settings.cache_batch_cooldown_label"),
                         format_static_duration(ITEM_CHECK_BATCH_COOLDOWN_SECS),
                         t!("settings.cache_batch_cooldown_description"),
                         colors))
         .child(stat_row(t!("settings.cache_batch_timer_label"),
                         format_static_duration(ITEM_CHECK_BATCH_TIMER_SECS),
                         t!("settings.cache_batch_timer_description"),
                         colors))
         .child(stat_row(t!("settings.cache_thumbnail_cooldown_label"),
                         format_static_duration(THUMBNAIL_COOLDOWN_SECS),
                         t!("settings.cache_thumbnail_cooldown_description"),
                         colors))
}

/// Renders the Advanced settings section: cache visibility (counts and
/// timing constants), an "Open cache folder" action, and a "Clear cache"
/// action with a confirmation dialog.
pub fn render_advanced_section(entity: Entity<SettingsController>, cache_counts: CacheCounts,
                               colors: &ColorTokens)
                               -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let border = colors.border;
    let entity_open_folder = entity.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(t!("settings.advanced_title")),
        )
        .child(div().h(px(1.0)).bg(border))
        .child(render_cache_details(cache_counts, colors))
        .child(
            Button::new("open-cache-folder-btn")
                .outline()
                .label(t!("settings.open_cache_folder_button"))
                .on_click(move |_, _, cx| {
                    entity_open_folder.read(cx).open_cache_folder();
                }),
        )
        .child(div().h(px(1.0)).bg(border))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child(t!("settings.clear_cache_description")),
                )
                .child(
                    Button::new("clear-cache-btn")
                        .danger()
                        .label(t!("settings.clear_cache_button"))
                        .on_click(move |_, window, cx| {
                            let entity = entity.clone();
                            window.open_alert_dialog(cx, move |alert, _, _| {
                                let entity = entity.clone();
                                alert
                                    .confirm()
                                    .title(t!("settings.clear_cache_confirm_title").to_string())
                                    .description(
                                        t!("settings.clear_cache_confirm_description").to_string(),
                                    )
                                    .on_ok(move |_, _window, cx| {
                                        entity.update(cx, |ctrl, cx| ctrl.clear_cache(cx));
                                        true
                                    })
                            });
                        }),
                ),
        )
}

/// Renders the About settings section: app name, version, and a short
/// description.
pub fn render_about_section(colors: &ColorTokens) -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;

    div().flex()
         .flex_col()
         .gap(px(8.0))
         .p(px(24.0))
         .child(div().text_lg()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(text_primary)
                     .child(t!("sidebar.app_name")))
         .child(div().text_sm()
                     .text_color(text_secondary)
                     .child(t!("about.version", version = env!("CARGO_PKG_VERSION"))))
         .child(div().text_xs()
                     .text_color(text_secondary)
                     .child(t!("about.description")))
}

#[cfg(test)]
mod tests {
    use super::format_static_duration;

    #[test]
    fn formats_seconds_under_a_minute() {
        assert_eq!(format_static_duration(60), "1 minute");
        assert_eq!(format_static_duration(45), "45 seconds");
        assert_eq!(format_static_duration(1), "1 second");
    }

    #[test]
    fn formats_minutes() {
        assert_eq!(format_static_duration(300), "5 minutes");
        assert_eq!(format_static_duration(900), "15 minutes");
    }

    #[test]
    fn formats_days() {
        assert_eq!(format_static_duration(7 * 24 * 60 * 60), "7 days");
        assert_eq!(format_static_duration(24 * 60 * 60), "1 day");
    }
}
