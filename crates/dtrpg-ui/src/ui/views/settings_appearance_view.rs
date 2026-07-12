//! Appearance settings section: searchable font pickers for the four font
//! roles (body, value, label, monospace) covering every font installed on
//! the user's system, a shared UI text size stepper, and the active color
//! theme picker.
//!
//! Font/theme changes apply immediately via `LibraryController`'s
//! `set_body_font`/`set_value_font`/`set_label_font`/`set_mono_font`/
//! `set_ui_text_size`/`set_theme`, which also persist the selection through
//! `crate::data::ui_prefs::UiPrefs` — see `settings-appearance-fonts`.

use gpui::{
    Entity, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled,
    div, px,
};
use gpui_component::button::Button;
use gpui_component::combobox::Combobox;
use gpui_component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::controllers::settings::FontSelectState;
use crate::data::constants::{MAX_UI_TEXT_SIZE, MIN_UI_TEXT_SIZE, MONO_SIZE_RATIO};
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

/// Renders one font-picker row: the role's label on the left, a searchable
/// combobox on the right showing the current font name and its rendered
/// size in points — the size is shown alongside the name (not just settable
/// elsewhere) so a low-vision user picking a font can see, without hunting
/// for a separate control, exactly what size it will render at.
///
/// `select` is `None` for one render pass at most (the root view attaches it
/// immediately after construction, before this section can be reached), in
/// which case a disabled placeholder renders instead of a broken combobox.
fn font_picker_row(title: impl Into<gpui::SharedString>, select: Option<&FontSelectState>,
                   size_pt: f32, colors: &ColorTokens)
                   -> impl IntoElement + 'static {
    let row = div().flex()
                   .items_center()
                   .justify_between()
                   .gap(px(12.0))
                   .child(div().text_sm()
                               .font_weight(gpui::FontWeight::MEDIUM)
                               .text_color(colors.text_primary)
                               .child(title.into()));

    let Some(select) = select
    else {
        return row.child(div().text_sm().text_color(colors.text_tertiary).child("…"));
    };

    row.child(Combobox::new(select).menu_width(px(280.0))
                                   .placeholder(t!("settings.appearance_font_placeholder"))
                                   .search_placeholder(t!("settings.appearance_font_search_placeholder"))
                                   .render_trigger(move |ctx, _window, _cx| {
                                       let label = match ctx.selection.first() {
                                           Some((_, name)) => format!("{name}, {size_pt:.0}pt"),
                                           None => ctx.placeholder
                                                      .map(ToString::to_string)
                                                      .unwrap_or_default(),
                                       };
                                       Button::new("font-picker-trigger").outline()
                                                                         .label(label)
                                                                         .into_any_element()
                                   }))
}

/// Renders the shared "Text Size" stepper row, matching the concurrency
/// stepper style already used in `settings_storage_view`.
fn text_size_row(current_pt: f32, entity: Entity<LibraryController>, colors: &ColorTokens)
                 -> impl IntoElement + 'static {
    let entity_dec = entity.clone();
    let entity_inc = entity;

    div().flex()
         .items_center()
         .justify_between()
         .gap(px(12.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::MEDIUM)
                     .text_color(colors.text_primary)
                     .child(t!("settings.appearance_text_size_label")))
         .child(div().flex()
                     .items_center()
                     .gap(px(8.0))
                     .child(div().id("appearance-text-size-decrement")
                                 .flex_none()
                                 .size(px(28.0))
                                 .rounded(px(6.0))
                                 .border_1()
                                 .border_color(colors.border)
                                 .flex()
                                 .items_center()
                                 .justify_center()
                                 .cursor_pointer()
                                 .tooltip(|window, cx| {
                                     Tooltip::new(t!("settings.appearance_text_size_decrement_tooltip")
                                          .to_string()).build(window, cx)
                                 })
                                 .on_click(move |_, _, cx| {
                                     let next = (current_pt - 1.0).max(MIN_UI_TEXT_SIZE);
                                     entity_dec.update(cx, |ctrl, cx| {
                                                   ctrl.set_ui_text_size(px(next), cx);
                                               });
                                 })
                                 .child(div().text_sm()
                                             .text_color(colors.text_primary)
                                             .child("\u{2212}")))
                     .child(div().w(px(40.0))
                                 .text_sm()
                                 .text_color(colors.text_primary)
                                 .text_align(gpui::TextAlign::Center)
                                 .child(format!("{current_pt:.0}pt")))
                     .child(div().id("appearance-text-size-increment")
                                 .flex_none()
                                 .size(px(28.0))
                                 .rounded(px(6.0))
                                 .border_1()
                                 .border_color(colors.border)
                                 .flex()
                                 .items_center()
                                 .justify_center()
                                 .cursor_pointer()
                                 .tooltip(|window, cx| {
                                     Tooltip::new(t!("settings.appearance_text_size_increment_tooltip")
                                          .to_string()).build(window, cx)
                                 })
                                 .on_click(move |_, _, cx| {
                                     let next = (current_pt + 1.0).min(MAX_UI_TEXT_SIZE);
                                     entity_inc.update(cx, |ctrl, cx| {
                                                   ctrl.set_ui_text_size(px(next), cx);
                                               });
                                 })
                                 .child(div().text_sm().text_color(colors.text_primary).child("+"))))
}

/// The four Appearance-page font picker states, bundled into one struct
/// rather than threaded as separate function arguments (see `docs/rust.md`'s
/// guidance against many-argument functions).
pub struct AppearanceFontSelects {
    pub body:  Option<FontSelectState>,
    pub value: Option<FontSelectState>,
    pub label: Option<FontSelectState>,
    pub mono:  Option<FontSelectState>,
}

/// Renders the Appearance settings section: body/value/label/monospace font
/// pickers (searchable, listing every font installed on the user's system),
/// a shared text-size stepper, and the active theme picker.
pub fn render_appearance_section(entity: Entity<LibraryController>, colors: &ColorTokens,
                                 theme: &LibriTheme, font_selects: AppearanceFontSelects)
                                 -> impl IntoElement + 'static + use<> {
    let AppearanceFontSelects { body: body_font_select,
                                value: value_font_select,
                                label: label_font_select,
                                mono: mono_font_select, } = font_selects;
    let text_primary = colors.text_primary;
    let border = colors.border;
    let theme_key = theme.key;
    let base_pt = theme.fonts.ui_text_size.as_f32();
    let mono_pt = base_pt * MONO_SIZE_RATIO;

    let entity_theme = entity.clone();

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
                         .items_center()
                         .justify_between()
                         .gap(px(12.0))
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
         .child(font_picker_row(t!("settings.appearance_body_font_label"),
                                body_font_select.as_ref(),
                                base_pt,
                                colors))
         .child(font_picker_row(t!("settings.appearance_value_font_label"),
                                value_font_select.as_ref(),
                                base_pt,
                                colors))
         .child(font_picker_row(t!("settings.appearance_label_font_label"),
                                label_font_select.as_ref(),
                                base_pt,
                                colors))
         .child(font_picker_row(t!("settings.appearance_mono_font_label"),
                                mono_font_select.as_ref(),
                                mono_pt,
                                colors))
         .child(text_size_row(base_pt, entity, colors))
         .child(div().h(px(1.0)).bg(border))
         .child(theme_row)
}
