//! Tests for working out the Insights aggregates.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use dtrpg_core::util::filter::SidebarFilter;
use gpui::TestAppContext;

use super::support::*;

#[gpui::test]
fn opening_insights_works_the_aggregates_out_in_the_background(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               (1..=3).map(|n| item_numbered(&format!("id{n}"), n))
                                      .collect());

    fixture.controller.update(cx, |ctrl, cx| {
                          ctrl.set_filter(SidebarFilter::Insights, cx);
                      });
    let computing = fixture.controller
                           .update(cx, |ctrl, _| ctrl.is_insights_computing());
    cx.run_until_parked();

    let titles = fixture.controller.update(cx, |ctrl, _| {
                                       ctrl.insights().map(|insights| insights.titles)
                                   });
    assert!(computing, "the work starts at once, off the main thread");
    assert_eq!(titles, Some(3));
}
