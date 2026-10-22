//! Tests that signing in keeps the stored library and signing out forgets it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use gpui::TestAppContext;

use super::tests_support::*;

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
                                       ctrl.library.all_items().len()
                                   });

    assert_eq!(stored, 3,
               "the stored items must not be cleared by signing in");
}

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
                                       ctrl.library.all_items().len()
                                   });

    assert_eq!(stored, 0);
}
