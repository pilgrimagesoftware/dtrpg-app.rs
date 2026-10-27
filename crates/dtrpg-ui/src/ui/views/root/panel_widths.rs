//! The widths of the sidebar and the inspector pane: their limits, and
//! remembering them between runs.

use dtrpg_core::data::constants::{INSPECTOR_PANE_MAX_WIDTH, INSPECTOR_PANE_MIN_WIDTH};
use dtrpg_core::data::ui_state::UiState;

/// The narrowest the sidebar can be dragged to.
pub(super) const SIDEBAR_MIN_WIDTH: f32 = 180.0;

/// The widest the sidebar can be dragged to.
pub(super) const SIDEBAR_MAX_WIDTH: f32 = 361.0;

/// The sidebar width to start with: the saved one if it is within the limits,
/// else `default`. A width saved while the window was squeezed would otherwise
/// bring the sidebar back too thin.
pub(super) fn restored_sidebar_width(saved: Option<f32>, default: f32) -> f32 {
    saved.filter(|width| (SIDEBAR_MIN_WIDTH..=SIDEBAR_MAX_WIDTH).contains(width))
         .unwrap_or(default)
}

/// The inspector width to start with, by the same rule as
/// [`restored_sidebar_width`].
pub(super) fn restored_inspector_width(saved: Option<f32>, default: f32) -> f32 {
    saved.filter(|width| (INSPECTOR_PANE_MIN_WIDTH..=INSPECTOR_PANE_MAX_WIDTH).contains(width))
         .unwrap_or(default)
}

/// Saves the panel widths, unless they are outside their limits, as they are
/// while the window is being squeezed smaller than the panels can fit. `pane`
/// is `None` while the inspector is closed, which keeps its last width.
///
/// Reads and writes a file, so it is called from a background thread after the
/// drag has stopped.
pub(super) fn save_panel_widths(sidebar: f32, pane: Option<f32>) {
    if !(SIDEBAR_MIN_WIDTH..=SIDEBAR_MAX_WIDTH).contains(&sidebar) {
        return;
    }
    match pane {
        None => UiState::load().save_sidebar_width(sidebar),
        Some(pane) if (INSPECTOR_PANE_MIN_WIDTH..=INSPECTOR_PANE_MAX_WIDTH).contains(&pane) => {
            UiState::load().save_panel_widths(sidebar, pane);
        }
        Some(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_saved_width_outside_the_limits_is_not_restored() {
        assert!((restored_sidebar_width(Some(40.0), 250.0) - 250.0).abs() < f32::EPSILON);
        assert!((restored_sidebar_width(Some(500.0), 250.0) - 250.0).abs() < f32::EPSILON);
        assert!((restored_sidebar_width(Some(220.0), 250.0) - 220.0).abs() < f32::EPSILON);
        assert!((restored_sidebar_width(None, 250.0) - 250.0).abs() < f32::EPSILON);
    }

    #[test]
    fn the_inspector_width_is_held_to_its_limits_too() {
        let default = INSPECTOR_PANE_MIN_WIDTH + 50.0;
        assert!((restored_inspector_width(Some(1.0), default) - default).abs() < f32::EPSILON);
    }
}
