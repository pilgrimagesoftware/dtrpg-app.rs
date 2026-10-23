//! Guards the render path: the state a frame reads from the controller must
//! never scan the catalog. GPUI re-renders on every keystroke, so a
//! whole-catalog read placed on this path runs per key, and every fix that
//! removed one was later re-added by a feature branch merged over it. This
//! test is the ratchet: reset core's scan counter, read one frame's worth of
//! view state, and expect nothing to have scanned. A failure names the call
//! site that scanned.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::hint::black_box;

use dtrpg_core::scan_counter::{catalog_scan_count, last_catalog_scanner, reset_catalog_scans};
use gpui::TestAppContext;

use super::tests_support::*;

/// One frame's reads, mirroring `root_view/render.rs`, `catalog_view/render.rs`
/// and the table delegates, and nothing that legitimately belongs to an event
/// handler.
fn read_one_frame(fixture: &Fixture, cx: &mut TestAppContext) {
    fixture.controller.update(cx, |ctrl, _| {
                          let snap = ctrl.snapshot();
                          black_box(ctrl.visible_items_count());
                          black_box(ctrl.is_view_loading());
                          black_box(ctrl.visible_items_slice(0..snap.matched_count.min(25)));
                          black_box(ctrl.item_activity_snapshot());
                          black_box(ctrl.library.checking_items_snapshot());
                          for collection in ctrl.smart_collections() {
                              black_box(ctrl.smart_collection_count(collection.id));
                          }
                      });
}

#[gpui::test]
fn a_frame_of_view_reads_does_not_scan_the_catalog(cx: &mut TestAppContext) {
    let items: Vec<_> = (1..=30).map(|n| item_numbered(&format!("id{n}"), n))
                                .collect();
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               4,
                               items);
    // The build `lifecycle` runs from the stored catalog before the first
    // frame; its own scans are legitimate and happen before the reset.
    fixture.controller
           .update(cx, |ctrl, _| ctrl.show_stored_catalog());

    reset_catalog_scans();
    for _ in 0..3 {
        read_one_frame(&fixture, cx);
    }

    let scans = catalog_scan_count();
    assert_eq!(scans,
               0,
               "the render path scanned the catalog {scans} times, the last from {}",
               last_catalog_scanner().map_or_else(|| "an unattributed query".to_owned(),
                                                  |caller| caller.to_string()));
}
