# rust-catalog-entry-detail-view Specification

## Purpose
TBD - created by archiving change define-rust-catalog-entry-detail-view. Update Purpose after archive.
## Requirements

### Requirement: Rust library data model MUST expose a per-item file array

`LibraryItem` MUST expose a `files: Vec<LibraryItemFile>` field, populated from the SDK's
`OrderProductFile` entries, so the app can determine item count and enumerate each item's name,
format, size, and download state.

#### Scenario: Single-item entry has one file

- **WHEN** a catalog entry's `files` array contains exactly one entry
- **THEN** the Rust app treats it as a single-item entry

#### Scenario: Multi-item entry has more than one file

- **WHEN** a catalog entry's `files` array contains more than one entry
- **THEN** the Rust app treats it as a multi-item entry

### Requirement: Rust expanded detail tab MUST render a persistent item list for multi-item entries

`render_detail_tab_content` MUST render a persistent, scrollable item list using
`gpui-component`'s `DataTable` or `Table` primitives (per this repo's UI policy) when the entry's
`files` array contains more than one entry. Each row MUST show item name and item type.

#### Scenario: Rendering the item list

- **WHEN** the expanded detail tab is shown for a multi-item entry
- **THEN** the tab renders a `DataTable`/`Table`-based item list showing all items, each with name
  and type

#### Scenario: Single-item entry renders no item list

- **WHEN** the expanded detail tab is shown for a single-item entry
- **THEN** no item list is rendered; item metadata is shown inline in the entry tier

### Requirement: Rust item list selection MUST update item metadata in place

Selecting a row in the item list MUST update a dedicated item metadata area within the same
expanded detail tab, without closing the tab or opening a new one.

#### Scenario: Selecting an item

- **WHEN** the user selects a row in the item list
- **THEN** the item metadata area shows that item's name, type, format, file size, and download
  state

#### Scenario: No item selected

- **WHEN** the expanded detail tab for a multi-item entry is first opened
- **THEN** the item metadata area shows a prompt indicating an item should be selected

### Requirement: Rust multi-file open action MUST route to the item list instead of only logging

When `ItemOpener::open` returns `OpenError::MultipleFilesRequireSelection`, the Rust app MUST open
(or focus, if already open) the expanded detail tab for that entry and bring the item list into
view, rather than only logging a warning.

#### Scenario: Attempting to open a multi-item entry directly from the catalog view

- **WHEN** the user triggers an open action on a multi-item entry from the catalog list or grid
- **THEN** the Rust app opens or focuses the expanded detail tab for that entry so the user can
  select a specific item

### Requirement: Rust catalog list and grid MUST show an item-count badge for multi-item entries

`catalog_view.rs` MUST render a visible item-count indicator on list rows and grid tiles for
entries whose `files` array contains more than one entry, and MUST NOT render it for single-item
entries.

#### Scenario: Multi-item entry in the catalog list or grid

- **WHEN** a catalog entry has more than one file
- **THEN** its list row or grid tile shows a visible item-count indicator

#### Scenario: Single-item entry in the catalog list or grid

- **WHEN** a catalog entry has exactly one file
- **THEN** no item-count indicator is shown

### Requirement: Rust popover detail view SHALL NOT gain an item list

`item_popover_view.rs` SHALL remain a lightweight entry-level summary and SHALL NOT render an item
list or item-selection control, regardless of item count.

#### Scenario: Single-clicking a multi-item entry

- **WHEN** the user single-clicks a catalog entry with more than one file
- **THEN** the popover shows entry-level summary information only, without an item list
