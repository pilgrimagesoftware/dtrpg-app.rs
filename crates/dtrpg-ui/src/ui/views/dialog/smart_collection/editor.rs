//! The smart collection editor: a name, the rules, a live count of the titles
//! that match, and the on-match switches.
//!
//! The choices live in a [`SmartCollectionDraft`]; this file draws them and
//! passes each change to it. The text boxes keep their own text, so the
//! draft is brought up to date from them whenever one changes, and the match
//! count is worked out then, not while drawing: it reads the whole catalog.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use dtrpg_core::data::smart_collection::{MatchMode, Rule};
use dtrpg_core::library::SmartPreview;
use gpui::prelude::*;
use gpui::{AnyElement, App, Entity, SharedString, Window, div, px};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::dialog::{DialogContent, DialogFooter, DialogHeader, DialogTitle};
use gpui_kit::component::input::{Input, InputEvent, InputState};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{Disableable, IconName, WindowExt as _, v_flex};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::theme::{ColorTokens, LibriTheme};
use crate::ui::views::dialog::smart_collection::rules::{mode_menu, render_rule_row};
use crate::view_models::smart_collection_editor::SmartCollectionDraft;

/// Everything the editor keeps while it is open.
pub(super) struct EditorState {
    /// The collection being edited.
    pub(super) draft:        SmartCollectionDraft,
    /// The box for the name.
    name_input:              Entity<InputState>,
    /// The box for each rule's value, in the same order as the rules.
    pub(super) value_inputs: Vec<Entity<InputState>>,
    /// What matches now: the counts and the first few titles.
    preview:                 Option<SmartPreview>,
    /// The mode and rules `preview` was worked out for; the count is only
    /// redone when they change, not for a new name or an on-match switch.
    previewed:               (MatchMode, Vec<Rule>),
    /// Counts the requests for a new preview, so a waiting one can tell a
    /// newer one came.
    generation:              u64,
}

/// How long typing has to pause before the matches are counted again; the
/// count reads the whole catalog, so doing it per keystroke stalls the window.
const PREVIEW_DELAY: Duration = Duration::from_millis(250);

/// The tallest the list of matching titles grows before it scrolls.
const PREVIEW_LIST_HEIGHT: f32 = 140.0;

/// The editor's state, shared by the dialog and the callbacks that change it.
pub(super) type SharedState = Rc<RefCell<EditorState>>;

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
    let collection = draft.collection().clone();
    let state = Rc::new(RefCell::new(EditorState { draft,
                                                   name_input: name_input.clone(),
                                                   value_inputs,
                                                   preview: None,
                                                   previewed: (collection.mode,
                                                               collection.rules),
                                                   generation: 0 }));
    watch_input(cx, &name_input, &state, &controller);
    let inputs = state.borrow().value_inputs.clone();
    for input in &inputs {
        watch_input(cx, input, &state, &controller);
    }
    // The first count starts at once; the box stays empty until it lands.
    schedule_preview(cx, &state, &controller, 0, Duration::ZERO);

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

/// Copies the text of the boxes into the draft, counts the matches again if
/// the rules changed, and redraws the editor.
pub(super) fn update(cx: &mut App, state: &SharedState, controller: &Entity<LibraryController>) {
    let generation = {
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
        let collection = editor.draft.collection();
        let query = (collection.mode, collection.rules.clone());
        if editor.previewed == query {
            None
        }
        else {
            editor.previewed = query;
            editor.generation += 1;
            Some(editor.generation)
        }
    };
    if let Some(generation) = generation {
        schedule_preview(cx, state, controller, generation, PREVIEW_DELAY);
    }
    cx.refresh_windows();
}

/// Counts the matches on a background thread once typing has paused for
/// `delay`, unless a newer request came, then stores the result and redraws.
fn schedule_preview(cx: &mut App, state: &SharedState, controller: &Entity<LibraryController>,
                    generation: u64, delay: Duration) {
    let (state, controller) = (state.clone(), controller.clone());
    cx.spawn(async move |cx| {
          cx.background_executor().timer(delay).await;
          let Some((library, collection)) = cx.update(|cx| {
                                                  let current = state.borrow();
                                                  (current.generation == generation).then(|| {
                                                      (controller.read(cx).library_handle(),
                                                       current.draft.collection().clone())
                                                  })
                                              })
          else {
              return;
          };
          let preview = cx.background_executor()
                          .spawn(async move { library.smart_collection_preview(&collection) })
                          .await;
          cx.update(|cx| {
                if state.borrow().generation == generation {
                    state.borrow_mut().preview = Some(preview);
                    cx.refresh_windows();
                }
            });
      })
      .detach();
}

fn section_heading(label: String, colors: &ColorTokens) -> impl IntoElement {
    div().text_xs()
         .font_weight(gpui::FontWeight::SEMIBOLD)
         .text_color(colors.text_secondary)
         .child(label.to_uppercase())
}

/// The box that says how many titles match now, with the first few of them
/// in a list that scrolls.
fn render_preview(state: &SharedState, colors: &ColorTokens) -> AnyElement {
    let Some(preview) = state.borrow().preview.clone()
    else {
        return div().into_any_element();
    };
    let hidden = preview.matching.saturating_sub(preview.titles.len());
    v_flex().p(px(12.0))
            .gap(px(8.0))
            .rounded_lg()
            .border_1()
            .border_color(colors.border)
            .bg(colors.surface_alt)
            .text_sm()
            .text_color(colors.text_primary)
            .child(t!("smart_collections.editor.preview",
                      matching = preview.matching,
                      total = preview.total).to_string())
            .when(!preview.titles.is_empty(), |column| {
                column.child(v_flex().id("smart-preview-titles")
                                     .max_h(px(PREVIEW_LIST_HEIGHT))
                                     .overflow_y_scroll()
                                     .gap(px(2.0))
                                     .text_color(colors.text_secondary)
                                     .children(preview.titles
                                                      .iter()
                                                      .map(|title| div().truncate().child(title.to_string())))
                                     .when(hidden > 0, |list| {
                                         list.child(t!("smart_collections.editor.preview_more",
                                                       count = hidden).to_string())
                                     }))
            })
            .into_any_element()
}

/// A switch for an on-match action, which `on_change` turns on or off in the
/// draft.
fn render_on_match_row(id: &'static str, label: String, checked: bool, colors: &ColorTokens,
                       on_change: impl Fn(bool, &mut App) + 'static)
                       -> impl IntoElement {
    div().flex()
         .items_center()
         .justify_between()
         .child(div().text_sm()
                     .text_color(colors.text_secondary)
                     .child(label))
         .child(Switch::new(id).checked(checked)
                               .on_click(move |checked, _window, cx| on_change(*checked, cx)))
}

fn render_content(content: DialogContent, _window: &mut Window, cx: &mut App,
                  state: &SharedState, controller: &Entity<LibraryController>)
                  -> DialogContent {
    let colors = cx.global::<LibriTheme>().colors.clone();
    let (is_new, name_input, rule_count, mode, can_save, query, on_match) = {
        let editor = state.borrow();
        (editor.draft.collection().id == 0,
         editor.name_input.clone(),
         editor.draft.rules().len(),
         editor.draft.mode(),
         editor.draft.can_save(),
         editor.draft.equivalent_query_text(),
         (editor.draft.download_new_matches(), editor.draft.mirror_as_symlinks()))
    };
    let (download_state, mirror_state) = (state.clone(), state.clone());
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
                                                                    on_match.0,
                                                                    &colors,
                                                                    move |on, cx| {
                                                                        download_state.borrow_mut()
                                                                                      .draft
                                                                                      .set_download_new_matches(on);
                                                                        cx.refresh_windows();
                                                                    }))
                                         .child(render_on_match_row("smart-on-match-mirror",
                                                                    t!("smart_collections.editor.mirror").to_string(),
                                                                    on_match.1,
                                                                    &colors,
                                                                    move |on, cx| {
                                                                        mirror_state.borrow_mut()
                                                                                    .draft
                                                                                    .set_mirror_as_symlinks(on);
                                                                        cx.refresh_windows();
                                                                    }))))
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
                    "smart_collections.editor.preview_more",
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
}
