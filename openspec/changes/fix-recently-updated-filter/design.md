## Context

`SidebarFilter::RecentlyAdded` and `SectionCounts::recently_added` both test
`item.added_order <= RECENTLY_ADDED_THRESHOLD` (`= 90`). `added_order` is a
stable rank assigned when an item enters the catalog (lower is more recent),
not a timestamp — so this filter always shows "the 90 most-recently-added
items," never "items added or updated in some actual time window." `LibraryItem`
already carries `date_added: Option<i64>` and `date_updated: Option<i64>`
(both Unix epoch seconds; `date_updated` is `None` until something updates the
item post-creation), which is what a real date-based filter needs.

`item_matches_filter` (`crates/dtrpg-ui/src/util/matching.rs`) and
`section_counts` (`crates/dtrpg-ui/src/data/library.rs`) are the two call
sites that need the new predicate; both currently duplicate the same
`added_order <= RECENTLY_ADDED_THRESHOLD` check independently.

## Goals / Non-Goals

**Goals:**
- Replace the rank-based cutoff with a real 30-day window keyed off
  `max(date_added, date_updated)`.
- Rename `RecentlyAdded` → `RecentlyUpdated` throughout the codebase (variant,
  field, i18n key, label) to match the new semantics.
- Keep the "current time" comparison testable without wall-clock flakiness,
  matching the existing `reload_cooldown_active(meta, now_secs)` pattern.

**Non-Goals:**
- Changing `added_order`'s existing use elsewhere (e.g.
  `SortMethod::MostRecentlyAdded`'s sort ordering) — that's a distinct,
  unrelated sort concern and stays as-is.
- Backfilling `date_updated` for existing cached items that predate this
  change — `date_updated: None` falls back to `date_added`, which is already
  populated for every synced item.
- A dedicated "Catalog" settings tab. The window preference is a small,
  infrequently-touched numeric setting; it lives at the top of the existing
  Advanced tab rather than justifying a new tab for one control.

## Decisions

- **New pure function `item_recently_updated(item: &LibraryItem, now_secs:
  i64) -> bool`** in `crates/dtrpg-ui/src/util/matching.rs`, taking `now_secs`
  explicitly rather than calling `SystemTime::now()` internally — mirrors
  `reload_cooldown_active`'s existing testable-clock pattern in this codebase
  and lets unit tests assert exact boundary behavior (e.g. exactly 30 days
  ago) without mocking time.
- **Compare `max(date_added, date_updated)` against `now_secs -
  RECENTLY_UPDATED_WINDOW_SECS`.** An item with no `date_updated` (the common
  case — most items are never updated after creation) falls back to
  `date_added` via `Option::max`/`Iterator::max` over the two optional
  timestamps; an item with neither timestamp never matches (can't be "recent"
  without a date).
- **`item_recently_updated` takes `window_days: u32` instead of a fixed
  constant**, converting to seconds internally (`i64::from(window_days) *
  86_400`). `RECENTLY_UPDATED_WINDOW_DEFAULT_DAYS: u32 = 30`,
  `RECENTLY_UPDATED_WINDOW_MIN_DAYS: u32 = 7`,
  `RECENTLY_UPDATED_WINDOW_MAX_DAYS: u32 = 90` in `constants.rs` replace the
  old fixed `RECENTLY_ADDED_THRESHOLD`/`RECENTLY_UPDATED_WINDOW_SECS`.
- **Editable via `gpui_component::input::NumberInput`**, not a hand-rolled
  +/- button pair. `NumberInput` already provides an editable text field,
  built-in increment/decrement buttons (default step 1), and
  clamp-to-`min`/`max` on blur when `InputState::min`/`max` are set — reusing
  it avoids reimplementing bounded-numeric-field behavior the component
  library already has.
- **The bound is enforced once more, at the setter**
  (`StorageConfig::set_recently_updated_window_days`), which clamps to
  `[MIN, MAX]` before persisting — `NumberInput`'s own clamp only fires on
  blur (an in-range value can transiently commit while typing, e.g. "4" of
  "45"), and a hand-edited `storage.toml` could contain anything, so the
  data-layer clamp is the actual guarantee; the UI clamp is a nicety.
- **Persisted in `StorageConfig`/`storage.toml`**, not `CatalogViewPrefs`.
  `CatalogViewPrefs` (`app_config.toml`) captures ambient, continuously
  re-saved view state (whatever filter/sort was last active); this is a
  deliberate, infrequent Settings-page choice like
  `max_concurrent_downloads`, so it follows that field's exact
  load/set/persist pattern (including the same `SettingsChanged` broadcast
  from `SettingsController` to `LibraryController` via `root_view.rs`) rather
  than the view-state one.
- **`LibraryController` caches the resolved `recently_updated_window_days`**
  (loaded at construction, updated via `set_recently_updated_window_days`
  the same way `set_max_concurrent_downloads` already works), and threads it
  through `item_matches_filter`/`section_counts` alongside `now_secs` — so a
  Settings-page change take effect immediately without needing this
  controller to re-read `StorageConfig` on every filter evaluation.
- **`section_counts` calls the same `item_recently_updated` predicate** (with
  `now_secs` threaded in as a parameter) rather than re-deriving its own
  window check, so the sidebar badge count and the actual filtered list can
  never disagree.
- **Rename, don't alias.** `SidebarFilter::RecentlyAdded` →
  `RecentlyUpdated`, `SectionCounts::recently_added` → `recently_updated`,
  i18n key `sidebar.recently_added` → `sidebar.recently_updated`. No
  deprecated alias or backwards-compat shim — this is UI-only application
  state with no external persistence format depending on the enum variant
  name (the catalog cache serializes `LibraryItem`/`LibraryItemFile`, not
  `SidebarFilter`), so a rename is a same-session, no-migration change.

## Risks / Trade-offs

- [`date_added`/`date_updated` both `None`] → Item never matches
  `RecentlyUpdated`, same as it would have failed a hypothetical date check
  before; no existing code path leaves both `None` for a synced item (only
  test fixtures might), so this is a non-issue in practice but the predicate
  handles it explicitly rather than panicking or defaulting to "recent."
- [Renaming the i18n key without a fallback] → Any locale file not updated in
  this change would render a missing-key placeholder. Mitigated by updating
  `en.yaml`, `de.yaml`, and `fr.yaml` in the same change (all three are
  small, hand-maintained files with this exact key).
