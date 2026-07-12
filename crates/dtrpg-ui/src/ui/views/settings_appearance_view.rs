//! Appearance settings section: pickers for the three font roles (body,
//! value, monospace) and the active color theme.
//!
//! Font/theme changes apply immediately via `LibraryController`'s
//! `set_body_font`/`set_value_font`/`set_mono_font`/`set_theme`, which also
//! persist the selection through `crate::data::ui_prefs::UiPrefs` — see
//! `settings-appearance-fonts`.

use gpui::{Entity, IntoElement, ParentElement, Styled, div, px};
use gpui_component::button::Button;
use gpui_component::menu::{DropdownMenu as _, PopupMenuItem};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::constants::{
    BODY_FONT_OPTIONS, FontOption, MONO_FONT_OPTIONS, VALUE_FONT_OPTIONS,
};
use crate::data::theme::{ColorTokens, LibriTheme, ThemeKey};

/// All theme keys offered by the picker, in display order.
const THEME_KEYS: [ThemeKey; 6] = [ThemeKey::Parchment,
                                   ThemeKey::Slate,
                                   ThemeKey::Sage,
                                   ThemeKey::Ink,
                                   ThemeKey::Moss,
                                   ThemeKey::Rosewood];

fn theme_label(key: ThemeKey) -> String {
    match key {
        ThemeKey::Parchment => t!("theme.parchment").to_string(),
        ThemeKey::Slate => t!("theme.slate").to_string(),
        ThemeKey::Sage => t!("theme.sage").to_string(),
        ThemeKey::Ink => t!("theme.ink").to_string(),
        ThemeKey::Moss => t!("theme.moss").to_string(),
        ThemeKey::Rosewood => t!("theme.rosewood").to_string(),
    }
}

/// Renders one picker row: a label above a dropdown button showing the
/// current selection, with `on_select` invoked with the chosen option's
/// index into `options`.
fn font_picker_row(row_id: &'static str, title: impl Into<gpui::SharedString>,
                   options: &'static [FontOption], current: &'static FontOption,
                   colors: &ColorTokens,
                   on_select: impl Fn(&'static FontOption, &mut gpui::Window, &mut gpui::App)
                   + Clone
                   + 'static)
                   -> impl IntoElement + 'static {
    let button = Button::new(row_id).outline()
                                    .label(t!(current.label_key).to_string())
                                    .dropdown_menu(move |menu, _, _| {
                                        let mut m = menu;
                                        for option in options {
                                            let on_select = on_select.clone();
                                            m = m.item(PopupMenuItem::new(t!(option.label_key)
                                                        .to_string())
                                                .checked(option.id == current.id)
                                                .on_click(move |_, window, cx| {
                                                    on_select(option, window, cx);
                                                }));
                                        }
                                        m
                                    });

    div().flex()
         .flex_col()
         .gap(px(6.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::MEDIUM)
                     .text_color(colors.text_primary)
                     .child(title.into()))
         .child(button)
}

/// Renders the Appearance settings section: body/value/monospace font
/// pickers and the active theme picker, each applying immediately.
pub fn render_appearance_section(entity: Entity<LibraryController>, colors: &ColorTokens,
                                 theme: &LibriTheme)
                                 -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let border = colors.border;
    let theme_key = theme.key;

    let entity_body = entity.clone();
    let entity_value = entity.clone();
    let entity_mono = entity.clone();
    let entity_theme = entity;

    let body_row =
        font_picker_row("appearance-body-font",
                        t!("settings.appearance_body_font_label"),
                        BODY_FONT_OPTIONS,
                        theme.body_font,
                        colors,
                        move |font, _window, cx| {
                            entity_body.update(cx, |ctrl, cx| ctrl.set_body_font(font, cx));
                        });

    let value_row = font_picker_row("appearance-value-font",
                                    t!("settings.appearance_value_font_label"),
                                    VALUE_FONT_OPTIONS,
                                    theme.value_font,
                                    colors,
                                    move |font, _window, cx| {
                                        entity_value.update(cx, |ctrl, cx| {
                                                        ctrl.set_value_font(font, cx);
                                                    });
                                    });

    let mono_row =
        font_picker_row("appearance-mono-font",
                        t!("settings.appearance_mono_font_label"),
                        MONO_FONT_OPTIONS,
                        theme.mono_font,
                        colors,
                        move |font, _window, cx| {
                            entity_mono.update(cx, |ctrl, cx| ctrl.set_mono_font(font, cx));
                        });

    let theme_button =
        Button::new("appearance-theme").outline()
                                       .label(theme_label(theme_key))
                                       .dropdown_menu(move |menu, _, _| {
                                           let mut m = menu;
                                           for key in THEME_KEYS {
                                               let e = entity_theme.clone();
                                               m = m.item(PopupMenuItem::new(theme_label(key))
                                                   .checked(key == theme_key)
                                                   .on_click(move |_, _, cx| {
                                                       e.update(cx, |ctrl, cx| ctrl.set_theme(key, cx));
                                                   }));
                                           }
                                           m
                                       });
    let theme_row = div().flex()
                         .flex_col()
                         .gap(px(6.0))
                         .child(div().text_sm()
                                     .font_weight(gpui::FontWeight::MEDIUM)
                                     .text_color(text_primary)
                                     .child(t!("settings.appearance_theme_label")))
                         .child(theme_button);

    div().flex()
         .flex_col()
         .gap(px(24.0))
         .p(px(24.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(text_primary)
                     .child(t!("settings.appearance_title")))
         .child(div().h(px(1.0)).bg(border))
         .child(body_row)
         .child(value_row)
         .child(mono_row)
         .child(div().h(px(1.0)).bg(border))
         .child(theme_row)
}
