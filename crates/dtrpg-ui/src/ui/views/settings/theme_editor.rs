//! The theme file editor in Settings > Appearance: a read-only view of the
//! active theme's file.

use gpui::prelude::*;
use gpui::{App, Entity, Window, div, px};
use gpui_kit::component::Icon;
use gpui_kit::component::button::Button;
use gpui_kit::component::input::{Editor, EditorState};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::data::theme_catalog::{ThemeId, theme_label};
use crate::data::{theme_loader, theme_source};
use crate::ui::views::settings_card::section_heading;
use crate::ui::widgets::selectable_note;

/// How tall the editor box is, in pixels.
const EDITOR_HEIGHT: f32 = 320.0;

/// What the theme editor needs from the settings window.
pub struct ThemeEditorData {
    /// The editor's text, once the root view has attached it.
    pub input:    Option<Entity<EditorState>>,
    /// Why the last save failed, if it did.
    pub error:    Option<String>,
    /// The settings controller, which holds the error.
    pub settings: Entity<SettingsController>,
}

/// Puts the active theme's file text in `editor`, unless it is already there.
///
/// Reads the file, so it belongs on an event such as a theme change, not on
/// the render path.
pub fn load_theme_editor(editor: &Entity<EditorState>, window: &mut Window, cx: &mut App) {
    let active = cx.global::<crate::data::theme::LibriTheme>().id.clone();
    let source = theme_loader::theme_source(&active).unwrap_or_default();
    editor.update(cx, |state, cx| {
              if state.value().as_ref() != source {
                  state.set_value(source, window, cx);
              }
          });
}

/// Renders the Theme file section: a note on what can be done with this theme,
/// the editor, the save and revert buttons for a user theme, and what the file
/// accepts.
pub fn render_theme_editor(data: ThemeEditorData, library: Entity<LibraryController>,
                           theme_id: &ThemeId, colors: &ColorTokens)
                           -> impl IntoElement + 'static + use<> {
    let built_in = theme_source::is_builtin(theme_id);
    let name = theme_label(theme_id);
    let note = if built_in {
        t!("settings.theme_editor_built_in", name = name).to_string()
    }
    else {
        t!("settings.theme_editor_user", name = name).to_string()
    };
    let path = if built_in {
        None
    }
    else {
        theme_loader::user_theme_path(theme_id.as_str())
    };
    let editor = data.input
                     .clone()
                     .map(|input| Editor::new(&input).h(px(EDITOR_HEIGHT)).readonly(true));
    let _buttons =
        (!built_in).then(|| {
                       data.input
                           .clone()
                           .map(|input| edit_buttons(input, library, data.settings.clone()))
                   });
    div().flex()
         .flex_col()
         .gap(px(10.0))
         .child(section_heading(t!("settings.theme_editor_title").to_string(), colors))
         .child(selectable_note("theme-editor-note", note, colors))
         .children(path.map(|p| {
                           div().text_xs()
                                .text_color(colors.text_secondary)
                                .child(p.display().to_string())
                       }))
         .children(editor)
         .children(data.error.map(|message| {
                                 div().text_xs()
                                      .text_color(colors.error)
                                      .child(t!("settings.theme_editor_not_saved",
                                                message = message).to_string())
                             }))
         .children(_buttons.flatten())
         .child(selectable_note("theme-editor-help",
                                t!("settings.theme_editor_help").to_string(),
                                colors))
         .child(div().text_xs()
                     .text_color(colors.text_secondary)
                     .child("Unknown keys are ignored."))
}

/// The save and revert icon buttons.
fn edit_buttons(input: Entity<EditorState>, library: Entity<LibraryController>,
                settings: Entity<SettingsController>)
                -> impl IntoElement {
    let revert_input = input.clone();
    let revert_settings = settings.clone();
    div()
        .flex()
        .gap(px(6.0))
        .child(
            Button::new("theme-editor-save")
                .outline()
                .icon(Icon::empty().path("icons/save.svg"))
                .tooltip(t!("settings.theme_editor_save_tooltip").to_string())
                .on_click(move |_, _, cx| {
                    let source = input.read(cx).value().to_string();
                    let result =
                        library.update(cx, |ctrl, cx| ctrl.save_active_theme_source(&source, cx));
                    let error = result.err().map(|error| error.to_string());
                    settings.update(cx, |ctrl, cx| ctrl.set_theme_editor_error(error, cx));
                }),
        )
        .child(
            Button::new("theme-editor-revert")
                .outline()
                .icon(Icon::empty().path("icons/rotate-ccw.svg"))
                .tooltip(t!("settings.theme_editor_revert_tooltip").to_string())
                .on_click(move |_, window, cx| {
                    load_theme_editor(&revert_input, window, cx);
                    revert_settings.update(cx, |ctrl, cx| {
                        ctrl.set_theme_editor_error(None, cx);
                    });
                }),
        )
}
