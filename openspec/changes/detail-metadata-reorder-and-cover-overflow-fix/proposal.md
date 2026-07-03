## Why

The detail tab's metadata table and cover layout have accumulated ordering issues from
earlier iterations: file size and release date each sit on their own row instead of next
to their related field, category is not visually distinguished from other metadata, and
the fixed-width cover column can overflow past the tab content area into the sidebar at
narrow window widths because the row container has no minimum-width constraint.

## What Changes

- `render_metadata_table` groups related fields onto the same row: system paired with
  release date, format paired with file size. Category moves to the last row and its
  value is prefixed with a folder icon.
- The detail tab's outer `.flex_row()` container gains `.min_w_0()` (and the cover
  column keeps `.flex_none()`) so the fixed-width cover can no longer force the row to
  overflow its parent bounds; content that would overflow scrolls or clips instead of
  painting over the sidebar.

## Capabilities

### New Capabilities

- `detail-metadata-field-order`: Detail tab metadata table rows pair system/release-date
  and format/file-size, with category last and icon-prefixed.

### Modified Capabilities

- `rust-main-window-library-layout`: Detail tab cover column is width-constrained so it
  cannot overflow into the sidebar at narrow tab widths.

## Impact

- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: `render_metadata_table` restructured
  to a 2-column `DescriptionList`; outer container gains `.min_w_0()`.
