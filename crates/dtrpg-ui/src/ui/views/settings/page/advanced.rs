//! Advanced settings section: the Recently Updated window, filter values,
//! native integration, and the cache maintenance actions (opening the cache
//! folder, clearing the cache). What the cache holds is shown in Insights.

use dtrpg_core::data::native_preferences::NativePreferencesFile;
use gpui::{AnyElement, Entity, IntoElement, ParentElement, Styled, div, px};
use gpui_kit::component::WindowExt as _;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{InputState, NumberInput};
use rust_i18n::t;

use crate::controllers::library::{FilterValueSection, LibraryController};
use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::ui::views::settings::card::field_row;
use crate::ui::views::settings::filter_values::render_filter_values_section;
use crate::ui::views::settings::page::native::render_native_section;
use crate::ui::widgets::selectable_note;

/// Renders the "Recently Updated window" row: a label/note pair and an
/// editable [`NumberInput`] (bounded 7-90 via `InputState::min`/`max`, set
/// when the field was created) with built-in +/- stepper buttons.
///
/// Falls back to a plain, non-editable value when `input` is `None` (before
/// the root view attaches the shared input state), matching the pattern used
/// for `storage_path_input` in `settings_storage_view`.
fn render_recently_updated_window_row(recently_updated_window_days: u32,
                                      input: Option<Entity<InputState>>, colors: &ColorTokens)
                                      -> impl IntoElement + 'static + use<> {
    let field: AnyElement = if let Some(input_state) = input {
        NumberInput::new(&input_state).into_any_element()
    }
    else {
        div().text_sm()
             .text_color(colors.text_primary)
             .child(recently_updated_window_days.to_string())
             .into_any_element()
    };

    field_row("recently-updated-window-note",
              t!("settings.recently_updated_window_title").to_string(),
              t!("settings.recently_updated_window_note").to_string(),
              field,
              colors)
}

/// Renders the Advanced settings section: the "Recently Updated window"
/// number field, cache visibility (counts and timing constants), an "Open
/// cache folder" action, and a "Clear cache" action with a confirmation
/// dialog.
#[allow(clippy::too_many_arguments)]
pub fn render_advanced_section(entity: Entity<SettingsController>,
                               native: NativePreferencesFile, colors: &ColorTokens,
                               label_font_family: &str, recently_updated_window_days: u32,
                               recently_updated_window_input: Option<Entity<InputState>>,
                               filter_values: &[FilterValueSection],
                               library: &Entity<LibraryController>)
                               -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let border = colors.border;
    let entity_open_folder = entity.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(t!("settings.advanced_title")),
        )
        .child(div().h(px(1.0)).bg(border))
        .child(render_recently_updated_window_row(
            recently_updated_window_days,
            recently_updated_window_input,
            colors,
        ))
        .child(div().h(px(1.0)).bg(border))
        .children(render_filter_values_section(filter_values, library, colors))
        .child(div().h(px(1.0)).bg(border))
        .child(render_native_section(entity.clone(), native, colors, label_font_family))
        .child(div().h(px(1.0)).bg(border))
        .child(selectable_note(
            "advanced-cache-note",
            t!("settings.cache_details_moved").to_string(),
            colors,
        ))
        .child(
            Button::new("open-cache-folder-btn")
                .outline()
                .label(t!("settings.open_cache_folder_button"))
                .on_click(move |_, _, cx| {
                    entity_open_folder.read(cx).open_cache_folder();
                }),
        )
        .child(div().h(px(1.0)).bg(border))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child(t!("settings.clear_cache_description")),
                )
                .child(
                    Button::new("clear-cache-btn")
                        .danger()
                        .label(t!("settings.clear_cache_button"))
                        .on_click(move |_, window, cx| {
                            let entity = entity.clone();
                            window.open_alert_dialog(cx, move |alert, _, _| {
                                let entity = entity.clone();
                                alert
                                    .confirm()
                                    .title(t!("settings.clear_cache_confirm_title").to_string())
                                    .description(
                                        t!("settings.clear_cache_confirm_description").to_string(),
                                    )
                                    .on_ok(move |_, _window, cx| {
                                        entity.update(cx, |ctrl, cx| ctrl.clear_cache(cx));
                                        true
                                    })
                            });
                        }),
                ),
        )
}
