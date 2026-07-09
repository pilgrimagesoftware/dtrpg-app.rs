## Context

Collections currently render in `self.collections` order, which is whatever
`CollectionsService::list_collections` returns (API page order, cache-restored order on
startup). `CollectionEntry` (`data/collection.rs`) only carries `id`, `name`, and
`member_ids` — there's no timestamp field, so "sort by date created" isn't possible without
first threading `dateCreated` through from the SDK's `ProductListAttributes` (which already
has it) into `CollectionEntry` and the on-disk collections cache.

The catalog toolbar already solved sort UI once: `util::sort::{SortMethod, SortDirection}`,
`toolbar_view::render_sort_selector`'s `PopupMenuItem` dropdown, and
`LibraryController::set_sort` / `set_sort_direction`. This change follows that shape for
collections rather than inventing a new one.

## Goals / Non-Goals

**Goals:**
- Sort the sidebar Collections list by name, date created, or item count, ascending or
  descending.
- Persist the chosen sort method and direction across app restarts (`UiPrefs`).
- Reuse the existing `SortDirection` enum and `PopupMenuItem` dropdown pattern instead of
  introducing new abstractions.

**Non-Goals:**
- Sorting or grouping *within* a collection (the items shown once a collection is
  selected) — that's the catalog view's existing sort, untouched here.
- Custom/manual collection ordering (drag-to-reorder). Only the three listed criteria.
- Backfilling `created_at` for collections cached before this change beyond a safe default
  of `0` (they'll sort as "oldest" until the next live fetch refreshes the cache).

## Decisions

- **Add `created_at: i64` (epoch seconds) to `CollectionEntry`, not a raw string.** Sorting
  a `String` copy of an RFC 3339 timestamp lexically works only if every value uses the same
  UTC offset; the API returns local offsets (`-05:00` in the observed payload), so string
  sort order isn't reliable. `util::datetime::parse_rfc3339_to_epoch` already exists and is
  used by the alert history view for the same reason — reuse it in `collections_sdk.rs`'s
  mapping rather than adding a second date-parsing path.
- **`#[serde(default)]` on `created_at` in `CollectionEntry`**, so the existing
  `collections_cache.rs` JSON cache (written before this field existed) still deserializes.
  Cached entries get `created_at: 0` until the next `load_collections` call overwrites the
  cache with real timestamps — acceptable since the cache is a startup-speed optimization,
  not a source of truth.
- **New `CollectionSortMethod` enum, reusing `SortDirection`.** `SortMethod` (catalog sort)
  and `CollectionSortMethod` (this change) sort different domain types (`LibraryItem` vs
  `CollectionEntry`) with different criteria (title/publisher/date-added/pages vs
  name/date-created/item-count) — a shared enum would need variants that don't apply to one
  side or the other. `SortDirection` has no domain-specific meaning, so it's shared as-is.
- **"Number of items" sorts by the catalog-intersected count, not the raw API
  `item_count`.** `render_collection_row` already computes
  `member_ids.iter().filter(|id| catalog_ids.contains(id)).count()` per row so the badge
  shown matches what's sorted; sorting by the unfiltered API count would let the displayed
  order disagree with the displayed numbers whenever a collection contains items outside
  the user's library.
- **Sort state lives on `LibraryController`, not a new controller.** It already owns
  `self.collections` and the existing `sort_direction` field for the catalog, so extending it
  with `collection_sort` / `collection_sort_direction` keeps sort state colocated with the
  data it orders, matching the catalog sort's existing placement.

## Risks / Trade-offs

- [Sorting by item count requires the catalog to have finished loading (`catalog_ids`); if
  collections load before the catalog, the count is briefly `0` for every row] → Already
  true today for the displayed count badge (not new to this change); re-sorting happens
  again when the catalog snapshot changes, per the existing `LibraryChanged` emission on
  catalog updates.
- [Cached `created_at: 0` values sort a stale-cached collection as "oldest" until the next
  live fetch] → Acceptable: the cache exists only to avoid an empty sidebar flash on
  startup, and every `load_collections` call overwrites it with live data within one
  background fetch.

## Migration Plan

No user-facing migration. The new `created_at` field defaults to `0` for any cache file
written before this change; the next successful `load_collections` fetch replaces those
entries with real timestamps. No schema versioning needed since the field is additive with
a serde default.
