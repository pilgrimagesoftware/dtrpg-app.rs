Issue: https://github.com/pilgrimagesoftware/dtrpg-app.rs/issues/95

## Why

The activity panel shipped with three usability gaps: long labels are ellipsized with no way to see the full text, the list clips at 300 px and is not scrollable when items overflow, and the empty state (though present in code) is visually minimal and easy to overlook. Users need to be able to read complete activity information without the panel being the main focus of the screen.

## What Changes

- **Scrollable list**: Replace `overflow_y_hidden` with a scrollable container so all activity items are reachable.
- **Hover tooltip**: When hovering an activity item row, display a tooltip with the full label (and error message for error items) so truncated text is accessible without expanding the row.
- **Click to expand**: Clicking an activity item row toggles it between the default (single-line, truncated) and an expanded state (full text, word-wrapped) so users can pin the detail in view.
- **Empty state refinement**: The "No recent activity." placeholder is already in place; give it a more prominent visual treatment (larger icon, two-line copy) consistent with the catalog empty states.

## Capabilities

### New Capabilities

- `activity-panel-ux`: Scrollable item list, hover tooltip for full item text, click-to-expand item rows, and refined empty state in the activity panel.

### Modified Capabilities

## Impact

- `dtrpg-ui/src/ui/views/activity_panel_view.rs`: scrollable list, tooltip on hover, expanded-row rendering, improved empty state.
- `dtrpg-ui/src/controllers/activity.rs`: add `selected_id: Option<u64>` field and `select_activity(id)` / `deselect_activity()` methods to track which item is expanded.
- `dtrpg-ui/src/data/activity.rs`: `ActivitySnapshot` gains `selected_id: Option<u64>`.
- No changes to the service layer, SDK, or API contract.
