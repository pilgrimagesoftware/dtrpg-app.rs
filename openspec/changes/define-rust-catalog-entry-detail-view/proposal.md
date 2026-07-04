## Why

`dtrpg-app` now defines `shared-catalog-entry-detail-view`, specifying the entry-tier/item-tier
layout and persistent item list that must fill the expanded detail tab. The Rust GPUI app's
`render_detail_tab_content` (`crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`) already renders
the entry tier but explicitly does not render a file list for multi-item entries — a gap called
out in its own doc comment and tracked against `add-rust-main-window-structure`. `ItemOpener`
already models the multi-file case (`OpenError::MultipleFilesRequireSelection`) but every call
site currently just logs a warning instead of prompting selection. `LibraryItem` has no per-item
file array to select from.

## What Changes

- Add a per-item file array to the Rust library data model (`LibraryItem` or a new adjacent type)
  mapping the SDK's `OrderProductFile` entries, so the app can distinguish single-item from
  multi-item entries and enumerate each item's name, format, size, and download state.
- Implement the persistent item list panel inside `render_detail_tab_content` for multi-item
  entries, using `gpui-component`'s `DataTable`/`Table` primitives per this repo's UI policy
  (never a custom flex-row column layout).
- Wire item list row selection to update an item metadata area in place within the same tab.
- Collapse item metadata inline into the entry tier for single-item entries (no item list rendered).
- Add the empty/prompt state for the item metadata area when no item is selected.
- Resolve `OpenError::MultipleFilesRequireSelection` call sites in `catalog_view.rs` and
  `detail_panel_view.rs` by directing the user into the item list instead of only logging a warning.
- Add an item-count badge to catalog list rows and grid tiles for multi-item entries.
- Confirm the single-click `Popover` (`item_popover_view.rs`) stays a lightweight entry-level
  summary and does not gain an item list, per `shared-catalog-entry-detail-view`.

## Capabilities

### New Capabilities

- `rust-catalog-entry-detail-view`: Defines the GPUI-specific data model, item list panel, and
  selection wiring implementing `shared-catalog-entry-detail-view` in the Rust app.

## Impact

- `dtrpg-app/rust/openspec`: Adds `rust-catalog-entry-detail-view`.
- Affected code: `crates/dtrpg-ui/src/data/library.rs` (per-item file model),
  `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs` (item list panel, item metadata area),
  `crates/dtrpg-ui/src/ui/views/catalog_view.rs` (item-count badge, multi-file open handling),
  `crates/dtrpg-ui/src/ui/views/item_popover_view.rs` (confirm no item list added),
  `crates/dtrpg-ui/src/util/item_opener.rs` (multi-file selection resolution).
- Depends on `dtrpg-app/openspec/changes/define-shared-catalog-entry-detail-view`.
- Depends on SDK coverage confirmed in `dtrpg/openspec/changes/multi-item-catalog-entry-detail`
  (`OrderProductFile` in `dtrpg-sdk/rust`).
