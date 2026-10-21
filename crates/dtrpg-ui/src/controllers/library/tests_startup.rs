//! Tests that the catalog stored between runs is shown at once, and stays
//! shown when a load finds nothing to fetch.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use dtrpg_core::library::LibraryEvent;
use gpui::TestAppContext;

use super::tests_support::*;

fn three_items() -> Vec<dtrpg_core::data::library::LibraryItem> {
    (1..=3).map(|n| item_numbered(&format!("id{n}"), n))
           .collect()
}

#[gpui::test]
fn the_stored_catalog_is_shown_without_waiting_for_a_load(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               three_items());

    let (before, after) = fixture.controller.update(cx, |ctrl, _| {
                                                let before = ctrl.visible_items_count();
                                                ctrl.show_stored_catalog();
                                                (before, ctrl.visible_items_count())
                                            });

    assert_eq!(before, 0, "nothing builds the view until something asks");
    assert_eq!(after, 3);
}

#[gpui::test]
fn a_load_that_skips_the_fetch_leaves_the_stored_catalog_on_show(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               three_items());

    let (shown, loading) =
        fixture.controller.update(cx, |ctrl, cx| {
                              ctrl.catalog_loading = true;
                              ctrl.handle_sync_event(LibraryEvent::SkippedFreshCache, cx);
                              (ctrl.visible_items_count(), ctrl.is_catalog_loading())
                          });

    assert_eq!(shown, 3);
    assert!(!loading);
}
