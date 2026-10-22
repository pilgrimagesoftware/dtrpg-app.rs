//! The Placement rules section of the Storage page: the default location, the
//! rules in the order they are tried, and the editor that adds one.

use gpui::prelude::*;
use gpui_kit::component::Icon;
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};

use super::*;
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
    /// Whether the "Add rule…" editor is showing.
    pub editor_open:     bool,
    /// Which kind of rule the editor is adding.
    pub editor_type:     RuleEditorType,
    /// The location the new rule will send downloads to.
    pub target_root_id:  u32,
    /// Publisher names offered as suggestions in the editor.
    pub publisher_names: Vec<String>,
    /// The editor's value fields, by rule type.
    pub inputs:          RuleInputs,
}

/// The value field of each kind of rule.
pub(super) struct RuleInputs {
    /// For a publisher rule.
    pub publisher:  Option<Entity<InputState>>,
    /// For a collection rule.
    pub collection: Option<Entity<InputState>>,
    /// For a size rule.
    pub size:       Option<Entity<InputState>>,
}

impl RuleInputs {
    /// The field for the kind of rule being added.
    fn for_type(&self, kind: RuleEditorType) -> Option<Entity<InputState>> {
        match kind {
            RuleEditorType::Publisher => self.publisher.clone(),
            RuleEditorType::Collection => self.collection.clone(),
            RuleEditorType::Size => self.size.clone(),
        }
    }
}

/// Renders the Placement rules heading, the rules card, and the button that
/// opens the editor.
pub(super) fn render_storage_rules(data: RulesData, entity: Entity<SettingsController>,
                                   colors: &ColorTokens)
                                   -> impl IntoElement + 'static + use<> {
    let editor_open = data.editor_open;
    let rules_card = rules_card(&data, &entity, colors);
    let editor = editor_open.then(|| rule_editor(&data, &entity, colors));
    div().flex()
         .flex_col()
         .gap(px(10.0))
         .child(section_heading(t!("settings.storage_rules_title").to_string(), colors))
         .child(rules_card)
         .children(editor)
         .child(div().flex().child(add_rule_button(editor_open, entity)))
}

/// The card: the default location, then a row for each rule.
fn rules_card(data: &RulesData, entity: &Entity<SettingsController>, colors: &ColorTokens)
              -> impl IntoElement + 'static + use<> {
    let count = data.rules.len();
    card(colors).child(default_location_row(&data.roots,
                                            data.display.default_root_id,
                                            entity,
                                            colors))
                .children(data.rules
                              .iter()
                              .enumerate()
                              .map(|(index, rule)| {
                                  rule_row(index, count, rule, data, entity, colors)
                              }))
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

/// The "Add rule…" button, which opens the editor, or closes it again.
fn add_rule_button(editor_open: bool, entity: Entity<SettingsController>) -> impl IntoElement {
    Button::new("add-placement-rule").outline()
                                     .icon(if editor_open {
                                               IconName::Close
                                           }
                                           else {
                                               IconName::Plus
                                           })
                                     .label(if editor_open {
                                                t!("settings.storage_cancel_rule").to_string()
                                            }
                                            else {
                                                t!("settings.storage_add_rule").to_string()
                                            })
                                     .on_click(move |_, _, cx| {
                                         entity.update(cx, |ctrl, cx| {
                                                   ctrl.set_rule_editor_open(!editor_open, cx)
                                               });
                                     })
}

/// The editor: which kind of rule, where it sends downloads, its value, and
/// the button that adds it.
fn rule_editor(data: &RulesData, entity: &Entity<SettingsController>, colors: &ColorTokens)
               -> impl IntoElement + 'static + use<> {
    let kind = data.editor_type;
    let input = data.inputs.for_type(kind);
    let add_entity = entity.clone();
    let add_input = input.clone();
    card(colors).gap(px(10.0))
                .p(px(16.0))
                .child(choice_row(t!("settings.storage_rule_type").to_string(),
                                  rule_type_buttons(kind, entity),
                                  colors))
                .child(choice_row(t!("settings.storage_rule_target").to_string(),
                                  target_buttons(&data.roots, data.target_root_id, entity),
                                  colors))
                .child(input.map_or_else(|| div().into_any_element(),
                                         |input| Input::new(&input).appearance(true).into_any_element()))
                .when(kind == RuleEditorType::Publisher, |editor| {
                    editor.child(publisher_suggestions(&data.publisher_names, data.inputs.publisher.clone()))
                })
                .child(div().flex().child(
                    Button::new("confirm-placement-rule").label(t!("settings.storage_add_rule_confirm").to_string())
                                                         .on_click(move |_, _, cx| {
                                                             let Some(input) = add_input.clone()
                                                             else {
                                                                 return;
                                                             };
                                                             let value = input.read(cx).value().to_string();
                                                             add_entity.update(cx, |ctrl, cx| match kind {
                                                                 RuleEditorType::Publisher => ctrl.add_publisher_rule(value, cx),
                                                                 RuleEditorType::Collection => ctrl.add_collection_rule(value, cx),
                                                                 RuleEditorType::Size => ctrl.add_size_rule(value, cx),
                                                             });
                                                         })))
}

/// A label with a row of buttons beside it.
fn choice_row(label: String, buttons: gpui::Div, colors: &ColorTokens) -> impl IntoElement {
    div().flex()
         .items_center()
         .gap(px(8.0))
         .child(div().w(px(80.0))
                     .text_xs()
                     .text_color(colors.text_secondary)
                     .child(label))
         .child(buttons)
}

/// One button per kind of rule, the chosen one selected.
fn rule_type_buttons(kind: RuleEditorType, entity: &Entity<SettingsController>) -> gpui::Div {
    let kinds = [(RuleEditorType::Publisher, t!("settings.storage_rule_kind_publisher")),
                 (RuleEditorType::Collection, t!("settings.storage_rule_kind_collection")),
                 (RuleEditorType::Size, t!("settings.storage_rule_kind_size"))];
    div().flex().gap(px(4.0)).children(kinds.into_iter().map(|(choice, label)| {
        let entity = entity.clone();
        Button::new(SharedString::from(format!("rule-type-{label}"))).ghost()
                                                                     .selected(choice == kind)
                                                                     .label(label.to_string())
                                                                     .on_click(move |_, _, cx| {
                                                                         entity.update(cx, |ctrl, cx| ctrl.set_rule_editor_type(choice, cx));
                                                                     })
    }))
}

/// One button per location, the chosen one selected.
fn target_buttons(roots: &[StorageRoot], target_root_id: u32,
                  entity: &Entity<SettingsController>)
                  -> gpui::Div {
    div().flex().gap(px(4.0)).children(roots.iter().map(|root| {
        let id = root.id;
        let entity = entity.clone();
        Button::new(SharedString::from(format!("rule-target-{id}"))).ghost()
                                                                    .selected(id == target_root_id)
                                                                    .label(root.name.clone())
                                                                    .on_click(move |_, _, cx| {
                                                                        entity.update(cx, |ctrl, cx| ctrl.set_rule_target_root(id, cx));
                                                                    })
    }))
}

/// The first few publisher names, each a button that fills the value field.
fn publisher_suggestions(names: &[String], input: Option<Entity<InputState>>) -> gpui::Div {
    div().flex().flex_wrap().gap(px(4.0)).children(names.iter().take(8).map(|name| {
        let input = input.clone();
        let value = name.clone();
        Button::new(SharedString::from(format!("publisher-suggestion-{name}"))).ghost()
                                                                               .label(name.clone())
                                                                               .on_click(move |_, window, cx| {
                                                                                   if let Some(input) = input.clone() {
                                                                                       input.update(cx, |state, cx| {
                                                                                                 state.set_value(value.clone(), window, cx);
                                                                                             });
                                                                                   }
                                                                               })
    }))
}
