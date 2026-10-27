//! Appearance settings section: a card with the text scale and the density,
//! the theme gallery (`settings_theme_gallery`) and the theme file editor
//! (`settings_theme_editor`). Every part follows the other settings pages: a
//! small-caps heading, then rows with a title and note on the left and the
//! control on the right.
//!
//! Changes apply immediately via `LibraryController`'s `set_ui_text_size`,
//! `set_density` and `set_theme`, which also save the choice through
//! `dtrpg_core::data::ui_preferences::UiPreferences`. Fonts are part of a
//! theme, so they are changed by editing the theme.

use dtrpg_core::data::constants::DEFAULT_UI_TEXT_SIZE;
use gpui::prelude::*;
use gpui::{AnyElement, Entity, div, px};
use gpui_kit::component::Sizable as _;
use gpui_kit::component::input::{InputState, NumberInput};
use gpui_kit::component::tab::{Tab, TabBar};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::theme::{ColorTokens, Density, LibriTheme};
use crate::ui::views::settings::card::{FIELD_WIDTH, card, card_row, section_heading};
use crate::ui::views::settings::theme_editor::{ThemeEditorData, render_theme_editor};
use crate::ui::views::settings::theme_gallery::render_theme_gallery;

/// The title and note of a row in the card, on the left.
fn row_text(title: String, note: String, colors: &ColorTokens) -> impl IntoElement {
    div().flex_1()
         .min_w_0()
         .flex()
         .flex_col()
         .gap(px(2.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(title))
         .child(div().text_xs()
                     .text_color(colors.text_secondary)
                     .child(note))
}

/// The text scale field: an editable number with +/- buttons, shown as a
/// multiplier in tenths where `1.0` is the app's normal text size. Shows the
/// plain scale until the root view attaches the shared field.
fn text_scale_control(scale: f32, input: Option<Entity<InputState>>, colors: &ColorTokens)
                      -> AnyElement {
    match input {
        Some(input_state) => div().w(px(FIELD_WIDTH))
                                  .flex_none()
                                  .child(NumberInput::new(&input_state).small())
                                  .into_any_element(),
        None => div().text_sm()
                     .text_color(colors.text_primary)
                     .child(format!("{scale:.1}"))
                     .into_any_element(),
    }
}

/// The label of `density` on the density selector.
fn density_label(density: Density) -> String {
    match density {
        Density::Comfortable => t!("settings.appearance_density_comfortable"),
        Density::Compact => t!("settings.appearance_density_compact"),
    }.to_string()
}

/// The density selector: two segments, comfortable and compact.
fn density_control(current: Density, entity: Entity<LibraryController>) -> impl IntoElement {
    let selected = Density::ALL.iter()
                               .position(|d| *d == current)
                               .unwrap_or_default();
    Density::ALL.into_iter()
                .fold(TabBar::new("appearance-density").segmented()
                                                       .selected_index(selected),
                      |bar, density| bar.child(Tab::new().label(density_label(density))))
                .on_click(move |ix, _, cx| {
                    if let Some(density) = Density::ALL.get(*ix).copied() {
                        entity.update(cx, |ctrl, cx| ctrl.set_density(density, cx));
                    }
                })
}

/// The "Text & density" card: the text scale and the density.
fn text_and_density(scale: f32, density: Density, entity: &Entity<LibraryController>,
                    text_scale_input: Option<Entity<InputState>>, colors: &ColorTokens)
                    -> impl IntoElement + 'static + use<> {
    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(section_heading(
            t!("settings.appearance_typography_title").to_string(),
            colors,
        ))
        .child(
            card(colors)
                .child(
                    card_row(true, colors)
                        .child(row_text(
                            t!("settings.appearance_text_size_label").to_string(),
                            t!("settings.appearance_text_size_note").to_string(),
                            colors,
                        ))
                        .child(text_scale_control(scale, text_scale_input, colors)),
                )
                .child(
                    card_row(false, colors)
                        .child(row_text(
                            t!("settings.appearance_density_label").to_string(),
                            t!("settings.appearance_density_note").to_string(),
                            colors,
                        ))
                        .child(density_control(density, entity.clone())),
                ),
        )
}

/// Renders the Appearance settings section: the text scale and density, the
/// theme gallery and the theme file editor.
pub fn render_appearance_section(entity: Entity<LibraryController>, colors: &ColorTokens,
                                 theme: &LibriTheme, theme_editor: ThemeEditorData,
                                 text_scale_input: Option<Entity<InputState>>)
                                 -> impl IntoElement + 'static + use<> {
    let scale = theme.fonts.ui_text_size.as_f32() / DEFAULT_UI_TEXT_SIZE;
    div().flex()
         .flex_col()
         .gap(px(24.0))
         .p(px(24.0))
         .child(div().flex()
                     .flex_col()
                     .gap(px(6.0))
                     .child(div().text_sm()
                                 .font_weight(gpui::FontWeight::SEMIBOLD)
                                 .text_color(colors.text_primary)
                                 .child(t!("settings.appearance_title")))
                     .child(div().text_sm()
                                 .text_color(colors.text_secondary)
                                 .child(t!("settings.appearance_intro"))))
         .child(text_and_density(scale, theme.density, &entity, text_scale_input, colors))
         .child(render_theme_gallery(entity.clone(), colors, &theme.id))
         .child(render_theme_editor(theme_editor, entity, &theme.id, colors))
}
