//! Builds and wires the text inputs and font pickers created once in
//! [`LibraryRootView::new`]: the Settings window's form fields (which live
//! for the app session, not just while the settings window is open — see
//! `wire_settings_inputs`) and the main window's own search/bulk/collection
//! inputs (`build_misc_inputs`).

use super::*;

/// Builds every input and font-picker entity the Settings window's pages
/// read, subscribes each to push its value into the `settings` controller,
/// and attaches it via the matching `SettingsController::set_*` call — so
/// these survive the settings window closing and reopening within the
/// session.
pub(super) fn wire_settings_inputs(window: &mut gpui::Window, cx: &mut Context<LibraryRootView>,
                                   settings: &Entity<SettingsController>) {
    let email_initial = settings.read(cx).email_draft().to_owned();
    let email_input = cx.new(|cx| {
                            let mut state = InputState::new(window, cx)
            .placeholder(t!("settings.email_placeholder").to_string());
                            if !email_initial.is_empty() {
                                state = state.default_value(&email_initial);
                            }
                            state
                        });
    let settings_for_email = settings.clone();
    cx.subscribe(&email_input,
                 move |_this, input_entity, event: &InputEvent, cx| {
                     if matches!(event, InputEvent::Change) {
                         let value = input_entity.read(cx).value().to_string();
                         settings_for_email.update(cx, |ctrl, cx| ctrl.set_email_draft(value, cx));
                     }
                 })
      .detach();
    settings.update(cx, |ctrl, _cx| ctrl.set_email_input(email_input));

    let password_input = cx.new(|cx| {
                               InputState::new(window, cx)
            .placeholder(t!("settings.password_placeholder").to_string())
            .masked(true)
                           });
    let settings_for_password = settings.clone();
    cx.subscribe(&password_input,
                 move |_this, input_entity, event: &InputEvent, cx| {
                     if matches!(event, InputEvent::Change) {
                         let value = input_entity.read(cx).value().to_string();
                         settings_for_password.update(cx, |ctrl, cx| {
                                                  ctrl.set_password_draft(value, cx)
                                              });
                     }
                 })
      .detach();
    settings.update(cx, |ctrl, _cx| ctrl.set_password_input(password_input));

    // The Anthropic key is saved to the keychain when the field loses focus,
    // and the field is cleared so the key is not left on screen.
    let anthropic_key_input = cx.new(|cx| {
                                    InputState::new(window, cx)
            .placeholder(t!("agent.summary_key_placeholder").to_string())
            .masked(true)
                                });
    let settings_for_key = settings.clone();
    cx.subscribe_in(&anthropic_key_input,
                    window,
                    move |_this, input_entity, event: &InputEvent, window, cx| {
                        if matches!(event, InputEvent::Blur) {
                            let key = input_entity.read(cx).value().to_string();
                            let saved = settings_for_key.update(cx, |ctrl, cx| {
                                                            ctrl.save_anthropic_key(&key, cx)
                                                        });
                            if saved {
                                input_entity.update(cx, |state, cx| {
                                                state.set_value("", window, cx)
                                            });
                            }
                        }
                    })
      .detach();
    settings.update(cx, |ctrl, _cx| {
                ctrl.set_anthropic_key_input(anthropic_key_input)
            });

    let theme_editor_input = cx.new(|cx| {
                                   gpui_kit::component::input::EditorState::new(window, cx)
                                        .language("toml")
                                        .line_number(false)
                               });
    settings.update(cx, |ctrl, _cx| {
                ctrl.set_theme_editor_input(theme_editor_input)
            });

    let storage_path_placeholder = {
        use dtrpg_core::data::storage::StorageConfig;
        StorageConfig::load().root_path()
                             .to_string_lossy()
                             .into_owned()
    };
    let storage_path_input =
        cx.new(|cx| InputState::new(window, cx).default_value(&storage_path_placeholder));
    let settings_for_storage = settings.clone();
    cx.subscribe(&storage_path_input,
                 move |_this, input_entity, event: &InputEvent, cx| {
                     if matches!(event, InputEvent::Change) {
                         let value = input_entity.read(cx).value().to_string();
                         settings_for_storage.update(cx, |ctrl, cx| {
                                                 ctrl.set_storage_path_draft(value, cx)
                                             });
                     }
                 })
      .detach();
    settings.update(cx, |ctrl, _cx| {
                ctrl.set_storage_path_input(storage_path_input)
            });

    let recently_updated_window_days_initial = {
        use dtrpg_core::data::storage::StorageConfig;
        StorageConfig::load().recently_updated_window_days()
    };
    let recently_updated_window_input = cx.new(|cx| {
                                              InputState::new(window, cx)
            .default_value(recently_updated_window_days_initial.to_string())
            .min(f64::from(
                dtrpg_core::data::constants::RECENTLY_UPDATED_WINDOW_MIN_DAYS,
            ))
            .max(f64::from(
                dtrpg_core::data::constants::RECENTLY_UPDATED_WINDOW_MAX_DAYS,
            ))
                                          });
    let settings_for_window = settings.clone();
    cx.subscribe_in(&recently_updated_window_input,
                    window,
                    move |_this, input_entity, event: &InputEvent, window, cx| {
                        match event {
                            InputEvent::Change => {
                                let value = input_entity.read(cx).value().to_string();
                                if let Ok(days) = value.parse::<u32>() {
                                    settings_for_window.update(cx, |ctrl, cx| {
                                        ctrl.set_recently_updated_window_days(days, cx);
                                    });
                                }
                            }
                            InputEvent::Blur => {
                                // `NumberInput`'s own min/max clamp only
                                // corrects the
                                // displayed text in some focus-loss paths;
                                // explicitly
                                // re-syncing the field to the persisted
                                // (already-clamped)
                                // value on every blur guarantees the
                                // display never shows an
                                // out-of-range number regardless of that
                                // widget-internal
                                // timing.
                                let clamped = settings_for_window.read(cx)
                                                                 .snapshot()
                                                                 .recently_updated_window_days;
                                let current = input_entity.read(cx).value().to_string();
                                if current != clamped.to_string() {
                                    input_entity.update(cx, |state, cx| {
                                                    state.set_value(clamped.to_string(),
                                                                    window,
                                                                    cx);
                                                });
                                }
                            }
                            _ => {}
                        }
                    })
      .detach();
    settings.update(cx, |ctrl, _cx| {
                ctrl.set_recently_updated_window_input(recently_updated_window_input)
            });

    let max_concurrent_downloads_initial = {
        use dtrpg_core::data::storage::StorageConfig;
        StorageConfig::load().max_concurrent_downloads()
    };
    let max_concurrent_downloads_input = cx.new(|cx| {
                                               use crate::ui::views::settings_storage_view::{
                                                   MAX_CONCURRENT_DOWNLOADS,
                                                   MIN_CONCURRENT_DOWNLOADS,
                                               };
                                               InputState::new(window, cx)
            .default_value(max_concurrent_downloads_initial.to_string())
            .min(MIN_CONCURRENT_DOWNLOADS as f64)
            .max(MAX_CONCURRENT_DOWNLOADS as f64)
                                           });
    let settings_for_concurrency_input = settings.clone();
    cx.subscribe(&max_concurrent_downloads_input,
                 move |_this, input_entity, event: &InputEvent, cx| {
                     if matches!(event, InputEvent::Change) {
                         let value = input_entity.read(cx).value().to_string();
                         if let Ok(n) = value.parse::<usize>() {
                             settings_for_concurrency_input.update(cx, |ctrl, cx| {
                                 ctrl.set_max_concurrent_downloads(n, cx);
                             });
                         }
                     }
                 })
      .detach();
    wiring_number_inputs::subscribe_concurrency_step(window,
                                                     cx,
                                                     &max_concurrent_downloads_input,
                                                     settings);
    settings.update(cx, |ctrl, _cx| {
                ctrl.set_max_concurrent_downloads_input(max_concurrent_downloads_input)
            });

    // One value field serves every kind of placement rule. Its text is kept in
    // the controller as it is typed, so the page can suggest names from it.
    let rule_value_input = cx.new(|cx| InputState::new(window, cx).default_value(""));
    let settings_for_rule = settings.clone();
    cx.subscribe(&rule_value_input,
                 move |_this, input_entity, event: &InputEvent, cx| {
                     if matches!(event, InputEvent::Change) {
                         let value = input_entity.read(cx).value().to_string();
                         settings_for_rule.update(cx, |ctrl, cx| ctrl.set_rule_value(value, cx));
                     }
                 })
      .detach();
    settings.update(cx, |ctrl, _cx| ctrl.set_rule_editor_input(rule_value_input));
}

/// Inputs owned by the main window itself (not the Settings window) that
/// need to survive into [`LibraryRootView`]'s fields: search, bulk-selection,
/// and the sidebar/dialog inline inputs.
pub(super) struct MiscInputs {
    pub(super) search_input:                Entity<InputState>,
    pub(super) bulk_pattern_input:          Entity<InputState>,
    pub(super) bulk_pattern_field:          Rc<Cell<SelectionField>>,
    pub(super) collection_name_input:       Entity<InputState>,
    pub(super) file_opener_extension_input: Entity<InputState>,
    pub(super) publisher_search_input:      Entity<InputState>,
    pub(super) collection_search_input:     Entity<InputState>,
}

pub(super) fn build_misc_inputs(window: &mut gpui::Window, cx: &mut Context<LibraryRootView>,
                                controller: &Entity<LibraryController>,
                                settings: &Entity<SettingsController>)
                                -> MiscInputs {
    let search_input =
        cx.new(|cx| InputState::new(window, cx).placeholder(t!("search.placeholder").to_string()));

    let controller_for_search = controller.clone();
    cx.subscribe(&search_input,
                 move |_this, input_entity, event: &InputEvent, cx| {
                     if matches!(event, InputEvent::Change) {
                         let value = input_entity.read(cx).value().to_string();
                         controller_for_search.update(cx, |ctrl, cx| {
                                                  ctrl.set_search_query_debounced(value, cx);
                                              });
                     }
                 })
      .detach();

    let bulk_pattern_input = cx.new(|cx| {
                                   InputState::new(window, cx)
            .placeholder(t!("toolbar.select_matches_placeholder").to_string())
                               });
    let bulk_pattern_field = Rc::new(Cell::new(SelectionField::Title));

    let collection_name_input =
        cx.new(|cx| {
            InputState::new(window, cx).placeholder(t!("collections.name_placeholder").to_string())
        });
    let file_opener_extension_input = cx.new(|cx| {
                                            InputState::new(window, cx)
            .placeholder(t!("settings.file_opener_extension_placeholder").to_string())
                                        });

    let publisher_search_input =
        cx.new(|cx| {
            InputState::new(window, cx).placeholder(t!("search.publishers_placeholder").to_string())
        });
    let controller_for_publisher_search = controller.clone();
    cx.subscribe(&publisher_search_input,
                 move |_this, input_entity, event: &InputEvent, cx| {
                     if matches!(event, InputEvent::Change) {
                         let value = input_entity.read(cx).value().to_string();
                         controller_for_publisher_search.update(cx, |ctrl, cx| {
                                                            ctrl.set_publisher_search_query(value,
                                                                                            cx);
                                                        });
                     }
                 })
      .detach();

    let collection_search_input = cx.new(|cx| {
                                        InputState::new(window, cx)
            .placeholder(t!("search.collections_placeholder").to_string())
                                    });
    let controller_for_collection_search = controller.clone();
    cx.subscribe(&collection_search_input,
                 move |_this, input_entity, event: &InputEvent, cx| {
                     if matches!(event, InputEvent::Change) {
                         let value = input_entity.read(cx).value().to_string();
                         controller_for_collection_search.update(cx, |ctrl, cx| {
                             ctrl.set_collection_search_query(value, cx);
                         });
                     }
                 })
      .detach();

    // Commit the pending "add file opener" row on Enter; on blur, commit if
    // the user typed something, otherwise discard the pending row
    // (matches clicking elsewhere in most inline-add UIs — no
    // separate confirm step needed).
    let settings_for_ext = settings.clone();
    cx.subscribe(&file_opener_extension_input,
                 move |_this, input_entity, event: &InputEvent, cx| match event {
                     InputEvent::PressEnter { .. } => {
                         let value = input_entity.read(cx).value().to_string();
                         settings_for_ext.update(cx, |ctrl, cx| {
                                             ctrl.commit_pending_file_opener(&value, cx)
                                         });
                     }
                     InputEvent::Blur => {
                         let value = input_entity.read(cx).value().to_string();
                         settings_for_ext.update(cx, |ctrl, cx| {
                                             if value.trim().is_empty() {
                                                 ctrl.cancel_pending_file_opener(cx);
                                             }
                                             else {
                                                 ctrl.commit_pending_file_opener(&value, cx);
                                             }
                                         });
                     }
                     _ => {}
                 })
      .detach();

    MiscInputs { search_input,
                 bulk_pattern_input,
                 bulk_pattern_field,
                 collection_name_input,
                 file_opener_extension_input,
                 publisher_search_input,
                 collection_search_input }
}
