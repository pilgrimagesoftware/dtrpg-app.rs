//! Native Integration settings section: how Libri works with the operating
//! system. For now, whether tags move between the library and files' tags.
//!
//! Libri keeps its own index as the source of truth and projects what it can
//! onto the operating system. What the platform cannot do is left out, not
//! faked: where files keep no tags, the switches are replaced by a note.

use dtrpg_core::data::native_preferences::NativePreferencesFile;
use dtrpg_core::platform::{Capabilities, Platform, PlatformIntegration};
use gpui::prelude::*;
use gpui::{AnyElement, Entity, SharedString, div, px};
use gpui_kit::component::switch::Switch;
use rust_i18n::t;

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::ui::widgets::{selectable_note, small_caps_text};

/// Renders the operating-system integration part of the Advanced page: what
/// Libri does with the system's own features, then what this system can do.
pub fn render_native_section(entity: Entity<SettingsController>, native: NativePreferencesFile,
                             colors: &ColorTokens, label_font_family: &str)
                             -> impl IntoElement + 'static + use<> {
    let tags: AnyElement = if Platform.capabilities().filesystem_tags {
        tag_switches(entity.clone(), native, colors).into_any_element()
    }
    else {
        selectable_note("native-tags-unavailable",
                        t!("settings.native_tags_unavailable").to_string(),
                        colors).into_any_element()
    };
    let heading = |text: String| {
        div().font_family(label_font_family.to_string())
             .text_sm()
             .text_color(colors.text_primary)
             .child(small_caps_text(text))
    };

    div().flex()
         .flex_col()
         .gap(px(16.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(t!("settings.native_title").to_string()))
         .child(selectable_note("native-intro",
                                t!("settings.native_intro").to_string(),
                                colors))
         .child(div().flex()
                     .flex_col()
                     .gap(px(12.0))
                     .child(heading(t!("settings.native_tags_heading").to_string()))
                     .child(tags))
         .child(div().flex()
                     .flex_col()
                     .gap(px(12.0))
                     .child(heading(t!("settings.native_windows_heading").to_string()))
                     .child(window_restore_setting(entity,
                                                   native.reopen_windows_on_launch,
                                                   colors)))
         .child(div().flex()
                     .flex_col()
                     .gap(px(12.0))
                     .child(heading(t!("settings.native_capabilities_heading").to_string()))
                     .child(capability_matrix(Platform.capabilities(), colors)))
}

/// How windows reopen at launch. macOS has its own system setting, which
/// Libri follows, so only a note is shown there; other platforms get a switch.
fn window_restore_setting(entity: Entity<SettingsController>, reopen: bool, colors: &ColorTokens)
                          -> AnyElement {
    if cfg!(target_os = "macos") {
        return selectable_note("native-windows-follows-system",
                               t!("settings.native_windows_follows_system").to_string(),
                               colors).into_any_element();
    }
    switch_row("native-reopen-windows",
               t!("settings.native_reopen_title").to_string(),
               t!("settings.native_reopen_note").to_string(),
               reopen,
               colors,
               move |checked, cx| {
                   entity.update(cx, |ctrl, cx| {
                             ctrl.set_reopen_windows_on_launch(checked, cx)
                         });
               })
}

/// The three tag switches, each with a title and what it does.
fn tag_switches(entity: Entity<SettingsController>, native: NativePreferencesFile,
                colors: &ColorTokens)
                -> impl IntoElement + 'static + use<> {
    let write = entity.clone();
    let read = entity.clone();
    let launch = entity;
    div().flex()
         .flex_col()
         .gap(px(16.0))
         .child(switch_row("native-write-tags",
                           t!("settings.native_write_title").to_string(),
                           t!("settings.native_write_note").to_string(),
                           native.write_tags_to_files,
                           colors,
                           move |checked, cx| {
                               write.update(cx, |ctrl, cx| {
                                        ctrl.set_write_tags_to_files(checked, cx)
                                    });
                           }))
         .child(switch_row("native-read-tags",
                           t!("settings.native_read_title").to_string(),
                           t!("settings.native_read_note").to_string(),
                           native.read_tags_from_files,
                           colors,
                           move |checked, cx| {
                               read.update(cx, |ctrl, cx| {
                                       ctrl.set_read_tags_from_files(checked, cx)
                                   });
                           }))
         .child(switch_row("native-reconcile-on-launch",
                           t!("settings.native_launch_title").to_string(),
                           t!("settings.native_launch_note").to_string(),
                           native.reconcile_on_launch,
                           colors,
                           move |checked, cx| {
                               launch.update(cx, |ctrl, cx| {
                                         ctrl.set_reconcile_on_launch(checked, cx)
                                     });
                           }))
}

/// One switch with its title and a note beneath, in the layout of the Storage
/// page's "Create collections" row.
pub(crate) fn switch_row(id: &'static str, title: String, note: String, checked: bool,
                         colors: &ColorTokens,
                         on_change: impl Fn(bool, &mut gpui::App) + 'static)
                         -> AnyElement {
    div().flex()
         .flex_col()
         .gap(px(4.0))
         .child(div().flex()
                     .items_center()
                     .justify_between()
                     .child(div().text_sm()
                                 .font_weight(gpui::FontWeight::SEMIBOLD)
                                 .text_color(colors.text_primary)
                                 .child(SharedString::from(title)))
                     .child(Switch::new(id).checked(checked)
                                           .on_click(move |checked, _window, cx| {
                                               on_change(*checked, cx)
                                           })))
         .child(selectable_note(format!("{id}-note"), note, colors))
         .into_any_element()
}

/// What this operating system lets Libri do, one row per ability, each marked
/// as available or not on this platform. Abilities that are missing are shown,
/// not hidden, so it is clear why a feature does nothing here.
fn capability_matrix(capabilities: Capabilities, colors: &ColorTokens)
                     -> impl IntoElement + 'static + use<> {
    let rows = [(t!("settings.native_capability_file_state"), capabilities.file_state),
                (t!("settings.native_capability_materialize"), capabilities.materialize),
                (t!("settings.native_capability_tags"), capabilities.filesystem_tags),
                (t!("settings.native_capability_reveal"), capabilities.reveal)];
    div().flex()
         .flex_col()
         .gap(px(6.0))
         .children(rows.into_iter().map(|(name, available)| {
             let (status, color) = if available {
                 (t!("settings.native_capability_available"), colors.text_primary)
             }
             else {
                 (t!("settings.native_capability_unavailable"), colors.text_secondary)
             };
             div().flex()
                  .items_center()
                  .justify_between()
                  .text_xs()
                  .child(div().text_color(colors.text_primary).child(SharedString::from(name.to_string())))
                  .child(div().text_color(color).child(SharedString::from(status.to_string())))
         }))
}
