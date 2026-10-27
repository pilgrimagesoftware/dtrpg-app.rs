//! The Cache card on the Insights page: what the app's regenerable caches hold
//! now, and the timing the app keeps them by. It takes counts already read, so
//! it touches no disk.

use dtrpg_core::data::cache_counts::CacheCounts;
use dtrpg_core::data::constants::{
    FORCE_RELOAD_COOLDOWN_SECS, ITEM_CHECK_BATCH_COOLDOWN_SECS, ITEM_CHECK_BATCH_TIMER_SECS,
    ITEM_CHECK_COOLDOWN_SECS, STALE_SECS, THUMBNAIL_COOLDOWN_SECS,
};
use gpui::prelude::*;
use gpui::{SharedString, div, px};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::v_flex;
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::ui::views::insights::cards::card;
use crate::util::datetime::{format_absolute, format_relative};
use crate::util::pluralize::pluralize;

/// Formats a fixed duration in seconds as a human-readable, localized string
/// ("60 seconds", "5 minutes", "7 days"), choosing the coarsest unit that
/// divides evenly and falling back to seconds.
///
/// Distinct from `activity_panel_view::format_duration`, which renders
/// *elapsed* time as "Xm Ys", a style that reads oddly for a fixed setting like
/// a 7-day staleness window ("10080m 0s").
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

/// One row: the name on the left, the value on the right, and what it means
/// under them.
fn row(label: String, value: impl IntoElement, description: String, colors: &ColorTokens)
       -> impl IntoElement {
    v_flex().gap(px(2.0))
            .child(div().flex()
                        .justify_between()
                        .items_baseline()
                        .gap(px(12.0))
                        .child(div().text_sm()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(colors.text_primary)
                                    .child(label))
                        .child(value))
            .child(div().text_xs()
                        .text_color(colors.text_secondary)
                        .child(description))
}

/// A row whose value is plain text.
fn stat_row(label: String, value: String, description: String, colors: &ColorTokens)
            -> impl IntoElement {
    row(label,
        div().text_sm()
             .text_color(colors.text_secondary)
             .child(value),
        description,
        colors)
}

/// A row for a recorded time: the value reads as relative ("2 hours ago"), with
/// the exact time in a tooltip. `id` names the value so it can carry the
/// tooltip.
fn timestamp_row(id: &'static str, label: String, ts: i64, description: String,
                 colors: &ColorTokens)
                 -> impl IntoElement {
    let absolute = format_absolute(ts);
    let value = div().id(SharedString::from(id))
                     .text_sm()
                     .text_color(colors.text_secondary)
                     .child(format_relative(ts))
                     .tooltip(move |window, cx| Tooltip::new(absolute.clone()).build(window, cx));
    row(label, value, description, colors)
}

/// The Cache card: counts of what is kept, when it was saved, and the
/// cooldowns and timers that govern it.
pub fn cache_card(counts: CacheCounts, colors: &ColorTokens) -> impl IntoElement {
    let avatar = if counts.avatar_cached {
                     t!("settings.cache_avatar_value_cached")
                 }
                 else {
                     t!("settings.cache_avatar_value_not_cached")
                 }.to_string();
    let text = |key: &str| t!(key).to_string();
    let body = v_flex().gap(px(12.0))
                       .child(stat_row(text("settings.cache_metadata_label"),
                                       counts.metadata_items.to_string(),
                                       text("settings.cache_metadata_description"),
                                       colors))
                       .children(counts.metadata_saved_at_secs.map(|ts| {
                                                                   timestamp_row("cache-stat-metadata-saved",
                                                                                 text("settings.cache_metadata_saved_label"),
                                                                                 ts,
                                                                                 text("settings.cache_metadata_saved_description"),
                                                                                 colors)
                                                               }))
                       .child(stat_row(text("settings.cache_covers_label"),
                                       counts.cover_thumbnails.to_string(),
                                       text("settings.cache_covers_description"),
                                       colors))
                       .child(stat_row(text("settings.cache_avatar_label"),
                                       avatar,
                                       text("settings.cache_avatar_description"),
                                       colors))
                       .child(stat_row(text("settings.cache_staleness_label"),
                                       format_static_duration(STALE_SECS),
                                       text("settings.cache_staleness_description"),
                                       colors))
                       .child(stat_row(text("settings.cache_reload_cooldown_label"),
                                       format_static_duration(FORCE_RELOAD_COOLDOWN_SECS),
                                       text("settings.cache_reload_cooldown_description"),
                                       colors))
                       .child(stat_row(text("settings.cache_item_check_cooldown_label"),
                                       format_static_duration(ITEM_CHECK_COOLDOWN_SECS),
                                       text("settings.cache_item_check_cooldown_description"),
                                       colors))
                       .child(stat_row(text("settings.cache_batch_cooldown_label"),
                                       format_static_duration(ITEM_CHECK_BATCH_COOLDOWN_SECS),
                                       text("settings.cache_batch_cooldown_description"),
                                       colors))
                       .children(counts.last_item_check_batch_secs.map(|ts| {
                                                                       timestamp_row("cache-stat-last-batch-check",
                                                                                     text("settings.cache_last_batch_check_label"),
                                                                                     ts,
                                                                                     text("settings.cache_last_batch_check_description"),
                                                                                     colors)
                                                                   }))
                       .child(stat_row(text("settings.cache_batch_timer_label"),
                                       format_static_duration(ITEM_CHECK_BATCH_TIMER_SECS),
                                       text("settings.cache_batch_timer_description"),
                                       colors))
                       .child(stat_row(text("settings.cache_thumbnail_cooldown_label"),
                                       format_static_duration(THUMBNAIL_COOLDOWN_SECS),
                                       text("settings.cache_thumbnail_cooldown_description"),
                                       colors));
    card("insights-cache",
         t!("insights.cache_title").to_string(),
         t!("insights.cache_note").to_string(),
         colors,
         body)
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

    #[test]
    fn the_cache_card_texts_resolve() {
        use rust_i18n::t;

        for key in ["insights.cache_title",
                    "insights.cache_note",
                    "settings.cache_details_moved"]
        {
            assert_ne!(t!(key).to_string(), key, "{key} has no text");
        }
    }
}
