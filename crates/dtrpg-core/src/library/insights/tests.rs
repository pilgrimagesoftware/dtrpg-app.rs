#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::*;
use crate::data::library::LibraryItem;

fn item(id: &str, publisher: &str, size_mb: f64) -> LibraryItem {
    let mut item = LibraryItem::new(id,
                                    id,
                                    publisher,
                                    "",
                                    "",
                                    "PDF",
                                    10,
                                    size_mb,
                                    2020,
                                    0,
                                    ItemStatus::Cloud,
                                    "#000000",
                                    "",
                                    None);
    item.size_mb = size_mb;
    item
}

// 2026-09-15 and 2026-07-02, in Unix seconds.
const SEPTEMBER: i64 = 1_789_430_400;
const JULY: i64 = 1_782_950_400;

#[test]
fn the_glance_adds_up_the_catalog() {
    let mut downloaded = item("a", "Paizo", 100.0);
    downloaded.status = ItemStatus::Downloaded;
    let items = [downloaded,
                 item("b", "paizo", 50.0),
                 item("c", "Chaosium", 25.0)];

    let insights = Insights::compute(&items, SEPTEMBER);

    assert_eq!(insights.titles, 3);
    assert!((insights.total_size_mb - 175.0).abs() < f64::EPSILON);
    assert_eq!(insights.total_pages, 30);
    assert_eq!(insights.downloaded, 1);
    assert_eq!(insights.publishers, 2,
               "publishers are counted ignoring case");
}

#[test]
fn publishers_are_listed_most_titles_first() {
    let items = [item("a", "Chaosium", 1.0),
                 item("b", "Paizo", 1.0),
                 item("c", "paizo", 1.0)];

    let by_publisher = Insights::compute(&items, SEPTEMBER).by_publisher;

    assert_eq!(by_publisher[0],
               Count { label: "Paizo".into(),
                       count: 2, });
    assert_eq!(by_publisher[1],
               Count { label: "Chaosium".into(),
                       count: 1, });
}

#[test]
fn a_title_is_counted_under_its_most_specific_system() {
    let items = [item("a", "Paizo", 1.0).with_filter("Rule System", "Pathfinder / Starfinder")
                                        .with_filter("Rule System", "Pathfinder OGL 1E"),
                 item("b", "Paizo", 1.0)];

    let insights = Insights::compute(&items, SEPTEMBER);

    assert_eq!(insights.by_system,
               [Count { label: "Pathfinder OGL 1E".into(),
                        count: 1, }]);
    assert_eq!(insights.systems, 1);
}

#[test]
fn disk_use_is_summed_per_publisher_largest_first() {
    let items = [item("a", "Paizo", 100.0),
                 item("b", "Chaosium", 300.0),
                 item("c", "paizo", 50.0)];

    let disk = Insights::compute(&items, SEPTEMBER).disk_by_publisher;

    assert_eq!(disk[0].label, "Chaosium");
    assert_eq!(disk[1].label, "Paizo");
    assert!((disk[1].mb - 150.0).abs() < f64::EPSILON);
}

#[test]
fn the_months_between_the_first_and_last_are_filled_in_with_zero() {
    let mut july = item("a", "Paizo", 1.0);
    july.date_added = Some(JULY);
    let mut september = item("b", "Paizo", 1.0);
    september.date_added = Some(SEPTEMBER);

    let insights = Insights::compute(&[july, september], SEPTEMBER);

    let counts: Vec<_> = insights.added_by_month
                                 .iter()
                                 .map(|m| (m.month, m.count))
                                 .collect();
    assert_eq!(counts, [(7, 1), (8, 0), (9, 1)]);
    assert_eq!(insights.added_this_month, 1);
}

#[test]
fn a_library_with_no_dates_has_no_months() {
    assert!(Insights::compute(&[item("a", "Paizo", 1.0)], SEPTEMBER).added_by_month
                                                                    .is_empty());
}

#[test]
fn the_csv_has_a_header_and_quotes_what_needs_it() {
    let items = [item("a", "Wizards, of the \"Coast\"", 1.0)];

    let csv = Insights::compute(&items, SEPTEMBER).to_csv();

    assert!(csv.starts_with("section,label,value\n"));
    assert!(csv.contains("glance,titles,1\n"));
    assert!(csv.contains("publisher,\"Wizards, of the \"\"Coast\"\"\",1\n"));
}
