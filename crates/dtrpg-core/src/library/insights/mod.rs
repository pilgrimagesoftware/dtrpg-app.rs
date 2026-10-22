//! The library's aggregates for the Insights view and the MCP statistics
//! tool, worked out from the catalog alone so they work offline.
//!
//! Everything here is a plain function of the items, with no window and no
//! I/O (the storage roots are the one part that reads the disk, in
//! [`Insights::with_roots`]), so it is tested directly and can run off the
//! main thread.

use std::collections::HashMap;

use chrono::{DateTime, Datelike, Utc};

use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::product_filter::{CATEGORY_PRODUCT_TYPE, CATEGORY_RULE_SYSTEM, names_in};
use crate::data::storage::roots::{StorageRoot, root_capacity};

mod csv;

/// A label and how many titles carry it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Count {
    /// What is counted, such as a publisher's name.
    pub label: String,
    /// How many titles.
    pub count: usize,
}

/// A label and the disk space its titles take, in megabytes.
#[derive(Clone, Debug, PartialEq)]
pub struct Share {
    /// What the space belongs to, such as a publisher's name.
    pub label: String,
    /// Megabytes.
    pub mb:    f64,
}

/// How many titles were added in one month.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MonthCount {
    /// The year, such as 2026.
    pub year:  i32,
    /// The month, 1 to 12.
    pub month: u32,
    /// How many titles.
    pub count: usize,
}

/// What Libri is using on one storage location.
#[derive(Clone, Debug, PartialEq)]
pub struct RootUsage {
    /// The location's name.
    pub name:         String,
    /// Space Libri's files take there, in megabytes, if it could be measured.
    pub used_mb:      Option<f64>,
    /// Free space on the volume, in megabytes, if it could be measured.
    pub available_mb: Option<f64>,
    /// Size of the volume, in megabytes, if it could be measured.
    pub total_mb:     Option<f64>,
}

/// The library's aggregates.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Insights {
    /// Titles in the library.
    pub titles:            usize,
    /// Combined size of every title, in megabytes.
    pub total_size_mb:     f64,
    /// Combined page count.
    pub total_pages:       u64,
    /// Titles that are on this machine.
    pub downloaded:        usize,
    /// Different publishers.
    pub publishers:        usize,
    /// Different game systems.
    pub systems:           usize,
    /// Titles added in the month of the time given to
    /// [`compute`](Self::compute).
    pub added_this_month:  usize,
    /// Titles per publisher, most first.
    pub by_publisher:      Vec<Count>,
    /// Titles per game system, most first.
    pub by_system:         Vec<Count>,
    /// Titles per product type, most first.
    pub by_type:           Vec<Count>,
    /// Disk space per publisher, most first.
    pub disk_by_publisher: Vec<Share>,
    /// Titles added each month, oldest first, with no month left out between
    /// the first and the last.
    pub added_by_month:    Vec<MonthCount>,
    /// What Libri uses on each storage location. Empty until
    /// [`with_roots`](Self::with_roots) fills it in.
    pub roots:             Vec<RootUsage>,
}

impl Insights {
    /// Works out the aggregates of `items`. `now_secs` is the time "this
    /// month" is measured from, in Unix seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::library::insights::Insights;
    ///
    /// let insights = Insights::compute(&[], 1_700_000_000);
    /// assert_eq!(insights.titles, 0);
    /// ```
    #[must_use]
    pub fn compute(items: &[LibraryItem], now_secs: i64) -> Self {
        let by_publisher = count_by(items, |item| Some(item.publisher.to_string()));
        let by_system = count_by(items, |item| deepest(item, CATEGORY_RULE_SYSTEM));
        let by_type = count_by(items, |item| deepest(item, CATEGORY_PRODUCT_TYPE));
        let added_by_month = months_added(items);
        let this_month = month_of(now_secs);
        Self { titles: items.len(),
               total_size_mb: items.iter().map(|item| item.size_mb).sum(),
               total_pages: items.iter().map(|item| u64::from(item.pages)).sum(),
               downloaded: items.iter()
                                .filter(|item| item.status == ItemStatus::Downloaded)
                                .count(),
               publishers: by_publisher.len(),
               systems: by_system.len(),
               added_this_month:
                   added_by_month.iter()
                                 .filter(|entry| Some((entry.year, entry.month)) == this_month)
                                 .map(|entry| entry.count)
                                 .sum(),
               disk_by_publisher: disk_by_publisher(items),
               by_publisher,
               by_system,
               by_type,
               added_by_month,
               roots: Vec::new() }
    }

    /// Fills in what Libri uses on each of `roots`. Reads the disk, so it
    /// belongs on a background thread.
    #[must_use]
    pub fn with_roots(mut self, roots: &[StorageRoot]) -> Self {
        self.roots = root_usages(roots);
        self
    }

    /// The aggregates as CSV: a `section,label,value` row for each figure, so
    /// a spreadsheet can read every part of it from one file.
    #[must_use]
    pub fn to_csv(&self) -> String {
        csv::render(self)
    }
}

/// What Libri uses on each of `roots`, and the room left on each one's volume.
/// Reads the disk, so it belongs on a background thread.
#[must_use]
pub fn root_usages(roots: &[StorageRoot]) -> Vec<RootUsage> {
    roots.iter()
         .map(|root| {
             let capacity = root_capacity(root);
             RootUsage { name:         root.name.clone(),
                         used_mb:      capacity.used_mb,
                         available_mb: capacity.available_mb,
                         total_mb:     capacity.total_mb, }
         })
         .collect()
}

/// The most specific name `item` has in `category`: the last the product lists,
/// since a product lists each level from the root down.
fn deepest(item: &LibraryItem, category: &str) -> Option<String> {
    names_in(&item.filters, category).last()
                                     .map(ToString::to_string)
}

/// Counts the titles for each label `label_of` gives, ignoring case, most first
/// and then by label. A title with no label is left out. The first spelling
/// seen is the one shown.
fn count_by(items: &[LibraryItem], label_of: impl Fn(&LibraryItem) -> Option<String>)
            -> Vec<Count> {
    let mut counts: HashMap<String, Count> = HashMap::new();
    for label in items.iter()
                      .filter_map(label_of)
                      .filter(|label| !label.is_empty())
    {
        counts.entry(label.to_lowercase())
              .or_insert_with(|| Count { label, count: 0 })
              .count += 1;
    }
    let mut counts: Vec<Count> = counts.into_values().collect();
    counts.sort_by(|a, b| {
              b.count
               .cmp(&a.count)
               .then_with(|| a.label.to_lowercase().cmp(&b.label.to_lowercase()))
          });
    counts
}

/// Disk space per publisher, ignoring case, most first.
fn disk_by_publisher(items: &[LibraryItem]) -> Vec<Share> {
    let mut shares: HashMap<String, Share> = HashMap::new();
    for item in items.iter().filter(|item| !item.publisher.is_empty()) {
        shares.entry(item.publisher.to_lowercase())
              .or_insert_with(|| Share { label: item.publisher.to_string(),
                                         mb:    0.0, })
              .mb += item.size_mb;
    }
    let mut shares: Vec<Share> = shares.into_values().collect();
    shares.sort_by(|a, b| b.mb.total_cmp(&a.mb).then_with(|| a.label.cmp(&b.label)));
    shares
}

/// The year and month of `secs`, or `None` if it is not a time.
fn month_of(secs: i64) -> Option<(i32, u32)> {
    DateTime::<Utc>::from_timestamp(secs, 0).map(|time| (time.year(), time.month()))
}

/// Titles added per month, oldest first, with the months between the first and
/// the last that had none shown as zero.
fn months_added(items: &[LibraryItem]) -> Vec<MonthCount> {
    let mut counts: HashMap<(i32, u32), usize> = HashMap::new();
    for month in items.iter()
                      .filter_map(|item| item.date_added)
                      .filter_map(month_of)
    {
        *counts.entry(month).or_default() += 1;
    }
    let (Some(first), Some(last)) = (counts.keys().min().copied(), counts.keys().max().copied())
    else {
        return Vec::new();
    };
    let mut months = Vec::new();
    let (mut year, mut month) = first;
    while (year, month) <= last {
        months.push(MonthCount { year,
                                 month,
                                 count: counts.get(&(year, month)).copied().unwrap_or(0) });
        if month == 12 {
            (year, month) = (year + 1, 1);
        }
        else {
            month += 1;
        }
    }
    months
}

#[cfg(test)]
mod tests;
