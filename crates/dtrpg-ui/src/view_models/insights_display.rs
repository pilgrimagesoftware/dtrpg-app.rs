//! What the Insights page says about the library's aggregates, as plain data.
//! No window and no I/O, so it can be tested directly.

use dtrpg_core::library::insights::{Count, Insights, MonthCount};

use crate::controllers::library::LiveAvailability;

/// How many rows a "top" card shows.
pub const TOP_ROWS: usize = 8;

/// One row of a bar card: a label, a count, and the bar's length as a share of
/// the longest bar.
#[derive(Clone, Debug, PartialEq)]
pub struct BarRow {
    /// The row's label, such as a publisher's name.
    pub label:    String,
    /// The count shown at the end of the row.
    pub count:    usize,
    /// The bar's length as a share of the longest bar, `0.0..=1.0`.
    pub fraction: f32,
}

/// The first `limit` of `counts` as bar rows, with how many there are in all.
///
/// # Examples
///
/// ```
/// use dtrpg_core::library::insights::Count;
/// use dtrpg_ui::view_models::insights_display::top_rows;
///
/// let counts = [Count { label: "Paizo".into(),
///                       count: 4, },
///               Count { label: "Chaosium".into(),
///                       count: 2, }];
/// let (rows, all) = top_rows(&counts, 1);
/// assert_eq!(all, 2);
/// assert_eq!(rows.len(), 1);
/// assert!((rows[0].fraction - 1.0).abs() < f32::EPSILON);
/// ```
#[must_use]
pub fn top_rows(counts: &[Count], limit: usize) -> (Vec<BarRow>, usize) {
    let longest = counts.first().map_or(1, |first| first.count.max(1));
    let rows = counts.iter()
                     .take(limit)
                     .map(|entry| BarRow { label:    entry.label.clone(),
                                           count:    entry.count,
                                           fraction: entry.count as f32 / longest as f32, })
                     .collect();
    (rows, counts.len())
}

/// A slice of the availability ring.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Availability {
    /// The title is on this machine.
    OnDevice,
    /// The title is only in the cloud.
    NotDownloaded,
    /// The title is being downloaded now.
    Downloading,
    /// The title's last download failed.
    Failed,
}

/// How many titles are in each availability state. A title that is downloading
/// or failed is counted there and not as not downloaded, so the slices add up
/// to the number of titles.
#[must_use]
pub fn availability_slices(insights: &Insights, live: LiveAvailability)
                           -> Vec<(Availability, usize)> {
    let not_downloaded = insights.titles.saturating_sub(insights.downloaded);
    let in_progress = (live.downloading + live.failed).min(not_downloaded);
    let downloading = live.downloading.min(in_progress);
    vec![(Availability::OnDevice, insights.downloaded),
         (Availability::NotDownloaded, not_downloaded - in_progress),
         (Availability::Downloading, downloading),
         (Availability::Failed, in_progress - downloading)]
}

/// A month as the time chart labels it, such as `07/26`.
#[must_use]
pub fn month_label(month: &MonthCount) -> String {
    format!("{:02}/{:02}", month.month, month.year.rem_euclid(100))
}

#[cfg(test)]
mod tests;
