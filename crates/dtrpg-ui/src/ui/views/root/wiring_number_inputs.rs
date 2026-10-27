//! Step handling for the Settings window's `NumberInput` fields.
//!
//! `InputState` steps its own value and emits `InputEvent::Change` when the
//! field has a step; it emits `NumberInputEvent::Step` only when it cannot
//! (no step set, or the current text is not a valid number). These
//! subscriptions apply the step against the controller's own bounds, so the
//! stored setting and the displayed text stay in range either way.

use dtrpg_core::data::constants::{
    DEFAULT_UI_TEXT_SIZE, MAX_UI_TEXT_SCALE, MIN_UI_TEXT_SCALE, UI_TEXT_SCALE_STEP,
};
use gpui_kit::component::input::{NumberInputEvent, StepAction};

use super::*;
use crate::ui::views::settings::page::storage::{
    MAX_CONCURRENT_DOWNLOADS, MIN_CONCURRENT_DOWNLOADS,
};

/// Decimal places the text scale is shown and stored with.
const TEXT_SCALE_DECIMALS: usize = 1;

/// `current` moved one `step` in `action`'s direction, kept within
/// `min..=max` and rounded to [`TEXT_SCALE_DECIMALS`] so repeated steps do
/// not accumulate float error.
fn stepped(current: f32, action: StepAction, step: f32, min: f32, max: f32) -> f32 {
    let delta = match action {
        StepAction::Increment => step,
        StepAction::Decrement => -step,
    };
    let factor = 10_f32.powi(TEXT_SCALE_DECIMALS as i32);
    (((current + delta) * factor).round() / factor).clamp(min, max)
}

/// Writes `value` into `input` without announcing a change.
fn show(input: &Entity<InputState>, value: String, window: &mut gpui::Window, cx: &mut gpui::App) {
    input.update(cx, |state, cx| state.set_value(value, window, cx));
}

/// Applies `NumberInputEvent::Step` on the concurrency field to the storage
/// setting, within `MIN_CONCURRENT_DOWNLOADS..=MAX_CONCURRENT_DOWNLOADS`.
pub(super) fn subscribe_concurrency_step(window: &mut gpui::Window,
                                         cx: &mut Context<LibraryRootView>,
                                         input: &Entity<InputState>,
                                         settings: &Entity<SettingsController>) {
    let settings = settings.clone();
    cx.subscribe_in(input,
                    window,
                    move |_this, input_entity, event: &NumberInputEvent, window, cx| {
                        let NumberInputEvent::Step(action) = event;
                        let current = input_entity.read(cx)
                                                  .value()
                                                  .parse::<f32>()
                                                  .unwrap_or(MIN_CONCURRENT_DOWNLOADS as f32);
                        let next = stepped(current,
                                           *action,
                                           1.0,
                                           MIN_CONCURRENT_DOWNLOADS as f32,
                                           MAX_CONCURRENT_DOWNLOADS as f32)
                                   as usize;
                        settings.update(cx, |ctrl, cx| ctrl.set_max_concurrent_downloads(next, cx));
                        show(input_entity, next.to_string(), window, cx);
                    })
      .detach();
}

/// Sets the UI text scale to `scale` and shows it in `input`.
fn apply_text_scale(library: &Entity<LibraryController>, input: &Entity<InputState>, scale: f32,
                    window: &mut gpui::Window, cx: &mut gpui::App) {
    library.update(cx, |ctrl, cx| {
               ctrl.set_ui_text_size(px(scale * DEFAULT_UI_TEXT_SIZE), cx);
           });
    show(input, format!("{scale:.TEXT_SCALE_DECIMALS$}"), window, cx);
}

/// The UI text scale now in effect.
fn current_text_scale(cx: &gpui::App) -> f32 {
    cx.global::<LibriTheme>().fonts.ui_text_size.as_f32() / DEFAULT_UI_TEXT_SIZE
}

/// Builds the text scale field, shown in tenths, and wires it: a step moves
/// the scale by `UI_TEXT_SCALE_STEP`, and leaving the field applies whatever
/// was typed. Both clamp to `MIN_UI_TEXT_SCALE..=MAX_UI_TEXT_SCALE`.
///
/// No `min`/`max` is set on the field itself: that would make it step on its
/// own, bypassing the controller that persists the scale.
pub(super) fn wire_text_scale_input(window: &mut gpui::Window,
                                    cx: &mut Context<LibraryRootView>,
                                    library: &Entity<LibraryController>,
                                    settings: &Entity<SettingsController>) {
    let scale = current_text_scale(cx);
    let input = cx.new(|cx| {
                      let mut state =
            InputState::new(window, cx).default_value(format!("{scale:.TEXT_SCALE_DECIMALS$}"));
                      state.set_step(None, window, cx);
                      state
                  });
    let library_for_step = library.clone();
    cx.subscribe_in(&input,
                    window,
                    move |_this, input_entity, event: &NumberInputEvent, window, cx| {
                        let NumberInputEvent::Step(action) = event;
                        let next = stepped(current_text_scale(cx),
                                           *action,
                                           UI_TEXT_SCALE_STEP,
                                           MIN_UI_TEXT_SCALE,
                                           MAX_UI_TEXT_SCALE);
                        apply_text_scale(&library_for_step, input_entity, next, window, cx);
                    })
      .detach();
    let library_for_blur = library.clone();
    cx.subscribe_in(&input,
                    window,
                    move |_this, input_entity, event: &InputEvent, window, cx| {
                        if !matches!(event, InputEvent::Blur) {
                            return;
                        }
                        let typed = input_entity.read(cx)
                                                .value()
                                                .parse::<f32>()
                                                .unwrap_or_else(|_| current_text_scale(cx));
                        let next = typed.clamp(MIN_UI_TEXT_SCALE, MAX_UI_TEXT_SCALE);
                        apply_text_scale(&library_for_blur, input_entity, next, window, cx);
                    })
      .detach();
    settings.update(cx, |ctrl, _cx| ctrl.set_text_scale_input(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stepped_moves_one_step_in_the_action_direction() {
        assert_eq!(stepped(1.0, StepAction::Increment, 0.1, 0.5, 2.0), 1.1);
        assert_eq!(stepped(1.0, StepAction::Decrement, 0.1, 0.5, 2.0), 0.9);
    }

    #[test]
    fn stepped_clamps_to_the_bounds() {
        assert_eq!(stepped(2.0, StepAction::Increment, 0.1, 0.5, 2.0), 2.0);
        assert_eq!(stepped(0.5, StepAction::Decrement, 0.1, 0.5, 2.0), 0.5);
    }

    #[test]
    fn stepped_does_not_accumulate_float_error() {
        let up = (0..7).fold(1.0, |v, _| stepped(v, StepAction::Increment, 0.1, 0.5, 2.0));
        assert_eq!(up, 1.7);
    }
}
