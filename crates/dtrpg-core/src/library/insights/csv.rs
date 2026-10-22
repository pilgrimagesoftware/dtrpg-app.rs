//! The CSV form of the aggregates.

use super::Insights;

/// Renders `insights` as `section,label,value` rows under a header row.
pub(super) fn render(insights: &Insights) -> String {
    let mut rows: Vec<[String; 3]> = vec![["section".into(), "label".into(), "value".into()]];
    let mut push = |section: &str, label: &str, value: String| {
        rows.push([section.to_owned(), label.to_owned(), value]);
    };
    push("glance", "titles", insights.titles.to_string());
    push("glance",
         "total_size_mb",
         format!("{:.1}", insights.total_size_mb));
    push("glance", "pages", insights.total_pages.to_string());
    push("glance", "downloaded", insights.downloaded.to_string());
    push("glance", "publishers", insights.publishers.to_string());
    push("glance", "systems", insights.systems.to_string());
    push("glance",
         "added_this_month",
         insights.added_this_month.to_string());
    for entry in &insights.by_publisher {
        push("publisher", &entry.label, entry.count.to_string());
    }
    for entry in &insights.by_system {
        push("system", &entry.label, entry.count.to_string());
    }
    for entry in &insights.by_type {
        push("type", &entry.label, entry.count.to_string());
    }
    for entry in &insights.disk_by_publisher {
        push("disk_mb", &entry.label, format!("{:.1}", entry.mb));
    }
    for entry in &insights.added_by_month {
        push("added",
             &format!("{}-{:02}", entry.year, entry.month),
             entry.count.to_string());
    }
    for root in &insights.roots {
        push("root_used_mb", &root.name, number(root.used_mb));
        push("root_free_mb", &root.name, number(root.available_mb));
    }
    rows.iter()
        .map(|row| {
            row.iter()
               .map(|cell| quote(cell))
               .collect::<Vec<_>>()
               .join(",")
        })
        .collect::<Vec<_>>()
        .join("\n")
    + "\n"
}

/// A measurement with one decimal place, or nothing if it could not be taken.
fn number(value: Option<f64>) -> String {
    value.map_or_else(String::new, |mb| format!("{mb:.1}"))
}

/// `cell` as a CSV field: in quotes, with quotes inside doubled, when it holds
/// a comma, a quote or a line break.
fn quote(cell: &str) -> String {
    if cell.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", cell.replace('"', "\"\""))
    }
    else {
        cell.to_owned()
    }
}
