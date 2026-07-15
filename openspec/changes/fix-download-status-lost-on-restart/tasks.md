## 1. Reconcile Fix

- [x] 1.1 In `reconcile_catalog` (`crates/dtrpg-ui/src/controllers/library.rs`),
      for a matched id, before replacing `item` with `live_item`, build a
      lookup of the existing item's `files[*].downloaded` by file `id`
- [x] 1.2 After assigning `item = live_item`, apply the looked-up `downloaded`
      flag to each of `item.files` whose id has an entry in the lookup
- [x] 1.3 Call `item.recompute_status()` after the merge so `status` reflects
      the merged `downloaded` flags rather than the live fetch's default

## 2. Tests

- [x] 2.1 Unit test: a matched item with `status: Downloaded` and all files
      `downloaded: true` keeps `status: Downloaded` after reconciling against
      a live item with the same file ids and `downloaded: false`
- [x] 2.2 Unit test: a live file id with no cached counterpart is
      `downloaded: false` after reconcile
- [x] 2.3 Unit test: a partially-downloaded item preserves per-file
      `downloaded` state and keeps `status: Cloud` after reconcile
- [x] 2.4 Confirm existing `reconcile_catalog` tests (is_available true/false
      for matched/unmatched/new items) still pass unmodified

## 3. Persist Downloads to Disk Cache

- [x] 3.1 In `dispatch_download`'s completion handler
      (`crates/dtrpg-ui/src/controllers/library.rs`), after the existing
      `ctrl.update` block that sets `file.downloaded = true` and calls
      `recompute_status()`, add a `save_catalog_cache` call guarded by
      `!cancelled && outcome.is_ok()`
- [x] 3.2 Clone `ctrl.catalog` and run the save on the background executor,
      matching the pattern already used in `start_load_inner`'s full/partial
      fetch completion

## 4. Build and Quality

- [x] 4.1 `cargo check --workspace`
- [x] 4.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 4.3 `cargo test --workspace`

## 5. Manual Verification

- [ ] 5.1 Download an item, quit and relaunch the app, confirm it still shows
      as Downloaded (not Cloud) once the startup live fetch completes
- [ ] 5.2 Download an item, quit and relaunch the app immediately (before the
      7-day cache staleness window would force a live fetch), confirm it
      still shows as Downloaded even when the auto-load policy skips the
      live fetch entirely
