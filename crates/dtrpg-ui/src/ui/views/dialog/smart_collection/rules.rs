//! The rules part of the smart collection editor: the menus that pick what a
//! rule tests and how, and the row that holds them with the value box.

use dtrpg_core::data::smart_collection::{MatchMode, Operator};
use dtrpg_core::util::search::Field;
use gpui::prelude::*;
use gpui::{Entity, div, px};
use gpui_kit::component::IconName;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::ui::views::dialog::smart_collection::editor::{SharedState, update};
use crate::view_models::smart_collection_editor::RULE_FIELDS;

/// The label of a rule field in the field menu.
pub(super) fn field_label(field: Field) -> String {
    match field {
        Field::Tag => t!("smart_collections.editor.field_tag"),
        Field::Publisher => t!("smart_collections.editor.field_publisher"),
        Field::System => t!("smart_collections.editor.field_system"),
        Field::Type => t!("smart_collections.editor.field_type"),
        Field::Format => t!("smart_collections.editor.field_format"),
        Field::Genre => t!("smart_collections.editor.field_genre"),
        Field::Availability => t!("smart_collections.editor.field_availability"),
        Field::Location => t!("smart_collections.editor.field_location"),
        Field::Collection => t!("smart_collections.editor.field_collection"),
    }.to_string()
}

/// The menu button that picks what a rule tests.
pub(super) fn field_menu(index: usize, current: Field, state: &SharedState,
                         controller: &Entity<LibraryController>)
                         -> impl IntoElement {
    let (state, controller) = (state.clone(), controller.clone());
    Button::new(("smart-rule-field", index)).outline()
                                            .label(field_label(current))
                                            .dropdown_caret(true)
                                            .dropdown_menu(move |menu, _, _| {
                                                RULE_FIELDS.into_iter().fold(menu, |menu, field| {
                let (state, controller) = (state.clone(), controller.clone());
                menu.item(PopupMenuItem::new(field_label(field)).checked(field == current)
                                                                .on_click(move |_, _, cx| {
                                                                    state.borrow_mut()
                                                                         .draft
                                                                         .set_field(index, field);
                                                                    update(cx, &state, &controller);
                                                                }))
            })
                                            })
}

/// The menu button that picks "is" or "is not".
pub(super) fn operator_menu(index: usize, current: Operator, state: &SharedState,
                            controller: &Entity<LibraryController>)
                            -> impl IntoElement {
    let label = |operator| match operator {
        Operator::Is => t!("smart_collections.editor.op_is").to_string(),
        Operator::IsNot => t!("smart_collections.editor.op_is_not").to_string(),
    };
    let (state, controller) = (state.clone(), controller.clone());
    Button::new(("smart-rule-operator", index)).outline()
                                               .label(label(current))
                                               .dropdown_caret(true)
                                               .dropdown_menu(move |menu, _, _| {
                                                   [Operator::Is, Operator::IsNot].into_iter()
                                                                                  .fold(menu, |menu, operator| {
                let (state, controller) = (state.clone(), controller.clone());
                menu.item(PopupMenuItem::new(label(operator)).checked(operator == current)
                                                             .on_click(move |_, _, cx| {
                                                                 state.borrow_mut()
                                                                      .draft
                                                                      .set_operator(index, operator);
                                                                 update(cx, &state, &controller);
                                                             }))
            })
                                               })
}

/// The menu button that picks "all of" or "any of".
pub(super) fn mode_menu(current: MatchMode, state: &SharedState,
                        controller: &Entity<LibraryController>)
                        -> impl IntoElement {
    let label = |mode| match mode {
        MatchMode::All => t!("smart_collections.editor.match_all").to_string(),
        MatchMode::Any => t!("smart_collections.editor.match_any").to_string(),
    };
    let (state, controller) = (state.clone(), controller.clone());
    Button::new("smart-rule-mode").outline()
                                  .label(label(current))
                                  .dropdown_caret(true)
                                  .dropdown_menu(move |menu, _, _| {
                                      [MatchMode::All, MatchMode::Any].into_iter().fold(menu, |menu, mode| {
                let (state, controller) = (state.clone(), controller.clone());
                menu.item(PopupMenuItem::new(label(mode)).checked(mode == current)
                                                         .on_click(move |_, _, cx| {
                                                             state.borrow_mut().draft.set_mode(mode);
                                                             update(cx, &state, &controller);
                                                         }))
            })
                                  })
}

/// One rule: what it tests, how, the value, and a button to remove it.
pub(super) fn render_rule_row(index: usize, state: &SharedState,
                              controller: &Entity<LibraryController>)
                              -> impl IntoElement {
    let editor = state.borrow();
    let rule = &editor.draft.rules()[index];
    let value_input = editor.value_inputs[index].clone();
    let (remove_state, remove_controller) = (state.clone(), controller.clone());
    div().flex()
         .items_center()
         .gap(px(8.0))
         .child(field_menu(index, rule.field, state, controller))
         .child(operator_menu(index, rule.operator, state, controller))
         .child(div().flex_1().child(Input::new(&value_input)))
         .child(Button::new(("smart-rule-remove", index)).ghost()
                                                         .compact()
                                                         .icon(IconName::Close)
                                                         .tooltip(t!("smart_collections.editor.remove_rule_tooltip")
                                                                      .to_string())
                                                         .on_click(move |_, window, cx| {
                                                             {
                                                                 let mut editor =
                                                                     remove_state.borrow_mut();
                                                                 editor.draft.remove_rule(index);
                                                                 editor.value_inputs.remove(index);
                                                             }
                                                             update(cx, &remove_state, &remove_controller);
                                                             window.refresh();
                                                         }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rule_field_has_a_label() {
        for field in RULE_FIELDS {
            assert!(!field_label(field).starts_with("smart_collections."),
                    "{field:?} has no text");
        }
    }
}
