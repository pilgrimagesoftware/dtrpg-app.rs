## 1. Data model

- [ ] 1.1 Add `created_at: i64` to `CollectionEntry` (`crates/dtrpg-ui/src/data/collection.rs`)
  with `#[serde(default)]` so pre-existing cache JSON without the field still deserializes
- [ ] 1.2 Update `collections_sdk.rs`'s `list_collections` and `create_collection` mappings
  to populate `created_at` via `util::datetime::parse_rfc3339_to_epoch(&attrs.date_created)
  .unwrap_or(0)`
- [ ] 1.3 Update the `CollectionsStubService` in `services/collections.rs` to populate a
  non-zero `created_at` on its seeded entries so stub/dev builds can exercise date sorting
- [ ] 1.4 Update `collections_cache.rs` tests' `make_entry` helper to set `created_at`

## 2. Sort types and helper

- [ ] 2.1 Add `CollectionSortMethod { Name, DateCreated, ItemCount }` enum to
  `crates/dtrpg-ui/src/util/sort.rs`, deriving the same traits as `SortMethod`
  (`Clone, Copy, PartialEq, Eq, Default` with `Name` as `#[default]`)
- [ ] 2.2 Add `sort_collections(entries: &mut [CollectionEntry], method: CollectionSortMethod,
  direction: SortDirection, catalog_ids: &HashSet<u64>)` to `util/sort.rs` — item-count
  sort computes the same catalog-intersected count as `render_collection_row`
- [ ] 2.3 Unit tests in `util/sort.rs` for each `CollectionSortMethod` variant in both
  directions, including a tie-break case (equal item counts/dates) to confirm sort
  stability doesn't produce surprising reordering

## 3. Preferences

- [ ] 3.1 Add `collection_sort: Option<String>` and `collection_sort_direction:
  Option<String>` (or a small serializable enum) fields to `UiPrefsFile`
  (`data/ui_prefs.rs`)
- [ ] 3.2 Add `UiPrefs::collection_sort() -> CollectionSortMethod` and
  `collection_sort_direction() -> SortDirection` accessors, defaulting to `Name` /
  `Ascending`
- [ ] 3.3 Add `UiPrefs::save_collection_sort(method, direction)` following the
  `save_collections_open` pattern

## 4. Controller

- [ ] 4.1 Add `collection_sort: CollectionSortMethod` and `collection_sort_direction:
  SortDirection` fields to `LibraryController`, initialized from `UiPrefs` on construction
- [ ] 4.2 Add `set_collection_sort(&mut self, method: CollectionSortMethod, cx: &mut
  Context<Self>)` — updates the field, persists via `UiPrefs`, re-sorts `self.collections`
  in place using current `catalog_ids`, emits `LibraryChanged`
- [ ] 4.3 Add `set_collection_sort_direction(&mut self, direction: SortDirection, cx: &mut
  Context<Self>)` mirroring `set_collection_sort`
- [ ] 4.4 In `apply_collections`, sort the incoming entries using the controller's current
  `collection_sort` / `collection_sort_direction` before storing them, so freshly loaded
  collections respect the persisted sort choice immediately

## 5. Sidebar UI

- [ ] 5.1 In `sidebar_view.rs`, add a sort control (icon `Button` with `.dropdown_menu(...)`,
  following `toolbar_view::render_sort_selector`'s `PopupMenuItem` construction) into the
  Collections header suffix row, alongside the existing count badge / search toggle / add
  button
- [ ] 5.2 Menu offers "Name", "Date Created", "Item Count" items (checked state reflects
  current `collection_sort`) plus a separator and "Ascending"/"Descending" items (checked
  state reflects current `collection_sort_direction`), calling `set_collection_sort` /
  `set_collection_sort_direction` on click
- [ ] 5.3 Hide or disable the sort control when `collection_search.open` is true, matching
  how the header suffix already swaps to a search input in that state

## 6. Localization

- [ ] 6.1 Add `sidebar.collections_sort_name`, `sidebar.collections_sort_date_created`,
  `sidebar.collections_sort_item_count`, `sidebar.collections_sort_ascending`,
  `sidebar.collections_sort_descending`, `sidebar.collections_sort_tooltip` keys to
  `en.yaml`, `de.yaml`, `fr.yaml`

## 7. Verify

- [ ] 7.1 Run `cargo check --workspace --all-targets`
- [ ] 7.2 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 7.3 Run `cargo fmt --all -- --check`
- [ ] 7.4 Run `cargo test --workspace`
- [ ] 7.5 Manually run the app, add several collections at different times with different
  membership, and confirm each sort method/direction combination reorders the sidebar list
  correctly and survives an app restart
