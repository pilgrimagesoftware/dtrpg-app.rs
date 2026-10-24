//! Tests that typed search settles once: the keystroke stream applies only
//! its last query, and programmatic paths keep applying immediately.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use gpui::TestAppContext;

use super::tests_support::*;

fn titled(cx: &mut TestAppContext, count: u64) -> Fixture {
    let items: Vec<_> = (1..=count).map(|n| {
                                       let mut item = item_numbered(&format!("id{n}"), n);
                                       item.title = format!("Title {n}").into();
                                       item
                                   })
                                   .collect();
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               4,
                               items);
    fixture.controller
           .update(cx, |ctrl, _| ctrl.show_stored_catalog());
    fixture
}

#[gpui::test]
fn a_burst_of_keystrokes_applies_only_the_last_query(cx: &mut TestAppContext) {
    let fixture = titled(cx, 5);

    fixture.controller.update(cx, |ctrl, cx| {
                          for i in 1..=4 {
                              let partial = (1..=i).map(|k| format!("Title {k}"))
                                                   .collect::<Vec<_>>()
                                                   .join(" ");
                              ctrl.set_search_query_debounced(partial, cx);
                          }
                          ctrl.set_search_query_debounced("Title 5".to_owned(), cx);
                      });
    cx.executor()
      .advance_clock(super::filtering::SEARCH_SETTLE_DELAY);
    cx.run_until_parked();

    let (query, visible) = fixture.controller
                                  .update(cx, |ctrl, _| {
                                      (ctrl.search_query.clone(), ctrl.visible_items_count())
                                  });
    assert_eq!(query, "Title 5",
               "the settled query is the burst's last one");
    assert_eq!(visible, 1, "the partial queries never filtered the catalog");
}

#[gpui::test]
fn nothing_applies_before_the_burst_has_settled(cx: &mut TestAppContext) {
    let fixture = titled(cx, 5);

    fixture.controller.update(cx, |ctrl, cx| {
                          ctrl.set_search_query_debounced("Title 3".to_owned(), cx);
                      });
    cx.executor()
      .advance_clock(super::filtering::SEARCH_SETTLE_DELAY / 2);
    cx.run_until_parked();

    let query = fixture.controller
                       .update(cx, |ctrl, _| ctrl.search_query.clone());
    assert!(query.is_empty(),
            "the query waits out the delay before applying");
}

#[gpui::test]
fn clearing_during_the_delay_does_not_revive_the_typed_query(cx: &mut TestAppContext) {
    let fixture = titled(cx, 5);

    fixture.controller.update(cx, |ctrl, cx| {
                          ctrl.set_search_query_debounced("Title 5".to_owned(), cx);
                          ctrl.clear_search_query(cx);
                      });
    cx.executor()
      .advance_clock(super::filtering::SEARCH_SETTLE_DELAY * 2);
    cx.run_until_parked();

    let (query, visible) = fixture.controller
                                  .update(cx, |ctrl, _| {
                                      (ctrl.search_query.clone(), ctrl.visible_items_count())
                                  });
    assert!(query.is_empty(), "the settle task was retired by the clear");
    assert_eq!(visible, 5);
}
