//! The placement rule editor: one compact row with the kind of rule, its value,
//! where it sends downloads, and save and cancel. It adds a rule, or, shown in
//! place of a rule's row, changes that rule.
//!
//! While a name is typed, the publishers or collections that match it are
//! listed under the row; picking one fills the value.

use gpui::prelude::*;
use gpui::{AnyElement, ClickEvent, Window, div, px};
use gpui_kit::component::IconName;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_kit::component::{Disableable, v_flex};
use rust_i18n::t;

use super::storage_rules_view::RulesData;
use super::*;
use crate::ui::views::settings_card::card_row;
use crate::view_models::rule_editor::{RuleEditorType, condition_from_text, suggestions};

/// How the kind of rule reads on the type menu.
pub(super) fn kind_label(kind: RuleEditorType) -> String {
    match kind {
        RuleEditorType::Publisher => t!("settings.storage_rule_kind_publisher"),
        RuleEditorType::Collection => t!("settings.storage_rule_kind_collection"),
        RuleEditorType::Size => t!("settings.storage_rule_kind_size"),
    }.to_string()
}

/// What the value field asks for, for the kind of rule.
pub(super) fn value_placeholder(kind: RuleEditorType) -> String {
    match kind {
        RuleEditorType::Publisher => t!("settings.storage_rule_value_publisher"),
        RuleEditorType::Collection => t!("settings.storage_rule_value_collection"),
        RuleEditorType::Size => t!("settings.storage_rule_value_size"),
    }.to_string()
}

/// The editor row, with the matching names under it.
pub(super) fn rule_editor_row(data: &RulesData, entity: &Entity<SettingsController>,
                              colors: &ColorTokens)
                              -> impl IntoElement + 'static + use<> {
    let editor = &data.editor;
    let names = &data.display.collection_names;
    let matches = suggestions(editor.kind, &editor.value, &data.publisher_names, names);
    let can_save = condition_from_text(editor.kind, &editor.value, names).is_some();

    let value: AnyElement =
        editor.input
              .as_ref()
              .map_or_else(|| div().into_any_element(),
                           |input| Input::new(input).appearance(true).into_any_element());
    let row = card_row(false, colors).child(kind_menu(data, entity))
                                     .child(div().flex_1().min_w_0().child(value))
                                     .child(target_menu(data, entity))
                                     .child(save_button(can_save, data, entity))
                                     .child(cancel_button(entity));
    v_flex().child(row)
            .children((!matches.is_empty()).then(|| suggestion_list(matches, data, colors)))
}

/// The menu that picks the kind of rule. Changing the kind empties the value
/// field, because a publisher's name means nothing as a size.
fn kind_menu(data: &RulesData, entity: &Entity<SettingsController>) -> impl IntoElement {
    let current = data.editor.kind;
    let input = data.editor.input.clone();
    let entity = entity.clone();
    Button::new("rule-editor-kind").outline()
                                   .label(kind_label(current))
                                   .dropdown_caret(true)
                                   .dropdown_menu(move |menu, _, _| {
                                       RuleEditorType::ALL.into_iter().fold(menu, |menu, kind| {
                                           let (entity, input) = (entity.clone(), input.clone());
                                           menu.item(PopupMenuItem::new(kind_label(kind))
                                                         .checked(kind == current)
                                                         .on_click(move |_, window, cx| {
                                                             change_kind(kind, &entity, input.as_ref(), window, cx);
                                                         }))
                                       })
                                   })
}

/// Switches the editor to `kind`: the controller learns it, and the value field
/// is emptied and asks for the new kind of value.
fn change_kind(kind: RuleEditorType, entity: &Entity<SettingsController>,
               input: Option<&Entity<InputState>>, window: &mut Window, cx: &mut gpui::App) {
    entity.update(cx, |ctrl, cx| ctrl.set_rule_editor_type(kind, cx));
    if let Some(input) = input {
        input.update(cx, |state, cx| {
                 state.set_value("", window, cx);
                 state.set_placeholder(value_placeholder(kind), window, cx);
             });
    }
}

/// The menu that picks where the rule sends downloads.
fn target_menu(data: &RulesData, entity: &Entity<SettingsController>) -> impl IntoElement {
    let current = data.editor.target_root_id;
    let label = data.roots
                    .iter()
                    .find(|root| root.id == current)
                    .map(|root| root.name.clone())
                    .unwrap_or_default();
    let choices: Vec<(u32, String)> = data.roots
                                          .iter()
                                          .map(|root| (root.id, root.name.clone()))
                                          .collect();
    let entity = entity.clone();
    Button::new("rule-editor-target").outline()
                                     .label(label)
                                     .dropdown_caret(true)
                                     .dropdown_menu(move |menu, _, _| {
                                         choices.iter().fold(menu, |menu, (id, name)| {
                                             let (id, entity) = (*id, entity.clone());
                                             menu.item(PopupMenuItem::new(name.clone())
                                                           .checked(id == current)
                                                           .on_click(move |_, _, cx| {
                                                               entity.update(cx, |ctrl, cx| {
                                                                         ctrl.set_rule_target_root(id, cx);
                                                                     });
                                                           }))
                                         })
                                     })
}

/// The button that saves the rule. It is off until the value describes a rule.
fn save_button(can_save: bool, data: &RulesData, entity: &Entity<SettingsController>)
               -> impl IntoElement {
    let entity = entity.clone();
    let names = data.display.collection_names.clone();
    Button::new("rule-editor-save").primary()
                                   .icon(IconName::Check)
                                   .tooltip(t!("settings.storage_rule_save_tooltip").to_string())
                                   .disabled(!can_save)
                                   .on_click(move |_: &ClickEvent, _, cx| {
                                       entity.update(cx, |ctrl, cx| ctrl.save_rule(&names, cx));
                                   })
}

/// The button that closes the editor without saving.
fn cancel_button(entity: &Entity<SettingsController>) -> impl IntoElement {
    let entity = entity.clone();
    Button::new("rule-editor-cancel").ghost()
                                     .icon(IconName::Close)
                                     .tooltip(t!("settings.storage_rule_cancel_tooltip").to_string())
                                     .on_click(move |_, _, cx| {
                                         entity.update(cx, |ctrl, cx| ctrl.close_rule_editor(cx));
                                     })
}

/// The names that match what is typed; picking one fills the value field.
fn suggestion_list(matches: Vec<String>, data: &RulesData, colors: &ColorTokens)
                   -> impl IntoElement {
    let input = data.editor.input.clone();
    v_flex().border_t_1()
            .border_color(colors.border)
            .children(matches.into_iter().enumerate().map(|(index, name)| {
                let input = input.clone();
                let picked = name.clone();
                div().id(("rule-editor-suggestion", index))
                     .px(px(16.0))
                     .py(px(6.0))
                     .text_sm()
                     .text_color(colors.text_primary)
                     .cursor_pointer()
                     .hover(|row| row.bg(colors.surface_alt))
                     .on_click(move |_, window, cx| {
                         if let Some(input) = input.clone() {
                             input.update(cx, |state, cx| state.set_value(picked.clone(), window, cx));
                         }
                     })
                     .child(name)
            }))
}
