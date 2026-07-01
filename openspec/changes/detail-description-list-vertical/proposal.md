## Why

The metadata table in the detail panel uses a horizontal `DescriptionList` layout, which puts the label (e.g. "Pages") next to the value on the same line. A vertical layout -- label above value -- is more readable at the detail panel's narrow width and matches the convention used by similar panels in other apps.

## What Changes

- Switch `DescriptionList::new()` to `DescriptionList::vertical()` in `render_metadata_table` inside `detail_panel_view.rs`

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- None

## Impact

- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: one-line change in `render_metadata_table`
