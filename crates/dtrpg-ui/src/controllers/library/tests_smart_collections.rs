//! Tests that a smart collection counts its members, shows them in the
//! catalog, and goes away cleanly.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use dtrpg_core::data::smart_collection::{MatchMode, Operator, Rule, SmartCollection};
use dtrpg_core::util::filter::SidebarFilter;
use dtrpg_core::util::search::Field;
use gpui::TestAppContext;

use super::tests_support::*;

/// A collection of the items whose publisher is "Publisher", which is what the
/// fixture's items have unless a test changes it.
fn publisher_collection() -> SmartCollection {
    SmartCollection { name: "Paizo".to_owned(),
                      mode: MatchMode::All,
                      rules: vec![Rule { field:    Field::Publisher,
                                         operator: Operator::Is,
                                         value:    "Publisher".to_owned(), }],
                      ..SmartCollection::default() }
}

/// Two items, of which only the first has the publisher "Publisher".
fn two_items() -> Vec<dtrpg_core::data::library::LibraryItem> {
    let first = item_with_files("first", 11, 1);
    let mut second = item_with_files("second", 12, 1);
    second.publisher = Arc::from("Other");
    vec![first, second]
}

#[gpui::test]
fn a_new_smart_collection_counts_its_members(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               two_items());

    let (id, count) =
        fixture.controller.update(cx, |ctrl, cx| {
                              let id = ctrl.add_smart_collection(publisher_collection(), cx);
                              (id, ctrl.smart_collection_count(id))
                          });

    assert_ne!(id, 0);
    assert_eq!(count, 1);
}

#[gpui::test]
fn showing_a_smart_collection_limits_the_catalog_to_its_members(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               two_items());

    let shown =
        fixture.controller.update(cx, |ctrl, cx| {
                              let id = ctrl.add_smart_collection(publisher_collection(), cx);
                              ctrl.set_filter(SidebarFilter::SmartCollection(id, "Paizo".into()),
                                              cx);
                              ctrl.visible_items()
                                  .iter()
                                  .map(|item| item.id.to_string())
                                  .collect::<Vec<_>>()
                          });

    assert_eq!(shown, ["first"]);
}

#[gpui::test]
fn deleting_the_smart_collection_on_show_returns_to_all_titles(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               two_items());

    let (filter, shown) =
        fixture.controller.update(cx, |ctrl, cx| {
                              let id = ctrl.add_smart_collection(publisher_collection(), cx);
                              ctrl.set_filter(SidebarFilter::SmartCollection(id, "Paizo".into()),
                                              cx);
                              ctrl.delete_smart_collection(id, cx);
                              (ctrl.filter.clone(), ctrl.visible_items_count())
                          });

    assert_eq!(filter, SidebarFilter::AllTitles);
    assert_eq!(shown, 2);
}

#[gpui::test]
fn editing_the_rules_updates_the_count(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               two_items());

    let counts = fixture.controller.update(cx, |ctrl, cx| {
                                       let id =
                                           ctrl.add_smart_collection(publisher_collection(), cx);
                                       let mut edited = ctrl.smart_collections()[0].clone();
                                       edited.rules[0].value = "Nobody".to_owned();
                                       ctrl.update_smart_collection(edited, cx);
                                       (id, ctrl.smart_collection_count(id))
                                   });

    assert_eq!(counts.1, 0, "no item has that publisher any more");
}
