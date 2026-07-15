## Why

The sidebar's Collections section (`sidebar-collections-and-collapsible`) currently renders
collections in whatever order the API returns them, with no way to reorder them. As a
user's collection count grows, finding a specific collection means scanning an unordered
list. Users need to sort collections the same way the catalog toolbar already lets them
sort library items.

## What Changes

- `CollectionEntry` gains a `created_at: i64` (Unix epoch seconds) field, populated from the
  product list's `dateCreated` attribute via the existing
  `util::datetime::parse_rfc3339_to_epoch` helper. `collections_sdk.rs`'s mapping and the
  collections disk cache (`collections_cache.rs`) both carry the new field; the cache reader
  defaults it to `0` for pre-existing cache files that predate this field (`#[serde(default)]`).
- New `CollectionSortMethod` enum (`Name`, `DateCreated`, `ItemCount`) in `util::sort`,
  reusing the existing `SortDirection` (`Ascending`/`Descending`) enum already used by the
  catalog toolbar sort.
- `LibraryController` gains `collection_sort: CollectionSortMethod` and
  `collection_sort_direction: SortDirection` fields (persisted via `UiPrefs`, mirroring
  `collections_open`) and a `set_collection_sort` / `set_collection_sort_direction` pair of
  methods that re-sort `self.collections` in place and emit `LibraryChanged`.
- `sidebar_view.rs`'s Collections section header gains a sort control (icon button with a
  dropdown menu, following `toolbar_view::render_sort_selector`'s `PopupMenuItem` pattern)
  offering the three sort methods plus ascending/descending toggle. Placed in the existing
  header suffix row alongside the count badge, search toggle, and add button.
- "Number of items" sorts by the same catalog-intersected count already computed per row
  (`member_ids` filtered against `catalog_ids`), not the raw API `item_count`, so the sort
  order matches what's displayed.

## Capabilities

### New Capabilities

- `sidebar-collections-sort`: The sidebar Collections section can be sorted by name, date
  created, or item count, in ascending or descending order, with the choice persisted
  across sessions.

### Modified Capabilities

_(none — this adds sorting on top of `sidebar-collections-and-collapsible`'s existing list
rendering without changing its requirements; that capability doesn't specify an order today)_

## Impact

- `dtrpg-ui/src/data/collection.rs` — add `created_at` field to `CollectionEntry`
- `dtrpg-ui/src/data/collections_cache.rs` — cache read/write carries `created_at`, defaults
  for legacy cache files
- `dtrpg-core/src/services/collections_sdk.rs` — populate `created_at` from
  `attributes.date_created` when mapping API responses to `CollectionEntry`
- `dtrpg-ui/src/services/collections.rs` — stub service populates `created_at` for tests
- `dtrpg-ui/src/util/sort.rs` — add `CollectionSortMethod`, add a `sort_collections` helper
- `dtrpg-ui/src/controllers/library.rs` — sort state fields, setters, re-sort on load/apply
- `dtrpg-ui/src/data/ui_prefs.rs` — persist `collection_sort` and `collection_sort_direction`
- `dtrpg-ui/src/ui/views/sidebar_view.rs` — sort control in the Collections header
- `dtrpg-ui/i18n/{en,de,fr}.yaml` — new sort menu label strings
