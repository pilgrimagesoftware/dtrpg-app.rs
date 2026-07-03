//! Formatting helpers for timestamps in the detail panel.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::data::constants::{MONTH_ABBRS, MONTH_NAMES};

/// Returns the current Unix timestamp in seconds.
fn now_secs() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH)
                     .map(|d| d.as_secs() as i64)
                     .unwrap_or(0)
}

/// Converts a Unix timestamp (seconds) to `(year, month, day)` using the
/// proleptic Gregorian calendar algorithm.
fn epoch_to_ymd(ts: i64) -> (i32, u32, u32) {
    // Days since Unix epoch (1970-01-01).
    let days = if ts >= 0 {
        ts / 86_400
    }
    else {
        (ts - 86_399) / 86_400
    };

    // Algorithm: civil calendar from days (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };

    (year as i32, m, d)
}

/// Returns `(hour_12, minute, is_pm)` from a Unix timestamp.
fn epoch_to_hms(ts: i64) -> (u32, u32, bool) {
    let secs_in_day = ((ts % 86_400) + 86_400) as u32 % 86_400;
    let total_minutes = secs_in_day / 60;
    let hour_24 = total_minutes / 60;
    let minute = total_minutes % 60;
    let is_pm = hour_24 >= 12;
    let hour_12 = match hour_24 % 12 {
        0 => 12,
        h => h,
    };
    (hour_12, minute, is_pm)
}

/// Formats a Unix timestamp as a human-readable relative string.
///
/// Buckets:
/// - < 60 s      → "just now"
/// - < 60 m      → "N minutes ago"
/// - < 24 h      → "N hours ago"
/// - 24–47 h     → "yesterday"
/// - 2–6 days    → "N days ago"
/// - 7–29 days   → "N weeks ago"
/// - same year   → "Mon D" (e.g. "Jan 5")
/// - older       → "Mon D, YYYY" (e.g. "Jan 5, 2023")
///
/// # Examples
///
/// ```
/// // Returns a relative string for a timestamp 2 hours ago.
/// let two_hours_ago = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
///                                                 .unwrap()
///                                                 .as_secs() as i64
///                     - 7200;
/// let label = dtrpg_ui::util::datetime::format_relative(two_hours_ago);
/// assert!(label.contains("hours ago") || label == "yesterday");
/// ```
pub fn format_relative(ts: i64) -> String {
    let now = now_secs();
    let elapsed = (now - ts).max(0);

    if elapsed < 60 {
        return "just now".to_string();
    }
    if elapsed < 3_600 {
        let mins = elapsed / 60;
        return format!("{mins} minute{} ago", if mins == 1 { "" } else { "s" });
    }
    if elapsed < 86_400 {
        let hrs = elapsed / 3_600;
        return format!("{hrs} hour{} ago", if hrs == 1 { "" } else { "s" });
    }
    if elapsed < 2 * 86_400 {
        return "yesterday".to_string();
    }
    if elapsed < 7 * 86_400 {
        let days = elapsed / 86_400;
        return format!("{days} days ago");
    }
    if elapsed < 30 * 86_400 {
        let weeks = elapsed / (7 * 86_400);
        return format!("{weeks} week{} ago", if weeks == 1 { "" } else { "s" });
    }

    let (ts_year, ts_month, ts_day) = epoch_to_ymd(ts);
    let (now_year, _, _) = epoch_to_ymd(now);
    let abbr = MONTH_ABBRS[(ts_month as usize).saturating_sub(1).min(11)];

    if ts_year == now_year {
        format!("{abbr} {ts_day}")
    }
    else {
        format!("{abbr} {ts_day}, {ts_year}")
    }
}

/// Formats a Unix timestamp as a full absolute date/time string.
///
/// Output format: "Month D, YYYY at H:MM AM/PM" (e.g. "January 5, 2024 at 3:42
/// PM").
///
/// # Examples
///
/// ```
/// let label = dtrpg_ui::util::datetime::format_absolute(0);
/// assert_eq!(label, "January 1, 1970 at 12:00 AM");
/// ```
pub fn format_absolute(ts: i64) -> String {
    let (year, month, day) = epoch_to_ymd(ts);
    let (hour, minute, is_pm) = epoch_to_hms(ts);
    let month_name = MONTH_NAMES[(month as usize).saturating_sub(1).min(11)];
    let ampm = if is_pm { "PM" } else { "AM" };
    format!("{month_name} {day}, {year} at {hour}:{minute:02} {ampm}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> i64 {
        now_secs()
    }

    #[test]
    fn just_now_at_59s() {
        assert_eq!(format_relative(now() - 59), "just now");
    }

    #[test]
    fn minutes_at_60s() {
        assert_eq!(format_relative(now() - 60), "1 minute ago");
    }

    #[test]
    fn minutes_at_59m() {
        assert_eq!(format_relative(now() - 59 * 60), "59 minutes ago");
    }

    #[test]
    fn hours_at_1h() {
        assert_eq!(format_relative(now() - 3_600), "1 hour ago");
    }

    #[test]
    fn hours_at_23h() {
        assert_eq!(format_relative(now() - 23 * 3_600), "23 hours ago");
    }

    #[test]
    fn yesterday_at_24h() {
        assert_eq!(format_relative(now() - 24 * 3_600), "yesterday");
    }

    #[test]
    fn yesterday_at_47h() {
        assert_eq!(format_relative(now() - 47 * 3_600), "yesterday");
    }

    #[test]
    fn days_at_48h() {
        assert_eq!(format_relative(now() - 48 * 3_600), "2 days ago");
    }

    #[test]
    fn days_at_6d() {
        assert_eq!(format_relative(now() - 6 * 86_400), "6 days ago");
    }

    #[test]
    fn weeks_at_7d() {
        assert_eq!(format_relative(now() - 7 * 86_400), "1 week ago");
    }

    #[test]
    fn weeks_at_29d() {
        assert_eq!(format_relative(now() - 29 * 86_400), "4 weeks ago");
    }

    #[test]
    fn absolute_epoch_zero() {
        assert_eq!(format_absolute(0), "January 1, 1970 at 12:00 AM");
    }

    #[test]
    fn absolute_known_date() {
        // 2024-01-05 15:42:00 UTC = 1704469320
        assert_eq!(format_absolute(1_704_469_320), "January 5, 2024 at 3:42 PM");
    }
}
