## Why

Many DriveThruRPG products are delivered as a Zip archive bundling multiple files (PDF,
EPUB, maps, character sheets). The detail tab currently shows no file list at all for an
item (a known gap tracked since `add-rust-main-window-structure`), so a user has no way to
see what a downloaded Zip contains without leaving the app and opening it in Finder/Explorer.

## What Changes

- `LibraryItem` gains a per-item file list (name, size, and whether the entry is a Zip
  archive), so the detail tab can render one row per bundled file instead of a single
  "open"/"download" action pair.
- The expanded detail tab renders this file list beneath the existing metadata, one row per
  file, using the existing `gpui-component` row/description primitives already used
  elsewhere in the tab.
- Hovering a Zip file row opens an anchored popover (mirrors the existing
  `render_item_popover` anchored/deferred pattern) showing a scrollable list of the
  archive's internal entries, read directly from the on-disk Zip's central directory (no
  extraction).
- Clicking the Zip file row pins the popover open (it no longer closes on mouse-out); a
  second click, an explicit close, or navigating away from the detail tab dismisses it.
- The pinned/hover popover state is owned by the detail tab's own view state, not global
  window state, so switching to another tab or closing the detail tab always hides the
  popover — it never leaks into other tabs or the catalog view.
- Reading Zip contents is best-effort: unreadable or non-Zip files fail closed (no popover,
  or a popover showing an inline error), never a panic.

## Capabilities

### New Capabilities

- `detail-file-list`: Renders the list of files bundled with a downloaded catalog item in
  the expanded detail tab, one row per file with name and size.
- `zip-content-preview`: Hover-to-preview / click-to-pin popover that lists the internal
  entries of a Zip file row in the detail tab, scoped to that detail tab's visibility.

### Modified Capabilities

_(none — no existing spec currently governs detail tab file rendering)_

## Impact

- `crates/dtrpg-ui/src/data/library.rs`: `LibraryItem` gains a `files: Vec<ItemFile>` field
  (or equivalent) describing bundled files.
- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: renders the new file list rows in
  place of the current single download/open/reveal action row.
- New view module (e.g. `crates/dtrpg-ui/src/ui/views/zip_preview_popover.rs`): renders the
  anchored, scrollable zip-entry popover.
- `crates/dtrpg-ui/src/controllers/library.rs`: gains hover/pin state for the zip preview,
  scoped per detail tab, plus a zip-listing adapter call.
- New dependency: a Zip-reading crate (e.g. `zip`) added to `dtrpg-ui` (or a lower-level
  crate) to read a Zip's central directory without extracting it.
