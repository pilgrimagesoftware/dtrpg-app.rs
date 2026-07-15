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

## 3. Build and Quality

- [x] 3.1 `cargo check --workspace`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Download an item, quit and relaunch the app, confirm it still shows
      as Downloaded (not Cloud) once the startup live fetch completes
