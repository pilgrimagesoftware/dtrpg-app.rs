## Why

The sidebar's "Recently Added" filter (`SidebarFilter::RecentlyAdded`) doesn't
filter by date at all: `item_matches_filter` and `section_counts` both test
`item.added_order <= RECENTLY_ADDED_THRESHOLD` (`RECENTLY_ADDED_THRESHOLD =
90`), where `added_order` is a stable rank position, not a timestamp. The
section always shows the 90 most-recently-added items regardless of how long
ago they were added — a user with a small library sees their entire catalog
in this filter forever, and a user who adds nothing for months still sees the
same 90 items indefinitely. It also only reflects an item's *addition* to the
library, never a later update (e.g. a re-download or metadata refresh).

## What Changes

- `SidebarFilter::RecentlyAdded` is renamed to `SidebarFilter::RecentlyUpdated`
  (and `SectionCounts::recently_added` to `SectionCounts::recently_updated`).
- The filter predicate changes from an `added_order` rank cutoff to a real
  30-day date window: an item matches if `max(date_added, date_updated)` is
  within the last 30 days of the current time.
- The sidebar and toolbar label changes from "Recently Added" to "Recently
  Updated" (`i18n` key `sidebar.recently_added` renamed to
  `sidebar.recently_updated`, value updated in all three locales).
- `RECENTLY_ADDED_THRESHOLD` (rank-based) is removed and replaced with a
  duration constant for the default 30-day window.
- The window is now a user preference, adjustable from a Storage settings
  page stepper, bounded to 7-90 days (default 30). Persisted alongside the
  existing `max_concurrent_downloads`/`create_collections` settings in
  `storage.toml`, following the same load/set/persist/broadcast pattern.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `libri-sidebar`: The "Recently Added" smart section is renamed "Recently
  Updated" and its requirement changes from an implicit rank-based cutoff to
  an explicit recency window (default 30 days, user-adjustable 7-90) keyed
  off `date_added`/`date_updated`.
- `string-catalog`: The sidebar navigation label scenario's literal label set
  changes from "Recently Added" to "Recently Updated".
- `thumbnail-queue-concurrency`'s sibling Storage-settings-page conventions
  (bounded stepper, `storage.toml` persistence, `SettingsChanged` broadcast)
  are reused for the new preference; no capability spec changes there since
  the pattern itself isn't a requirement.

## Impact

- `crates/dtrpg-ui/src/util/filter.rs`: `SidebarFilter::RecentlyAdded` →
  `RecentlyUpdated`.
- `crates/dtrpg-ui/src/util/matching.rs`: `item_matches_filter`'s predicate for
  this variant switches from `added_order` to a date-window check.
- `crates/dtrpg-ui/src/data/library.rs`: `SectionCounts::recently_added` →
  `recently_updated`; `section_counts`'s corresponding filter updated to match.
- `crates/dtrpg-ui/src/data/constants.rs`: `RECENTLY_ADDED_THRESHOLD` removed,
  replaced with default/min/max day-count constants.
- `crates/dtrpg-ui/src/data/storage.rs`: `StorageConfig` gains a persisted,
  clamped `recently_updated_window_days` field alongside
  `max_concurrent_downloads`.
- `crates/dtrpg-ui/src/controllers/settings.rs`,
  `crates/dtrpg-ui/src/controllers/library.rs`: settings snapshot/setter and
  the `SettingsChanged`-driven sync into `LibraryController`, mirroring
  `max_concurrent_downloads`'s existing wiring.
- `crates/dtrpg-ui/src/ui/views/settings_storage_view.rs`: new stepper row
  for the window, reusing the concurrency stepper's layout.
- `crates/dtrpg-ui/src/ui/views/root_view.rs`: new `SettingsChanged`
  subscription propagating the window to `LibraryController`.
- `crates/dtrpg-ui/src/ui/views/toolbar_view.rs`,
  `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`: references to
  `SidebarFilter::RecentlyAdded` and the `sidebar.recently_added` i18n key
  updated to the renamed variant/key.
- `crates/dtrpg-ui/i18n/en.yaml`, `de.yaml`, `fr.yaml`: `recently_added` key
  renamed to `recently_updated`, values updated to "Recently Updated" /
  translated equivalents.
- Other capabilities that merely list "Recently Added" as one of several
  filter names in passing (`sidebar-nav`, `publisher-filter-title`) aren't
  listed as modified since their requirements don't depend on the specific
  label text or cutoff semantics, but their prose will read stale until a
  later cosmetic pass updates the label mentioned there too.
