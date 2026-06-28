## Why

The activity panel button in the sidebar uses `text_xs()` to render its symbols (`↻`, `●`, `○`) and count label. At extra-small size these glyphs are difficult to read at a glance — the spinner arrow and dot are too small to be immediately recognisable as status indicators.

## What Changes

- Change the activity button's text size from `text_xs()` to `text_sm()` so the symbols are more legible without changing the button's padding or layout.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

_(none — this is a visual tweak with no spec-level behavior change)_

## Impact

- `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`: `render_activity_button` — one line change
