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

/// Converts a proleptic Gregorian civil date to days since the Unix epoch.
///
/// Inverse of [`epoch_to_ymd`]; uses Howard Hinnant's `days_from_civil`
/// algorithm.
fn civil_to_days(year: i32, month: u32, day: u32) -> i64 {
    let y = if month <= 2 {
        year as i64 - 1
    }
    else {
        year as i64
    };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe as i64 - 719_468
}

/// Parses an RFC 3339 / ISO 8601 timestamp (e.g. `2024-07-16T10:45:52-05:00`
/// or `2024-07-16T10:45:52Z`) into a Unix timestamp in seconds.
///
/// Accepts an optional fractional-seconds component (discarded) and either a
/// `Z` suffix or a numeric `+HH:MM` / `-HH:MM` offset. Returns `None` if the
/// input does not match the expected shape.
///
/// # Examples
///
/// ```
/// let ts = dtrpg_ui::util::datetime::parse_rfc3339_to_epoch("2024-07-16T10:45:52-05:00");
/// assert_eq!(ts, Some(1_721_144_752));
/// ```
pub fn parse_rfc3339_to_epoch(input: &str) -> Option<i64> {
    let bytes = input.as_bytes();
    if bytes.len() < 19 || bytes.get(4) != Some(&b'-') || bytes.get(7) != Some(&b'-') {
        return None;
    }
    let date_part = input.get(..10)?;
    let time_start = input.get(10..11)?;
    if time_start != "T" && time_start != "t" && time_start != " " {
        return None;
    }
    let time_part = input.get(11..19)?;

    let year: i32 = date_part.get(0..4)?.parse().ok()?;
    let month: u32 = date_part.get(5..7)?.parse().ok()?;
    let day: u32 = date_part.get(8..10)?.parse().ok()?;
    let hour: i64 = time_part.get(0..2)?.parse().ok()?;
    let minute: i64 = time_part.get(3..5)?.parse().ok()?;
    let second: i64 = time_part.get(6..8)?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }

    let remainder = input.get(19..)?;
    let offset_str = match remainder.find(['+', '-', 'Z', 'z']) {
        Some(idx) => remainder.get(idx..)?,
        None => return None,
    };

    let offset_secs: i64 = if offset_str.eq_ignore_ascii_case("z") {
        0
    }
    else {
        let sign = if offset_str.starts_with('-') { -1 } else { 1 };
        let digits = offset_str.get(1..)?;
        let oh: i64 = digits.get(0..2)?.parse().ok()?;
        let om: i64 = digits.get(3..5)?.parse().ok()?;
        sign * (oh * 3_600 + om * 60)
    };

    let days = civil_to_days(year, month, day);
    let utc_secs = days * 86_400 + hour * 3_600 + minute * 60 + second - offset_secs;
    Some(utc_secs)
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

    #[test]
    fn parses_rfc3339_with_negative_offset() {
        // 2024-07-16T10:45:52-05:00 == 2024-07-16T15:45:52Z
        assert_eq!(parse_rfc3339_to_epoch("2024-07-16T10:45:52-05:00"),
                   Some(1_721_144_752));
    }

    #[test]
    fn parses_rfc3339_with_positive_offset() {
        // 2024-07-17T00:45:52+09:00 == 2024-07-16T15:45:52Z
        assert_eq!(parse_rfc3339_to_epoch("2024-07-17T00:45:52+09:00"),
                   Some(1_721_144_752));
    }

    #[test]
    fn parses_rfc3339_with_z_suffix() {
        assert_eq!(parse_rfc3339_to_epoch("2024-01-05T15:42:00Z"),
                   Some(1_704_469_320));
    }

    #[test]
    fn parses_rfc3339_epoch_zero() {
        assert_eq!(parse_rfc3339_to_epoch("1970-01-01T00:00:00Z"), Some(0));
    }

    #[test]
    fn rejects_malformed_rfc3339() {
        assert_eq!(parse_rfc3339_to_epoch("not-a-date"), None);
        assert_eq!(parse_rfc3339_to_epoch("2024-13-05T15:42:00Z"), None);
        assert_eq!(parse_rfc3339_to_epoch(""), None);
    }

    #[test]
    fn civil_to_days_round_trips_epoch_to_ymd() {
        for ts in [0_i64, 1_704_469_320, -86_400, 1_721_140_952] {
            let (y, m, d) = epoch_to_ymd(ts);
            let days = civil_to_days(y, m, d);
            assert_eq!(days,
                       if ts >= 0 {
                           ts / 86_400
                       }
                       else {
                           (ts - 86_399) / 86_400
                       });
        }
    }
}
