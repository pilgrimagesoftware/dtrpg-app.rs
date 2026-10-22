//! The Placement rules section of the Storage page: the default location, the
//! rules in the order they are tried, and the editor that adds or changes one
//! (`storage_rule_editor_view`).

use std::collections::HashMap;
use std::sync::Arc;

use gpui::Window;
use gpui::prelude::*;
use gpui_kit::component::Icon;
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};

use super::storage_rule_editor_view::{rule_editor_row, value_placeholder};
use super::*;
use crate::controllers::settings::RuleEditor;
use crate::ui::views::settings_card::{card, card_row, section_heading};
use crate::view_models::storage_display::describe_rule;

/// Everything the rules section needs to draw itself.
pub(super) struct RulesData {
    /// The placement rules, in the order they are tried.
    pub rules:           Vec<PlacementRule>,
    /// The locations a rule can send a download to.
    pub roots:           Vec<StorageRoot>,
    /// Names and counts used to describe the rules.
    pub display:         StorageDisplay,
    /// The editor: adding a rule or changing one.
    pub editor:          RuleEditor,
    /// Publisher names the editor suggests from.
    pub publisher_names: Vec<String>,
}

/// Renders the Placement rules heading, the rules card, and the button that
/// adds a rule.
pub(super) fn render_storage_rules(data: RulesData, entity: Entity<SettingsController>,
                                   colors: &ColorTokens)
                                   -> impl IntoElement + 'static + use<> {
    let editor_open = data.editor.open;
    let rules_card = rules_card(&data, &entity, colors);
    let collection_names = data.display.collection_names.clone();
    div().flex()
         .flex_col()
         .gap(px(10.0))
         .child(section_heading(t!("settings.storage_rules_title").to_string(), colors))
         .child(rules_card)
         .child(div().flex()
                     .children((!editor_open).then(|| add_rule_button(entity, collection_names))))
}

/// The card: the default location, then a row for each rule. The editor takes
/// the place of the row it changes, or comes last when it adds a rule.
fn rules_card(data: &RulesData, entity: &Entity<SettingsController>, colors: &ColorTokens)
              -> impl IntoElement + 'static + use<> {
    let count = data.rules.len();
    let editor = &data.editor;
    let adding = editor.open && editor.editing.is_none();
    card(colors).child(default_location_row(&data.roots,
                                            data.display.default_root_id,
                                            entity,
                                            colors))
                .children(data.rules.iter().enumerate().map(|(index, rule)| {
                    if editor.open && editor.editing == Some(index) {
                        rule_editor_row(data, entity, colors).into_any_element()
                    }
                    else {
                        rule_row(index, count, rule, data, entity, colors).into_any_element()
                    }
                }))
                .children(adding.then(|| rule_editor_row(data, entity, colors)))
}

/// The row that picks where downloads go when no rule matches.
fn default_location_row(roots: &[StorageRoot], default_root_id: u32,
                        entity: &Entity<SettingsController>, colors: &ColorTokens)
                        -> impl IntoElement + 'static + use<> {
    let current = roots.iter()
                       .find(|root| root.id == default_root_id)
                       .map(|root| root.name.clone())
                       .unwrap_or_default();
    let choices: Vec<(u32, String)> = roots.iter()
                                           .map(|root| (root.id, root.name.clone()))
                                           .collect();
    let entity = entity.clone();
    card_row(true, colors).child(div().flex_1()
                                      .flex()
                                      .flex_col()
                                      .child(div().text_sm()
                                                  .font_weight(gpui::FontWeight::SEMIBOLD)
                                                  .text_color(colors.text_primary)
                                                  .child(t!("settings.storage_default_location").to_string()))
                                      .child(div().text_xs()
                                                  .text_color(colors.text_secondary)
                                                  .child(t!("settings.storage_default_location_note").to_string())))
                          .child(Button::new("default-location").outline()
                                                                .label(current)
                                                                .dropdown_caret(true)
                                                                .dropdown_menu(move |menu, _, _| {
                                                                    choices.iter().fold(menu, |menu, (id, name)| {
                                                                        let id = *id;
                                                                        let entity = entity.clone();
                                                                        menu.item(PopupMenuItem::new(name.clone())
                                                                                      .checked(id == default_root_id)
                                                                                      .on_click(move |_, _, cx| {
                                                                                          entity.update(cx, |ctrl, cx| {
                                                                                                    ctrl.set_default_root(id, cx);
                                                                                                });
                                                                                      }))
                                                                    })
                                                                }))
}

/// One rule: what it does and what kind it is, with buttons to move it up or
/// down (the order is the order rules are tried in) and to delete it.
fn rule_row(index: usize, count: usize, rule: &PlacementRule, data: &RulesData,
            entity: &Entity<SettingsController>, colors: &ColorTokens)
            -> impl IntoElement + 'static + use<> {
    let description = describe_rule(rule, &data.roots, &data.display.collection_names);
    let up = entity.clone();
    let down = entity.clone();
    let remove = entity.clone();
    let edit = entity.clone();
    let collection_names = data.display.collection_names.clone();
    card_row(false, colors).child(div().flex_1()
                                       .flex()
                                       .flex_col()
                                       .child(div().text_sm()
                                                   .font_weight(gpui::FontWeight::SEMIBOLD)
                                                   .text_color(colors.text_primary)
                                                   .child(description.title))
                                       .child(div().text_xs()
                                                   .text_color(colors.text_secondary)
                                                   .child(description.category)))
                           .child(Button::new(SharedString::from(format!("rule-edit-{index}")))
                                      .ghost()
                                      .icon(Icon::empty().path("icons/pencil.svg"))
                                      .tooltip(t!("settings.storage_rule_edit_tooltip").to_string())
                                      .on_click(move |_, window, cx| {
                                          open_editor(&edit, Some(index), &collection_names, window, cx);
                                      }))
                           .when(index > 0, |row| {
                               row.child(Button::new(SharedString::from(format!("rule-up-{index}")))
                                             .ghost()
                                             .icon(IconName::ChevronUp)
                                             .tooltip(t!("settings.storage_rule_move_up_tooltip").to_string())
                                             .on_click(move |_, _, cx| {
                                                 up.update(cx, |ctrl, cx| ctrl.move_storage_rule(index, false, cx));
                                             }))
                           })
                           .when(index + 1 < count, |row| {
                               row.child(Button::new(SharedString::from(format!("rule-down-{index}")))
                                             .ghost()
                                             .icon(IconName::ChevronDown)
                                             .tooltip(t!("settings.storage_rule_move_down_tooltip").to_string())
                                             .on_click(move |_, _, cx| {
                                                 down.update(cx, |ctrl, cx| ctrl.move_storage_rule(index, true, cx));
                                             }))
                           })
                           .child(Button::new(SharedString::from(format!("rule-remove-{index}")))
                                      .ghost()
                                      .icon(Icon::empty().path("icons/trash-2.svg"))
                                      .tooltip(t!("settings.storage_rule_remove_tooltip").to_string())
                                      .on_click(move |_, _, cx| {
                                          remove.update(cx, |ctrl, cx| ctrl.remove_storage_rule(index, cx));
                                      }))
}

/// The "Add rule…" button, which opens the editor on a new rule.
fn add_rule_button(entity: Entity<SettingsController>, collection_names: HashMap<u64, Arc<str>>)
                   -> impl IntoElement {
    Button::new("add-placement-rule").outline()
                                     .icon(IconName::Plus)
                                     .label(t!("settings.storage_add_rule").to_string())
                                     .on_click(move |_, window, cx| {
                                         open_editor(&entity, None, &collection_names, window, cx);
                                     })
}

/// Opens the editor on a new rule (`editing` is `None`) or on the rule at that
/// index, and puts the rule's value in the value field.
fn open_editor(entity: &Entity<SettingsController>, editing: Option<usize>,
               collection_names: &HashMap<u64, Arc<str>>, window: &mut Window,
               cx: &mut gpui::App) {
    let (text, input, kind) = entity.update(cx, |ctrl, cx| {
                                        let text =
                                            ctrl.open_rule_editor(editing, collection_names, cx);
                                        let editor = ctrl.rule_editor();
                                        (text, editor.input.clone(), editor.kind)
                                    });
    if let Some(input) = input {
        input.update(cx, |state, cx| {
                 state.set_value(text, window, cx);
                 state.set_placeholder(value_placeholder(kind), window, cx);
             });
    }
}
