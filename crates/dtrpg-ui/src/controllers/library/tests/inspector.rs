//! Tests for the inspector's summary tab in the library controller. The index
//! is real, in the fixture's temporary directory.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use dtrpg_core::data::index::LibraryIndex;
use dtrpg_core::data::summary::ContentSummary;
use gpui::TestAppContext;
use crate::view_models::inspector::{InspectorSummaryState};

use super::{support::*};

/// A controller whose catalog is items `a` and `b`, both of them in the library
/// index too.
fn fixture_with_index(cx: &mut TestAppContext) -> Fixture {
    let items = vec![item_numbered("a", 1), item_numbered("b", 2)];
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               items.clone());
    LibraryIndex::open_and_migrate(&fixture.cache).unwrap()
                                                  .replace_all_items(&items)
                                                  .unwrap();
    fixture
}

fn summary_of(fixture: &Fixture, cx: &mut TestAppContext, id: &str) -> InspectorSummaryState {
    fixture.controller
           .read_with(cx, |ctrl, _| ctrl.inspector_summary_for(id))
}

#[gpui::test]
fn a_summary_that_was_not_asked_for_reads_as_loading(cx: &mut TestAppContext) {
    let fixture = fixture_with_index(cx);

    assert_eq!(summary_of(&fixture, cx, "a"),
               InspectorSummaryState::Loading);
}

#[gpui::test]
fn the_stored_summary_shows_once_it_is_read(cx: &mut TestAppContext) {
    let fixture = fixture_with_index(cx);
    let stored = ContentSummary { item_id:     "a".to_owned(),
                                  text:        "A heist.".to_owned(),
                                  model:       "local".to_owned(),
                                  created_at:  1_700_000_000,
                                  token_count: 2, };
    LibraryIndex::open_and_migrate(&fixture.cache).unwrap()
                                                  .save_summary(&stored)
                                                  .unwrap();

    fixture.controller
           .update(cx, |ctrl, cx| ctrl.load_inspector_summary("a".into(), cx));
    wait_for(cx, |cx| {
        summary_of(&fixture, cx, "a") != InspectorSummaryState::Loading
    });

    assert_eq!(summary_of(&fixture, cx, "a"),
               InspectorSummaryState::Loaded(stored));
}

#[gpui::test]
fn an_item_with_no_summary_reads_as_missing(cx: &mut TestAppContext) {
    let fixture = fixture_with_index(cx);

    fixture.controller
           .update(cx, |ctrl, cx| ctrl.load_inspector_summary("b".into(), cx));
    wait_for(cx, |cx| {
        summary_of(&fixture, cx, "b") != InspectorSummaryState::Loading
    });

    assert_eq!(summary_of(&fixture, cx, "b"),
               InspectorSummaryState::Missing);
}
