## Context

`LibraryItem` (`crates/dtrpg-ui/src/data/library.rs`) is currently a flattened, single-item-per-entry
model: one `format` string, one `size_mb` value, no per-file breakdown. `render_detail_tab_content`
already renders the entry tier (cover, title, description, actions, metadata) but its doc comment
states it "does not render a file list for multi-item entries: that requires a per-item file list
data model this crate does not yet have." `ItemOpener::open` already returns
`OpenError::MultipleFilesRequireSelection` for entries with more than one file, and every call site
(`detail_panel_view.rs`, `catalog_view.rs`, five occurrences) currently just logs a warning and
does nothing further.

## Goals / Non-Goals

**Goals:**

- Add a minimal per-item file model and wire it from the SDK's `OrderProductFile` through to
  `LibraryItem` (or a sibling type queried alongside it).
- Implement the item list panel and item metadata area inside `render_detail_tab_content` per
  `shared-catalog-entry-detail-view`, using `gpui-component::table::DataTable` or `Table`.
- Resolve the existing `MultipleFilesRequireSelection` gap by routing the user to the item list
  instead of a dead-end warning log.
- Add the item-count badge to `catalog_view.rs` list rows and grid tiles.

**Non-Goals:**

- Changing `item_popover_view.rs` beyond confirming it has no item list (no new popover features).
- Persisting selected-item state across sessions (ephemeral per `shared-catalog-entry-detail-view`).
- Redesigning `ItemOpener`'s single-file open path.

## Decisions

### 1. Add `LibraryItemFile` as a new type, keep `LibraryItem` flattened fields for backward compatibility

Add a new `LibraryItemFile` struct (`id`, `name`, `format`, `size_mb`, `download_state`) and a
`pub files: Vec<LibraryItemFile>` field on `LibraryItem`, populated from the SDK's
`OrderProductFile[]`. The existing flattened `format`/`size_mb`/`pages` fields on `LibraryItem`
remain as entry-tier summary fields (already used across catalog list/grid rendering) rather than
being removed.

**Rationale:** Avoids a breaking rewrite of every existing `LibraryItem` consumer (catalog list,
grid, sort, group, cache serialization) for a feature that only needs to add new information, not
replace old information. `files.len() > 1` becomes the multi-item detection rule.

**Alternative considered:** Replace flattened fields with a single-item derived from `files[0]`.
Rejected — touches every existing catalog rendering path for no functional gain, and risks
regressing `enhance-rust-library-ui-controls` and other in-flight catalog changes.

### 2. Item list panel uses `gpui-component::table::DataTable`

Per this repo's UI policy, the item list is implemented with `DataTable` (virtualized,
delegate-based), not a custom flex-row layout. Each row shows item name and item type (derived
from file extension via existing format-detection logic, if any, or the file's `format` field).

**Rationale:** Matches the standing repo instruction to always prefer `gpui-component` tabular
primitives over hand-rolled layouts, and Correctly handles alignment for a list that can scroll.

**Alternative considered:** A plain scrollable `div` list of custom rows. Rejected per repo UI
policy — `DataTable`/`Table` are required whenever content is tabular.

### 3. Resolve `MultipleFilesRequireSelection` by focusing the item list, not a new dialog

When `ItemOpener::open` returns `MultipleFilesRequireSelection` from a catalog-view context
(single/double-click open action outside the detail tab), the app opens the expanded detail tab
for that entry (if not already open) and scrolls/focuses the item list, rather than presenting a
separate file-picker dialog.

**Rationale:** Reuses the persistent item list this change already builds instead of introducing a
second, redundant multi-item selection UI. Matches the umbrella design decision that item
selection lives in the detail view, not a modal.

**Alternative considered:** A transient popover file picker triggered directly from the catalog row.
Rejected — duplicates the item list panel and diverges from `shared-catalog-entry-detail-view`,
which specifies the persistent panel as the single selection affordance.

## Risks / Trade-offs

- **Risk: SDK library-list responses may not include the `files` array by default (only on
  detail/prepare fetches)** → Mitigation: verify during implementation whether `order_products`
  list responses include `files`, or whether a per-entry detail fetch is required when the detail
  tab opens; if the latter, load `files` lazily when the tab opens rather than eagerly for the
  whole library.
- **Risk: Existing `MultipleFilesRequireSelection` call sites diverge in behavior once wired up**
  → Mitigation: consolidate the resolution behavior into one helper function used by all five
  call sites rather than duplicating tab-opening logic at each site.

## Migration Plan

1. Add `LibraryItemFile` and `LibraryItem.files`, sourced from `OrderProductFile`.
2. Implement the item list panel and item metadata area in `render_detail_tab_content`.
3. Wire the five `MultipleFilesRequireSelection` call sites to open/focus the item list.
4. Add the item-count badge to `catalog_view.rs`.
5. Verify the popover (`item_popover_view.rs`) is unaffected.

## Open Questions

- Does the currently cached/serialized `LibraryItem` format (used by the on-disk library cache)
  need a schema-version bump when `files` is added, or can it default to an empty vec for
  previously cached entries?
