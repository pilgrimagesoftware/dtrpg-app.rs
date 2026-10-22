//! Tests for the placement rule editor's state. Saving a rule writes the real
//! storage settings file, so only what happens without a save is tested here;
//! turning text into a rule is tested in `view_models::rule_editor`.

use std::collections::HashMap;
use std::sync::Arc;

use gpui::{AppContext, TestAppContext};

use super::tests::UnusedLoginService;
use super::*;

#[gpui::test]
fn opening_the_editor_to_add_a_rule_starts_with_an_empty_value(cx: &mut TestAppContext) {
    let ctrl = cx.new(|_| SettingsController::for_test(Arc::new(UnusedLoginService)));

    let text = ctrl.update(cx, |ctrl, cx| {
                       ctrl.set_rule_value("left over".to_owned(), cx);
                       ctrl.open_rule_editor(None, &HashMap::new(), cx)
                   });

    assert!(text.is_empty());
    ctrl.read_with(cx, |ctrl, _| {
            let editor = ctrl.rule_editor();
            assert!(editor.open);
            assert_eq!(editor.editing, None);
            assert!(editor.value.is_empty());
        });
}

#[gpui::test]
fn the_kind_the_value_and_closing_are_kept_by_the_editor(cx: &mut TestAppContext) {
    let ctrl = cx.new(|_| SettingsController::for_test(Arc::new(UnusedLoginService)));

    ctrl.update(cx, |ctrl, cx| {
            ctrl.open_rule_editor(None, &HashMap::new(), cx);
            ctrl.set_rule_editor_type(RuleEditorType::Size, cx);
            ctrl.set_rule_value("250".to_owned(), cx);
        });

    ctrl.read_with(cx, |ctrl, _| {
            assert_eq!(ctrl.rule_editor().kind, RuleEditorType::Size);
            assert_eq!(ctrl.rule_editor().value, "250");
        });

    ctrl.update(cx, |ctrl, cx| ctrl.close_rule_editor(cx));

    ctrl.read_with(cx, |ctrl, _| assert!(!ctrl.rule_editor().open));
}

#[gpui::test]
fn saving_text_that_describes_no_rule_leaves_the_editor_open(cx: &mut TestAppContext) {
    let ctrl = cx.new(|_| SettingsController::for_test(Arc::new(UnusedLoginService)));

    let saved = ctrl.update(cx, |ctrl, cx| {
                        ctrl.open_rule_editor(None, &HashMap::new(), cx);
                        ctrl.set_rule_editor_type(RuleEditorType::Size, cx);
                        ctrl.set_rule_value("big".to_owned(), cx);
                        ctrl.save_rule(&HashMap::new(), cx)
                    });

    assert!(!saved);
    ctrl.read_with(cx, |ctrl, _| assert!(ctrl.rule_editor().open));
}
