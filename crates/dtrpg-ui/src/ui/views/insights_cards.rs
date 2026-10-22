//! The cards on the Insights page. Each takes the aggregates (already worked
//! out) and returns an element, so none reads a catalog or the disk.

use dtrpg_core::library::insights::{Insights, MonthCount, RootUsage, Share};
use gpui::{Hsla, IntoElement, ParentElement, SharedString, Styled, div, px};
use gpui_kit::component::chart::{AreaChart, PieChart};
use gpui_kit::component::group_box::GroupBox;
use gpui_kit::component::progress::Progress;
use rust_i18n::t;

use crate::controllers::library::LiveAvailability;
use crate::data::theme::ColorTokens;
use crate::util::pluralize::pluralize;
use crate::view_models::insights_display::{
    Availability, TOP_ROWS, availability_slices, month_label, top_rows,
};
use crate::view_models::storage_display::volume_size_label;

/// The colors the charts take in turn, as red, blue, green, ochre, purple,
/// teal, mauve and olive. Fixed rather than taken from the theme so a chart
/// keeps its meaning whichever theme is on.
const SERIES: [(u8, u8, u8); 8] = [(0xA9, 0x5D, 0x54),
                                   (0x59, 0x81, 0xB4),
                                   (0x56, 0x8B, 0x5A),
                                   (0xB8, 0x87, 0x2F),
                                   (0x7A, 0x63, 0xA3),
                                   (0x4A, 0x90, 0x96),
                                   (0x8E, 0x5A, 0x7A),
                                   (0x85, 0x92, 0x4F)];

/// The side of the square a ring is drawn in, in pixels.
const RING_SIZE: f32 = 130.0;

/// The height of the time chart, in pixels.
const TIME_CHART_HEIGHT: f32 = 140.0;

/// The `index`th series color, starting over after the last.
fn series(index: usize) -> Hsla {
    let (r, g, b) = SERIES[index % SERIES.len()];
    gpui::rgb((u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)).into()
}

/// A card: a titled box with a note under the title.
fn card(id: &'static str, title: String, note: String, colors: &ColorTokens,
        body: impl IntoElement)
        -> impl IntoElement {
    GroupBox::new().id(id)
                   .title(div().flex()
                               .flex_col()
                               .child(div().text_sm()
                                           .font_weight(gpui::FontWeight::SEMIBOLD)
                                           .text_color(colors.text_primary)
                                           .child(title))
                               .child(div().text_xs()
                                           .text_color(colors.text_secondary)
                                           .child(note)))
                   .child(div().pt(px(8.0)).child(body))
}

/// A figure with its caption under it, and an optional line under that.
fn stat(value: String, caption: String, extra: Option<String>, colors: &ColorTokens)
        -> impl IntoElement {
    div().flex()
         .flex_col()
         .child(div().text_2xl()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(value))
         .child(div().text_xs()
                     .text_color(colors.text_secondary)
                     .child(caption))
         .children(extra.map(|line| div().text_xs().text_color(colors.accent).child(line)))
}

/// The figures at the top: titles, size, pages, downloaded, publishers,
/// systems.
pub(super) fn glance_card(insights: &Insights, colors: &ColorTokens) -> impl IntoElement {
    let added = (insights.added_this_month > 0).then(|| {
                                                   t!("insights.added_this_month",
                                                      count = insights.added_this_month).to_string()
                                               });
    let stats = div().flex()
                     .flex_wrap()
                     .gap(px(32.0))
                     .child(stat(insights.titles.to_string(),
                                 t!("insights.stat_titles").to_string(),
                                 added,
                                 colors))
                     .child(stat(volume_size_label(insights.total_size_mb),
                                 t!("insights.stat_size").to_string(),
                                 None,
                                 colors))
                     .child(stat(insights.total_pages.to_string(),
                                 t!("insights.stat_pages").to_string(),
                                 None,
                                 colors))
                     .child(stat(format!("{}/{}", insights.downloaded, insights.titles),
                                 t!("insights.stat_downloaded").to_string(),
                                 None,
                                 colors))
                     .child(stat(insights.publishers.to_string(),
                                 t!("insights.stat_publishers").to_string(),
                                 None,
                                 colors))
                     .child(stat(insights.systems.to_string(),
                                 t!("insights.stat_systems").to_string(),
                                 None,
                                 colors));
    card("insights-glance",
         t!("insights.glance_title").to_string(),
         t!("insights.glance_note").to_string(),
         colors,
         stats)
}

/// Rows of a label, a bar and a count, for a "top" card.
fn bar_rows(counts: &[dtrpg_core::library::insights::Count], colors: &ColorTokens)
            -> impl IntoElement {
    let (rows, _) = top_rows(counts, TOP_ROWS);
    div().flex()
         .flex_col()
         .gap(px(8.0))
         .children(rows.into_iter().enumerate().map(|(index, row)| {
             div().flex()
                  .items_center()
                  .gap(px(8.0))
                  .child(div().w(px(150.0))
                              .truncate()
                              .text_sm()
                              .text_color(colors.text_primary)
                              .child(row.label.clone()))
                  .child(div().flex_1()
                              .child(Progress::new(("insights-bar", index)).value(row.fraction * 100.0)
                                                                           .color(series(index))))
                  .child(div().w(px(32.0))
                              .text_right()
                              .text_sm()
                              .text_color(colors.text_secondary)
                              .child(row.count.to_string()))
         }))
}

/// "Titles by publisher" or "Titles by system": the top rows of `counts`.
pub(super) fn top_card(id: &'static str, title_key: &str, note: Option<String>,
                       counts: &[dtrpg_core::library::insights::Count], colors: &ColorTokens)
                       -> impl IntoElement {
    let (_, all) = top_rows(counts, TOP_ROWS);
    let note = note.unwrap_or_else(|| {
                       t!("insights.top_of",
                          shown = counts.len().min(TOP_ROWS),
                          all = all).to_string()
                   });
    card(id,
         t!(title_key).to_string(),
         note,
         colors,
         bar_rows(counts, colors))
}

/// A ring with a legend of `(color, label, count)` beside it.
fn ring_with_legend(id: &'static str, slices: Vec<(Hsla, String, usize)>, colors: &ColorTokens)
                    -> impl IntoElement {
    let legend =
        div().flex().flex_col().gap(px(4.0)).children(slices.iter()
                                                            .map(|(color, label, count)| {
                                                                div().flex()
                                                      .items_center()
                                                      .gap(px(8.0))
                                                      .child(div().size(px(10.0))
                                                                  .rounded(px(2.0))
                                                                  .bg(*color))
                                                      .child(div().w(px(120.0))
                                                                  .text_sm()
                                                                  .text_color(colors.text_primary)
                                                                  .child(label.clone()))
                                                      .child(div().text_sm()
                                                                  .text_color(colors.text_secondary)
                                                                  .child(count.to_string()))
                                                            }));
    let ring = PieChart::new(slices.into_iter()
                                   .filter(|(_, _, count)| *count > 0)
                                   .collect::<Vec<_>>()).id(id)
                                                        .value(|slice| slice.2 as f32)
                                                        .color(|slice| slice.0)
                                                        .inner_radius(RING_SIZE * 0.3)
                                                        .outer_radius(RING_SIZE * 0.48);
    div().flex()
         .items_center()
         .gap(px(16.0))
         .child(div().size(px(RING_SIZE)).child(ring))
         .child(legend)
}

/// "By type": a ring of the product types.
pub(super) fn type_card(insights: &Insights, colors: &ColorTokens) -> impl IntoElement {
    let slices = insights.by_type
                         .iter()
                         .take(TOP_ROWS)
                         .enumerate()
                         .map(|(index, entry)| (series(index), entry.label.clone(), entry.count))
                         .collect();
    card("insights-type",
         t!("insights.type_title").to_string(),
         t!("insights.type_note").to_string(),
         colors,
         ring_with_legend("insights-type-ring", slices, colors))
}

/// "Availability": a ring of what is on this machine and what is not.
pub(super) fn availability_card(insights: &Insights, live: LiveAvailability,
                                colors: &ColorTokens)
                                -> impl IntoElement {
    let slices =
        availability_slices(insights, live).into_iter()
                                           .enumerate()
                                           .map(|(index, (state, count))| {
                                               let key = match state {
                                                   Availability::OnDevice => "insights.on_device",
                                                   Availability::NotDownloaded => {
                                                       "insights.not_downloaded"
                                                   }
                                                   Availability::Downloading => {
                                                       "insights.downloading"
                                                   }
                                                   Availability::Failed => "insights.failed",
                                               };
                                               (series(index), t!(key).to_string(), count)
                                           })
                                           .collect();
    card("insights-availability",
         t!("insights.availability_title").to_string(),
         t!("insights.availability_note").to_string(),
         colors,
         ring_with_legend("insights-availability-ring", slices, colors))
}

/// "Disk usage by publisher": each publisher's share of the total, largest
/// first.
pub(super) fn disk_card(insights: &Insights, colors: &ColorTokens) -> impl IntoElement {
    let largest = insights.disk_by_publisher
                          .first()
                          .map_or(1.0, |share| share.mb.max(1.0));
    let rows = insights.disk_by_publisher
                       .iter()
                       .take(TOP_ROWS)
                       .enumerate()
                       .map(|(index, Share { label, mb })| {
                           div().flex()
                 .items_center()
                 .gap(px(8.0))
                 .child(div().w(px(150.0))
                             .truncate()
                             .text_sm()
                             .text_color(colors.text_primary)
                             .child(label.clone()))
                 .child(div().flex_1()
                             .child(Progress::new(("insights-disk", index))
                                        .value((mb / largest * 100.0) as f32)
                                        .color(series(index))))
                 .child(div().w(px(80.0))
                             .text_right()
                             .text_sm()
                             .text_color(colors.text_secondary)
                             .child(volume_size_label(*mb)))
                       });
    card("insights-disk",
         t!("insights.disk_title").to_string(),
         t!("insights.disk_note",
            total = volume_size_label(insights.total_size_mb)).to_string(),
         colors,
         div().flex().flex_col().gap(px(8.0)).children(rows))
}

/// "Titles added over time": an area chart by month.
pub(super) fn time_card(months: &[MonthCount], colors: &ColorTokens) -> impl IntoElement {
    let line = series(0);
    let chart = AreaChart::new(months.to_vec()).id("insights-time-chart")
                                               .x(|month| SharedString::from(month_label(month)))
                                               .y(|month| month.count as f64)
                                               .stroke(line)
                                               .fill(line.opacity(0.2));
    card("insights-time",
         t!("insights.time_title").to_string(),
         t!("insights.time_note").to_string(),
         colors,
         div().h(px(TIME_CHART_HEIGHT)).w_full().child(chart))
}

/// "Storage locations": what Libri uses on each, against the space free there.
pub(super) fn roots_card(roots: &[RootUsage], colors: &ColorTokens) -> impl IntoElement {
    let rows = roots.iter().enumerate().map(|(index, root)| {
                                            let used = root.used_mb.unwrap_or(0.0);
                                            let fraction = root.total_mb
                                                               .filter(|total| *total > 0.0)
                                                               .map_or(0.0, |total| used / total);
                                            let line =
                                                root.available_mb.map(|free| {
                                                                     t!("insights.root_line",
                                                                        used = volume_size_label(used),
                                                                        free = volume_size_label(free))
                                                                         .to_string()
                                                                 });
                                            div().flex()
                                                 .flex_col()
                                                 .gap(px(2.0))
                                                 .child(div().text_sm()
                                                             .text_color(colors.text_primary)
                                                             .child(root.name.clone()))
                                                 .child(Progress::new(("insights-root", index))
                                                            .value(fraction as f32 * 100.0)
                                                            .color(series(index)))
                                                 .children(line.map(|line| {
                                                     div().text_xs()
                                                          .text_color(colors.text_secondary)
                                                          .child(line)
                                                 }))
                                        });
    card("insights-roots",
         t!("insights.roots_title").to_string(),
         t!("insights.roots_note").to_string(),
         colors,
         div().flex().flex_col().gap(px(12.0)).children(rows))
}

/// "N titles analyzed" for the page's header.
pub(super) fn analyzed_label(titles: usize) -> String {
    t!("insights.analyzed",
       titles = pluralize(titles, "count.title", "count.titles")).to_string()
}
