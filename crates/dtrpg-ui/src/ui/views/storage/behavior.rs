//! The Behavior section of the Storage page: switches for what Libri does when
//! it stores files, and the number of downloads it runs at once.

use dtrpg_core::platform::{Platform, PlatformIntegration, PlatformKind};
use gpui::prelude::*;
use gpui::{AnyElement, Entity, div, px};
use gpui_kit::component::Sizable as _;
use gpui_kit::component::input::{InputState, NumberInput};
use rust_i18n::t;

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::ui::views::settings_card::{card, card_row, field_row, section_heading};
use crate::ui::views::settings_native_view::switch_row;

/// What the Behavior section shows.
pub(super) struct BehaviorData {
    /// Whether a download that would not fit is refused.
    pub check_free_space:         bool,
    /// Whether collections are mirrored as folders of symlinks.
    pub create_collections:       bool,
    /// How many downloads run at once.
    pub max_concurrent_downloads: usize,
    /// The field that edits `max_concurrent_downloads`, once attached.
    pub max_concurrent_input:     Option<Entity<InputState>>,
}

/// Renders the Behavior heading and its card.
pub(super) fn render_storage_behavior(data: BehaviorData, entity: Entity<SettingsController>,
                                      colors: &ColorTokens)
                                      -> impl IntoElement + 'static + use<> {
    let free_space_entity = entity.clone();
    let collections_entity = entity;
    div().flex()
         .flex_col()
         .gap(px(10.0))
         .child(section_heading(t!("settings.storage_behavior_title").to_string(), colors))
         .child(card(colors).child(card_row(true, colors).child(div().flex_1().child(switch_row(
                "check-free-space",
                t!("settings.check_free_space_title").to_string(),
                t!("settings.check_free_space_note").to_string(),
                data.check_free_space,
                colors,
                move |checked, cx| {
                    free_space_entity.update(cx, |ctrl, cx| ctrl.set_check_free_space(checked, cx));
                },
            ))))
            .child(card_row(false, colors).child(div().flex_1().child(switch_row(
                "create-collections",
                t!("settings.create_collections_title").to_string(),
                create_collections_note(),
                data.create_collections,
                colors,
                move |checked, cx| {
                    collections_entity.update(cx, |ctrl, cx| ctrl.set_create_collections(checked, cx));
                },
            ))))
            .child(card_row(false, colors).child(div().flex_1().child(concurrency_row(
                data.max_concurrent_downloads,
                data.max_concurrent_input,
                colors,
            )))))
}

/// The note under the collections switch, with the Windows Developer Mode
/// caveat only on Windows, where symlinks need it.
fn create_collections_note() -> String {
    if Platform.kind() == PlatformKind::Windows {
        format!("{} {}",
                t!("settings.create_collections_note"),
                t!("settings.create_collections_note_windows"))
    }
    else {
        t!("settings.create_collections_note").to_string()
    }
}

/// The "Max concurrent downloads" row: a title and a note, with the stepper
/// field on the right.
/// Shows the plain number until the root view attaches the shared field.
fn concurrency_row(max_concurrent_downloads: usize, input: Option<Entity<InputState>>,
                   colors: &ColorTokens)
                   -> AnyElement {
    let field: AnyElement = match input {
        Some(input_state) => NumberInput::new(&input_state).small().into_any_element(),
        None => div().text_sm()
                     .text_color(colors.text_primary)
                     .child(max_concurrent_downloads.to_string())
                     .into_any_element(),
    };
    field_row("max-concurrent-downloads-note",
              t!("settings.max_concurrent_downloads_title").to_string(),
              t!("settings.max_concurrent_downloads_note").to_string(),
              field,
              colors)
}
