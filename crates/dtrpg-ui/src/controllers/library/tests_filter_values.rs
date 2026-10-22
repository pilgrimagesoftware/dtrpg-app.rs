//! Tests that the settings page lists the catalog's filter values, split into
//! shown and hidden.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use dtrpg_core::util::search::Field;
use gpui::TestAppContext;

use super::tests_support::*;

#[gpui::test]
fn the_sections_list_the_values_the_catalog_has_with_the_noise_hidden(cx: &mut TestAppContext) {
    let items = vec![item_numbered("a", 1).with_filter("Format", "PDF")
                                          .with_filter("Format", "Human-Created Without AI")
                                          .with_filter("Genre", "Horror"),
                     item_numbered("b", 2).with_filter("Format", "Audio")];
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               items);

    let sections = fixture.controller.update(cx, |ctrl, _| {
                                         ctrl.refresh_known_filter_values();
                                         ctrl.filter_value_sections()
                                     });

    let format = sections.iter()
                         .find(|section| section.field == Field::Format)
                         .unwrap();
    assert_eq!(format.shown, ["Audio", "PDF"]);
    assert_eq!(format.hidden, ["Human-Created Without AI"]);
    let genre = sections.iter()
                        .find(|section| section.field == Field::Genre)
                        .unwrap();
    assert_eq!(genre.shown, ["Horror"]);
    assert!(sections.iter().all(|section| section.field != Field::Type),
            "no values, no section");
}

#[gpui::test]
fn a_catalog_with_no_filters_has_no_sections(cx: &mut TestAppContext) {
    let fixture = fixture_with(cx,
                               ScriptedTransport::open(),
                               ScriptedLibrary::open(),
                               3,
                               vec![item_numbered("a", 1)]);

    let sections = fixture.controller.update(cx, |ctrl, _| {
                                         ctrl.refresh_known_filter_values();
                                         ctrl.filter_value_sections()
                                     });

    assert!(sections.is_empty());
}
