//! Tests that signing in keeps the stored library and signing out forgets it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use dtrpg_core::data::collection::entry::CollectionEntry;
use gpui::TestAppContext;

use super::support::*;

#[gpui::test]
fn signing_in_keeps_the_stored_library(cx: &mut TestAppContext) {
    let items = (1..=3).map(|n| item_numbered(&format!("id{n}"), n))
                       .collect();
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               items);

    let stored = fixture.controller.update(cx, |ctrl, cx| {
                                       ctrl.install_services(fixture.library_service(),
                                                             fixture.collections_service(),
                                                             cx);
                                       ctrl.library.all_items_scan().len()
                                   });

    assert_eq!(stored, 3,
               "the stored items must not be cleared by signing in");
}
#[gpui::test]
fn signing_in_keeps_the_collections_on_show(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               0,
                               Vec::new());

    let (count, loaded) = fixture.controller.update(cx, |ctrl, cx| {
                                                ctrl.library
                                  .set_collections(vec![CollectionEntry { id:         7,
                                                                          name:
                                                                              "Cached".into(),
                                                                          member_ids: Arc::from([]),
                                                                          created_at: 0, }]);
                                                ctrl.install_services(fixture.library_service(),
                                                    fixture.collections_service(),
                                                    cx);
                                                (ctrl.collections().len(),
                                                 ctrl.library.collections_loaded())
                                            });

    assert_eq!((count, loaded),
               (1, true),
               "the sidebar must not empty while the next load runs");
}
#[ignore]
#[gpui::test]
fn signing_out_forgets_the_stored_library(cx: &mut TestAppContext) {
    let items = (1..=3).map(|n| item_numbered(&format!("id{n}"), n))
                       .collect();
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               items);

    let stored = fixture.controller.update(cx, |ctrl, cx| {
                                       ctrl.forget_library(cx);
                                       ctrl.library.all_items_scan().len()
                                   });

    assert_eq!(stored, 0);
}
