//! File Openers settings section: extension → application overrides, laid out
//! like the Storage page's Locations: a card for each override, and a button
//! that adds another.

use std::path::{Path, PathBuf};

use dtrpg_core::data::file_openers::FileOpenerEntry;
use gpui::prelude::*;
use gpui::{
    AnyElement, App, Entity, IntoElement, ParentElement, SharedString, Styled, Window, div, px,
};
use gpui_kit::component::Disableable as _;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::tag::Tag;
use gpui_kit::component::{Icon, IconName, WindowExt as _};
use rust_i18n::t;

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::ui::views::settings_card::section_heading;
use crate::ui::widgets::{selectable_note, selectable_text};

/// Renders the File Openers settings section.
///
/// `pending_file_opener`, when `Some`, is the application path picked via the
/// native file dialog and awaiting an extension; a pending card renders at the
/// end of the list with `extension_input` focused for inline entry.
pub fn render_file_openers_section(
    file_openers: &[FileOpenerEntry], entity: Entity<SettingsController>, colors: &ColorTokens,
    extension_input: Entity<InputState>, pending_file_opener: Option<PathBuf>,
) -> impl IntoElement + 'static + use<> {
    let is_pending = pending_file_opener.is_some();
    let empty = file_openers.is_empty() && !is_pending;
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(colors.text_primary)
                        .child(t!("settings.file_openers_title")),
                )
                .child(selectable_note(
                    "file-openers-description",
                    t!("settings.file_openers_description").to_string(),
                    colors,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(section_heading(
                    t!("settings.file_openers_section").to_string(),
                    colors,
                ))
                .children(empty.then(|| render_empty_state(colors)))
                .children(
                    file_openers
                        .iter()
                        .map(|entry| render_entry_card(entry, &entity, colors)),
                )
                .children(pending_file_opener.map(|app_path| {
                    render_pending_card(&app_path, extension_input.clone(), entity.clone(), colors)
                }))
                .child(
                    div()
                        .flex()
                        .child(add_button(entity, extension_input, is_pending)),
                ),
        )
}

fn render_empty_state(colors: &ColorTokens) -> impl IntoElement + 'static {
    div()
        .flex()
        .items_center()
        .justify_center()
        .py(px(24.0))
        .child(
            div()
                .text_sm()
                .text_color(colors.text_tertiary)
                .child(t!("settings.file_openers_empty")),
        )
}

/// A bordered card with an application icon at its left, like a location.
fn card_shell(id: impl Into<gpui::ElementId>, colors: &ColorTokens) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(12.0))
        .px(px(16.0))
        .py(px(12.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(colors.border)
        .child(
            Icon::empty()
                .path("icons/app-window.svg")
                .text_color(colors.text_secondary),
        )
}

/// The in-progress "add file opener" card: an editable, focused extension
/// input and the application's name, in place of the extension tag a saved
/// override has.
///
/// Escape cancels the pending add without persisting anything, and stops the
/// keystroke from bubbling to the settings panel's own Escape-to-close handler.
fn render_pending_card(
    app_path: &Path, extension_input: Entity<InputState>, entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> AnyElement {
    let entity_escape = entity.clone();
    card_shell("file-opener-pending-row", colors)
        .on_key_down(move |event, _window, cx| {
            if event.keystroke.key == "escape" {
                cx.stop_propagation();
                entity_escape.update(cx, |ctrl, cx| ctrl.cancel_pending_file_opener(cx));
            }
        })
        .child(div().w(px(110.0)).child(Input::new(&extension_input)))
        .child(
            div().flex_1().min_w_0().child(
                selectable_text(
                    "file-opener-pending-app-name",
                    app_path.to_string_lossy().into_owned(),
                )
                .text_sm()
                .text_color(colors.text_secondary)
                .truncate(),
            ),
        )
        .child(
            Button::new("cancel-pending-file-opener")
                .ghost()
                .icon(IconName::Close)
                .tooltip(t!("settings.file_opener_add_cancel").to_string())
                .on_click(move |_, _, cx| {
                    entity.update(cx, |ctrl, cx| {
                        ctrl.cancel_pending_file_opener(cx);
                    });
                }),
        )
        .into_any_element()
}

/// One override: its extension and application, where the application is, and
/// a button to remove it. An application that is no longer there is flagged.
fn render_entry_card(
    entry: &FileOpenerEntry, entity: &Entity<SettingsController>, colors: &ColorTokens,
) -> AnyElement {
    let extension = entry.extension.clone();
    let missing = !entry.app_path.exists();
    let remove_entity = entity.clone();
    let remove_extension = extension.clone();
    card_shell(format!("file-opener-row-{extension}"), colors)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(
                            selectable_text(
                                format!("file-opener-app-name-{extension}"),
                                entry.app_path.to_string_lossy().into_owned(),
                            )
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(colors.text_primary)
                            .truncate(),
                        )
                        .child(Tag::secondary().outline().child(format!(".{extension}")))
                        .children(missing.then(|| {
                            div()
                                .text_xs()
                                .text_color(colors.warning_text)
                                .child(format!("⚠ {}", t!("settings.file_opener_app_not_found")))
                        })),
                ),
        )
        .child(
            Button::new(SharedString::from(format!("remove-opener-{extension}")))
                .ghost()
                .icon(Icon::empty().path("icons/trash-2.svg"))
                .tooltip(t!("settings.file_opener_remove_tooltip").to_string())
                .on_click(move |_, window, cx| {
                    confirm_remove(&remove_entity, &remove_extension, window, cx);
                }),
        )
        .into_any_element()
}

/// Asks for confirmation, then removes the override for `extension`.
fn confirm_remove(
    entity: &Entity<SettingsController>, extension: &str, window: &mut Window, cx: &mut App,
) {
    let (entity, ext) = (entity.clone(), extension.to_owned());
    window.open_alert_dialog(cx, move |alert, _, _| {
        let (entity, ext_for_ok) = (entity.clone(), ext.clone());
        alert
            .confirm()
            .title(t!("settings.file_opener_remove_confirm_title", ext = ext).to_string())
            .description(t!("settings.file_opener_remove_confirm_description").to_string())
            .on_ok(move |_, _window, cx| {
                entity.update(cx, |ctrl, cx| ctrl.remove_file_opener(&ext_for_ok, cx));
                true
            })
    });
}

/// The "Add file opener…" button. Off while an add is already in progress, so a
/// second click cannot start a new pending card and orphan the first.
fn add_button(
    entity: Entity<SettingsController>, extension_input: Entity<InputState>, _is_pending: bool,
) -> impl IntoElement + 'static {
    Button::new("add-file-opener")
        .outline()
        .icon(IconName::Plus)
        .label(t!("settings.file_opener_add").to_string())
        .on_click(move |_, window, cx| {
            pick_app_and_begin_add(&entity, &extension_input, window, cx);
        })
}

/// Runs the native application picker and, if an app was chosen, starts the
/// inline "add file opener" flow: resets and focuses `extension_input`,
/// reclaims OS key-window status (the native panel can steal it), and tells
/// the controller to render a pending row for the picked app.
///
/// The picker is presented asynchronously (`AsyncFileDialog`) instead of
/// blocking this handler: a blocking `rfd::FileDialog` runs AppKit's modal loop
/// while the handler still holds gpui's `App` borrow, and a foreground task
/// resuming mid-modal aborts in `AppCell::borrow_mut` (see issue #257). The
/// post-pick UI work therefore goes through the window context's `update`
/// rather than the borrowed `window`/`cx` handed to this function.
fn pick_app_and_begin_add(
    entity: &Entity<SettingsController>, extension_input: &Entity<InputState>, window: &mut Window,
    cx: &mut App,
) {
    let entity = entity.clone();
    let extension_input = extension_input.clone();
    window
        .to_async(cx)
        .spawn(async move |cx| {
            let picked = rfd::AsyncFileDialog::new()
                .add_filter(
                    t!("settings.file_opener_app_filter_name").to_string(),
                    &["app"],
                )
                .set_directory("/Applications")
                .pick_file()
                .await;
            let Some(handle) = picked else {
                return;
            };
            let app_path = handle.path().to_path_buf();

            cx.update(|window, cx| {
                extension_input.update(cx, |state, cx| state.set_value("", window, cx));
                // The native panel can leave the GPUI window without OS
                // key-window status. Without reclaiming it here, gpui's
                // focus tracking still thinks the handle is focused but no
                // keyboard events arrive, so the extension input below
                // silently refuses focus.
                window.activate_window();
            })
            .ok();

            entity.update(cx, |ctrl, cx| ctrl.begin_add_file_opener(app_path, cx));

            cx.update(|window, cx| {
                extension_input.update(cx, |state, cx| state.focus(window, cx));
            })
            .ok();
        })
        .detach();
}

// ── Helpers
// ───────────────────────────────────────────────────────────────────

/// Derives a display-friendly application name from the path.
fn app_name_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}
