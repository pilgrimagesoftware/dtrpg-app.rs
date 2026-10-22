//! The smart collection editor: a name, the rules, a live count of the titles
//! that match, and the on-match switches.
//!
//! The choices live in a [`SmartCollectionDraft`]; this file draws them and
//! passes each change to it. The text boxes keep their own text, so the
//! draft is brought up to date from them whenever one changes, and the match
//! count is worked out then, not while drawing: it reads the whole catalog.

use std::cell::RefCell;
use std::rc::Rc;

use dtrpg_core::data::smart_collection::{MatchMode, Operator};
use dtrpg_core::util::search::Field;
use gpui::prelude::*;
use gpui::{App, Entity, SharedString, Window, div, px};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::dialog::{DialogContent, DialogFooter, DialogHeader, DialogTitle};
use gpui_kit::component::input::{Input, InputEvent, InputState};
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{Disableable, IconName, WindowExt as _, v_flex};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::theme::{ColorTokens, LibriTheme};
use crate::view_models::smart_collection_editor::{RULE_FIELDS, SmartCollectionDraft};

/// Everything the editor keeps while it is open.
struct EditorState {
    /// The collection being edited.
    draft:        SmartCollectionDraft,
    /// The box for the name.
    name_input:   Entity<InputState>,
    /// The box for each rule's value, in the same order as the rules.
    value_inputs: Vec<Entity<InputState>>,
    /// How many titles match now, of how many there are.
    preview:      (usize, usize),
}

/// The editor's state, shared by the dialog and the callbacks that change it.
type SharedState = Rc<RefCell<EditorState>>;

/// Opens the editor on `draft`. Saving adds the collection, or replaces the
/// one with the draft's id.
pub fn open_smart_collection_editor(window: &mut Window, cx: &mut App,
                                    controller: Entity<LibraryController>,
                                    draft: SmartCollectionDraft) {
    let name_input = cx.new(|cx| {
                           InputState::new(window, cx)
            .placeholder(t!("smart_collections.editor.name_placeholder").to_string())
            .default_value(draft.name().to_owned())
                       });
    let value_inputs = draft.rules()
                            .iter()
                            .map(|rule| new_value_input(window, cx, &rule.value))
                            .collect();
    let state = Rc::new(RefCell::new(EditorState { draft,
                                                   name_input: name_input.clone(),
                                                   value_inputs,
                                                   preview: (0, 0) }));
    watch_input(cx, &name_input, &state, &controller);
    let inputs = state.borrow().value_inputs.clone();
    for input in &inputs {
        watch_input(cx, input, &state, &controller);
    }
    update(cx, &state, &controller);

    window.open_dialog(cx, move |dialog, _window, _cx| {
              let (state, controller) = (state.clone(), controller.clone());
              dialog.w(px(560.))
                    .close_button(false)
                    .overlay_closable(true)
                    .content(move |content, window, cx| {
                        render_content(content, window, cx, &state, &controller)
                    })
          });
}

/// A box for a rule's value, holding `text`.
fn new_value_input(window: &mut Window, cx: &mut App, text: &str) -> Entity<InputState> {
    cx.new(|cx| {
          InputState::new(window, cx)
            .placeholder(t!("smart_collections.editor.value_placeholder").to_string())
            .default_value(text.to_owned())
      })
}

/// Brings the draft up to date when `input` changes.
fn watch_input(cx: &mut App, input: &Entity<InputState>, state: &SharedState,
               controller: &Entity<LibraryController>) {
    let (state, controller) = (state.clone(), controller.clone());
    cx.subscribe(input, move |_, event: &InputEvent, cx| {
          if matches!(event, InputEvent::Change) {
              update(cx, &state, &controller);
          }
      })
      .detach();
}

/// Copies the text of the boxes into the draft, recounts the matches, and
/// redraws the editor.
fn update(cx: &mut App, state: &SharedState, controller: &Entity<LibraryController>) {
    {
        let mut editor = state.borrow_mut();
        let name = editor.name_input.read(cx).value().to_string();
        editor.draft.set_name(&name);
        let values: Vec<String> = editor.value_inputs
                                        .iter()
                                        .map(|input| input.read(cx).value().to_string())
                                        .collect();
        for (index, value) in values.iter().enumerate() {
            editor.draft.set_value(index, value);
        }
        let collection = editor.draft.collection().clone();
        editor.preview = controller.read(cx).smart_collection_preview(&collection);
    }
    cx.refresh_windows();
}

/// The label of a rule field in the field menu.
fn field_label(field: Field) -> String {
    match field {
        Field::Tag => t!("smart_collections.editor.field_tag"),
        Field::Publisher => t!("smart_collections.editor.field_publisher"),
        Field::System => t!("smart_collections.editor.field_system"),
        Field::Type => t!("smart_collections.editor.field_type"),
        Field::Format => t!("smart_collections.editor.field_format"),
        Field::Genre => t!("smart_collections.editor.field_genre"),
        Field::Availability => t!("smart_collections.editor.field_availability"),
        Field::Location => t!("smart_collections.editor.field_location"),
    }.to_string()
}

fn section_heading(label: String, colors: &ColorTokens) -> impl IntoElement {
    div().text_xs()
         .font_weight(gpui::FontWeight::SEMIBOLD)
         .text_color(colors.text_secondary)
         .child(label.to_uppercase())
}

/// The menu button that picks what a rule tests.
fn field_menu(index: usize, current: Field, state: &SharedState,
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
fn operator_menu(index: usize, current: Operator, state: &SharedState,
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
fn mode_menu(current: MatchMode, state: &SharedState, controller: &Entity<LibraryController>)
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
fn render_rule_row(index: usize, state: &SharedState, controller: &Entity<LibraryController>)
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

/// The box that says how many titles match now.
fn render_preview(state: &SharedState, colors: &ColorTokens) -> impl IntoElement {
    let (matching, total) = state.borrow().preview;
    div().p(px(12.0))
         .rounded_lg()
         .border_1()
         .border_color(colors.border)
         .bg(colors.surface_alt)
         .text_sm()
         .text_color(colors.text_primary)
         .child(t!("smart_collections.editor.preview",
                   matching = matching,
                   total = total).to_string())
}

/// A switch for an on-match action. The actions are not built yet, so the
/// switch is off and cannot be turned on; the tooltip says so.
fn render_on_match_row(id: &'static str, label: String, colors: &ColorTokens) -> impl IntoElement {
    div().flex()
         .items_center()
         .justify_between()
         .child(div().text_sm()
                     .text_color(colors.text_secondary)
                     .child(label))
         .child(div().id(id)
                     .tooltip(|window, cx| {
                         gpui_kit::component::tooltip::Tooltip::new(
                             t!("smart_collections.editor.on_match_later_tooltip").to_string(),
                         )
                         .build(window, cx)
                     })
                     .child(Switch::new(id).checked(false).disabled(true)))
}

fn render_content(content: DialogContent, _window: &mut Window, cx: &mut App,
                  state: &SharedState, controller: &Entity<LibraryController>)
                  -> DialogContent {
    let colors = cx.global::<LibriTheme>().colors.clone();
    let (is_new, name_input, rule_count, mode, can_save, query) = {
        let editor = state.borrow();
        (editor.draft.collection().id == 0,
         editor.name_input.clone(),
         editor.draft.rules().len(),
         editor.draft.mode(),
         editor.draft.can_save(),
         editor.draft.equivalent_query_text())
    };
    let title = if is_new {
        t!("smart_collections.editor.title_new")
    }
    else {
        t!("smart_collections.editor.title_edit")
    };
    let rows: Vec<_> = (0..rule_count).map(|index| render_rule_row(index, state, controller))
                                      .collect();
    let (add_state, add_controller) = (state.clone(), controller.clone());
    let (save_state, save_controller) = (state.clone(), controller.clone());
    let query_line: SharedString =
        query.map_or_else(|| t!("smart_collections.editor.no_equivalent_query").to_string(),
                          |text| {
                              t!("smart_collections.editor.equivalent_query",
                                                               query = text).to_string()
                          })
             .into();

    content.child(DialogHeader::new().px_4()
                                     .pt_4()
                                     .child(DialogTitle::new().child(title.to_string()))
                                     .child(div().text_sm()
                                                 .text_color(colors.text_secondary)
                                                 .child(t!("smart_collections.editor.description").to_string())))
           .child(v_flex().gap(px(16.0))
                          .p_4()
                          .child(v_flex().gap(px(6.0))
                                         .child(section_heading(t!("smart_collections.editor.name_heading").to_string(),
                                                                &colors))
                                         .child(Input::new(&name_input)))
                          .child(v_flex().gap(px(8.0))
                                         .child(section_heading(t!("smart_collections.editor.rules_heading").to_string(),
                                                                &colors))
                                         .child(div().flex()
                                                     .items_center()
                                                     .justify_between()
                                                     .child(div().text_sm()
                                                                 .child(t!("smart_collections.editor.match_label")
                                                                            .to_string()))
                                                     .child(mode_menu(mode, state, controller)))
                                         .children(rows)
                                         .child(div().child(Button::new("smart-rule-add").outline()
                                                                                         .icon(IconName::Plus)
                                                                                         .label(t!("smart_collections.editor.add_rule")
                                                                                                    .to_string())
                                                                                         .on_click(move |_, window, cx| {
                                                                                             let input = new_value_input(window, cx, "");
                                                                                             {
                                                                                                 let mut editor = add_state.borrow_mut();
                                                                                                 editor.draft.add_rule();
                                                                                                 editor.value_inputs.push(input.clone());
                                                                                             }
                                                                                             watch_input(cx, &input, &add_state, &add_controller);
                                                                                             update(cx, &add_state, &add_controller);
                                                                                             window.refresh();
                                                                                         }))))
                          .child(render_preview(state, &colors))
                          .child(v_flex().gap(px(8.0))
                                         .child(section_heading(t!("smart_collections.editor.on_match_heading").to_string(),
                                                                &colors))
                                         .child(render_on_match_row("smart-on-match-download",
                                                                    t!("smart_collections.editor.download_new").to_string(),
                                                                    &colors))
                                         .child(render_on_match_row("smart-on-match-mirror",
                                                                    t!("smart_collections.editor.mirror").to_string(),
                                                                    &colors))))
           .child(DialogFooter::new().px_4()
                                     .pb_4()
                                     .justify_between()
                                     .child(div().text_sm()
                                                 .text_color(colors.text_secondary)
                                                 .child(query_line))
                                     .child(div().flex()
                                                 .gap(px(8.0))
                                                 .child(Button::new("smart-editor-cancel").label(t!("smart_collections.editor.cancel")
                                                                                                     .to_string())
                                                                                          .on_click(|_, window, cx| window.close_dialog(cx)))
                                                 .child(Button::new("smart-editor-save").primary()
                                                                                        .label(t!("smart_collections.editor.save")
                                                                                                   .to_string())
                                                                                        .disabled(!can_save)
                                                                                        .on_click(move |_, window, cx| {
                                                                                            let collection = save_state.borrow().draft.clone().into_collection();
                                                                                            save_controller.update(cx, |ctrl, cx| {
                                                                                                if collection.id == 0 {
                                                                                                    ctrl.add_smart_collection(collection, cx);
                                                                                                }
                                                                                                else {
                                                                                                    ctrl.update_smart_collection(collection, cx);
                                                                                                }
                                                                                            });
                                                                                            window.close_dialog(cx);
                                                                                        }))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_editor_texts_resolve() {
        for key in ["smart_collections.editor.title_new",
                    "smart_collections.editor.title_edit",
                    "smart_collections.editor.description",
                    "smart_collections.editor.name_heading",
                    "smart_collections.editor.rules_heading",
                    "smart_collections.editor.match_all",
                    "smart_collections.editor.match_any",
                    "smart_collections.editor.op_is",
                    "smart_collections.editor.op_is_not",
                    "smart_collections.editor.add_rule",
                    "smart_collections.editor.remove_rule_tooltip",
                    "smart_collections.editor.on_match_later_tooltip",
                    "smart_collections.editor.no_equivalent_query",
                    "smart_collections.editor.field_format",
                    "smart_collections.editor.field_genre",
                    "smart_collections.editor.save",
                    "smart_collections.edit",
                    "filters.save_as_smart_collection",
                    "filters.save_as_smart_collection_tooltip",
                    "menu.catalog_new_smart_collection"]
        {
            assert_ne!(t!(key).to_string(), key, "{key} has no text");
        }
    }

    #[test]
    fn every_rule_field_has_a_label() {
        for field in RULE_FIELDS {
            assert!(!field_label(field).starts_with("smart_collections."),
                    "{field:?} has no text");
        }
    }
}
