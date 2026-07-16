## 1. Constants and Filter Enum

- [x] 1.1 In `crates/dtrpg-ui/src/data/constants.rs`, remove
      `RECENTLY_ADDED_THRESHOLD` and add
      `RECENTLY_UPDATED_WINDOW_SECS: i64 = 30 * 24 * 60 * 60`
- [x] 1.2 In `crates/dtrpg-ui/src/util/filter.rs`, rename
      `SidebarFilter::RecentlyAdded` to `RecentlyUpdated` (including its
      `PartialEq` arm)

## 2. Filter Predicate

- [x] 2.1 In `crates/dtrpg-ui/src/util/matching.rs`, add
      `item_recently_updated(item: &LibraryItem, now_secs: i64) -> bool`
      that compares `item.date_updated.or(item.date_added)` (falling back to
      `date_added` when there's no update timestamp) against
      `now_secs - RECENTLY_UPDATED_WINDOW_SECS`; returns `false` when both
      are `None`
- [x] 2.2 Update `item_matches_filter`'s `SidebarFilter::RecentlyUpdated` arm
      to call `item_recently_updated(item, now_secs)`, threading a `now_secs:
      i64` parameter through `item_matches_filter`'s signature and its call
      sites

## 3. Section Counts

- [x] 3.1 In `crates/dtrpg-ui/src/data/library.rs`, rename
      `SectionCounts::recently_added` to `recently_updated`
- [x] 3.2 Update `section_counts` to take a `now_secs: i64` parameter and use
      `item_recently_updated` (from `util::matching`) for the
      `recently_updated` count instead of the `added_order` check; update
      `section_counts`'s call sites to pass the current time

## 4. UI Views

- [x] 4.1 In `crates/dtrpg-ui/src/ui/views/toolbar_view.rs`, update the
      `SidebarFilter::RecentlyAdded` match arm (title label lookup) and the
      filter-construction call site to use `RecentlyUpdated`
- [x] 4.2 In `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`, update the nav
      item's filter reference, `counts.recently_added` field access, and
      active-state comparison to use `RecentlyUpdated` / `recently_updated`

## 5. Localization

- [x] 5.1 Rename the `sidebar.recently_added` key to `sidebar.recently_updated`
      in `crates/dtrpg-ui/i18n/en.yaml`, updating its value to "Recently
      Updated"
- [x] 5.2 Apply the same key rename in `de.yaml` and `fr.yaml`, updating
      values to their translated equivalents ("Kürzlich aktualisiert" /
      "Mises à jour récentes")

## 6. Tests

- [x] 6.1 Unit test: an item whose `date_updated` is within the last 30 days
      matches `item_recently_updated`
- [x] 6.2 Unit test: an item whose `date_updated` is `None` but whose
      `date_added` is within the last 30 days matches
      `item_recently_updated`
- [x] 6.3 Unit test: an item whose most recent timestamp is exactly 30 days
      plus one second ago does not match `item_recently_updated`
- [x] 6.4 Unit test: an item with both `date_added` and `date_updated` as
      `None` does not match `item_recently_updated`
- [x] 6.5 Unit test: `item_matches_filter` with `SidebarFilter::RecentlyUpdated`
      delegates correctly for a matching and a non-matching item
- [x] 6.6 Unit test: `section_counts`'s `recently_updated` count reflects only
      items within the 30-day window

## 7. Build and Quality

- [x] 7.1 `cargo check --workspace`
- [x] 7.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 7.3 `cargo test --workspace`
- [x] 7.4 `cargo +nightly fmt --all -- --check`

## 8. Manual Verification

- [ ] 8.1 Confirm the sidebar shows "Recently Updated" (not "Recently Added")
      and its badge count matches the number of items added or updated in
      the last 30 days
- [ ] 8.2 Confirm the toolbar title shows "Recently Updated" when that
      filter is active

## 9. Preference Constants and Predicate

- [x] 9.1 In `crates/dtrpg-ui/src/data/constants.rs`, replace
      `RECENTLY_UPDATED_WINDOW_SECS` with `RECENTLY_UPDATED_WINDOW_MIN_DAYS:
      u32 = 7`, `RECENTLY_UPDATED_WINDOW_MAX_DAYS: u32 = 90`, and
      `RECENTLY_UPDATED_WINDOW_DEFAULT_DAYS: u32 = 30`
- [x] 9.2 In `crates/dtrpg-ui/src/util/matching.rs`, change
      `item_recently_updated` to take `window_days: u32` instead of reading
      the fixed constant, converting to seconds internally
      (`i64::from(window_days) * 86_400`)
- [x] 9.3 Thread `window_days: u32` through `item_matches_filter`'s
      signature and its call sites (mirroring `now_secs`)
- [x] 9.4 Thread `window_days: u32` through `section_counts`'s signature and
      its call sites

## 10. Persisted Preference

- [x] 10.1 In `crates/dtrpg-ui/src/data/storage.rs`, add
      `recently_updated_window_days: u32` to `StorageConfigFile` and
      `StorageConfig` (default via `RECENTLY_UPDATED_WINDOW_DEFAULT_DAYS`),
      following `max_concurrent_downloads`'s exact field/serde pattern
- [x] 10.2 Add `StorageConfig::recently_updated_window_days()` getter and
      `set_recently_updated_window_days(&mut self, days: u32)` setter that
      clamps to `[RECENTLY_UPDATED_WINDOW_MIN_DAYS,
      RECENTLY_UPDATED_WINDOW_MAX_DAYS]` before storing and persisting

## 11. Settings Controller and Wiring

- [x] 11.1 In `crates/dtrpg-ui/src/controllers/settings.rs`, add
      `recently_updated_window_days` to `SettingsSnapshot` and a
      `set_recently_updated_window_days(&mut self, days: u32, cx: &mut
      Context<Self>)` method that updates `self.storage` and emits
      `SettingsChanged`, mirroring `set_max_concurrent_downloads`
- [x] 11.2 In `crates/dtrpg-ui/src/controllers/library.rs`, add a
      `recently_updated_window_days: u32` field (loaded from
      `StorageConfig::load()` at construction) and a
      `set_recently_updated_window_days(&mut self, days: u32, cx: &mut
      Context<Self>)` method that updates the field and invalidates the
      cached filtered items/section counts, mirroring
      `set_max_concurrent_downloads`
- [x] 11.3 In `crates/dtrpg-ui/src/ui/views/root_view.rs`, add a
      `SettingsChanged` subscription that reads
      `settings.snapshot().recently_updated_window_days` and calls
      `controller.set_recently_updated_window_days`, mirroring the existing
      `max_concurrent_downloads` subscription

## 12. Settings UI

- [x] 12.1 In `crates/dtrpg-ui/src/ui/views/settings_advanced_view.rs`, add
      an editable "Recently Updated window" row (7-90 days) at the top of
      the Advanced section, using `gpui_component::input::NumberInput`
      bound to an `InputState` with `min`/`max` set, wired to
      `SettingsController::set_recently_updated_window_days` via an
      `InputEvent::Change` subscription created in `root_view.rs`
- [x] 12.2 Add `settings.recently_updated_window_title` and
      `settings.recently_updated_window_note` i18n keys to `en.yaml`,
      `de.yaml`, `fr.yaml` (the decrement/increment tooltip keys from the
      earlier hand-rolled stepper are no longer needed — `NumberInput`
      supplies its own step buttons)

## 13. Tests

- [x] 13.1 Unit test: `item_recently_updated` respects a non-default
      `window_days` (e.g. an item 10 days old matches a 7-day window's
      complement but not a narrower one)
- [x] 13.2 Unit test: `StorageConfig::set_recently_updated_window_days`
      clamps values below 7 and above 90
- [x] 13.3 Unit test: `section_counts` reflects a non-default `window_days`

## 14. Build and Quality (re-run)

- [x] 14.1 `cargo check --workspace`
- [x] 14.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 14.3 `cargo test --workspace`
- [x] 14.4 `cargo +nightly fmt --all -- --check`

## 15. Manual Verification (additional)

- [ ] 15.1 Confirm the "Recently Updated window" field at the top of the
      Advanced settings page is directly editable (typing a value) and its
      +/- buttons work, that it won't go below 7 or above 90, and that
      changing it immediately updates the sidebar's "Recently Updated"
      badge count without restarting the app
