//! Tests for the Needs Attention section.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use dtrpg_core::util::filter::SidebarFilter;
use gpui::TestAppContext;

use super::support::*;

fn items() -> Vec<dtrpg_core::data::library::LibraryItem> {
    (1..=3).map(|n| {
               let mut item = item_numbered(&format!("id{n}"), n);
               item.product_id = 100 + n;
               item
           })
           .collect()
}

#[gpui::test]
fn a_failed_download_shows_under_needs_attention_and_a_success_clears_it(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               items());

    let (failed, cleared) = fixture.controller.update(cx, |ctrl, cx| {
                                                  ctrl.show_stored_catalog();
                                                  ctrl.note_download_result(&"id2".into(), false);
                                                  ctrl.set_filter(SidebarFilter::NeedsAttention,
                                                                  cx);
                                                  let failed = ctrl.visible_items()
                                                                   .iter()
                                                                   .map(|i| i.id.to_string())
                                                                   .collect::<Vec<_>>();
                                                  ctrl.note_download_result(&"id2".into(), true);
                                                  ctrl.invalidate_cache();
                                                  (failed, ctrl.visible_items_count())
                                              });

    assert_eq!(failed, vec!["id2".to_owned()]);
    assert_eq!(cleared, 0);
}

#[gpui::test]
fn the_count_in_the_snapshot_is_the_number_of_failed_items(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               items());

    let count = fixture.controller.update(cx, |ctrl, _| {
                                      ctrl.note_download_result(&"id1".into(), false);
                                      ctrl.note_download_result(&"id3".into(), false);
                                      ctrl.snapshot().counts.needs_attention
                                  });

    assert_eq!(count, 2);
}
