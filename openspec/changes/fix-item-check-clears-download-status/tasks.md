## 1. Preserve Downloaded State Through a Check

- [ ] 1.1 In `apply_check_result` (`crates/dtrpg-ui/src/controllers/library.rs`),
      on the `Ok(fresh)` branch, before `*item = fresh;`, build a lookup of
      the existing item's `files[*].downloaded` by file `id`
- [ ] 1.2 After `*item = fresh;`, apply the looked-up `downloaded` flag to
      each of `item.files` whose id has an entry in the lookup
- [ ] 1.3 Call `item.recompute_status()` after the merge, alongside the
      existing identity/membership field restoration and
      `item.is_available = true;`

## 2. Keep Section Counts Synchronized

- [ ] 2.1 In `start_item_check`'s completion handler, after
      `ctrl.invalidate_cache()`, add `ctrl.section_counts =
      section_counts(&ctrl.catalog);`

## 3. Tests

- [ ] 3.1 Unit test: an item with `status: Downloaded` and all files
      `downloaded: true` keeps `status: Downloaded` after
      `apply_check_result` with a fresh response carrying the same file ids
      and `downloaded: false`
- [ ] 3.2 Unit test: a fresh file id with no existing counterpart is
      `downloaded: false` after `apply_check_result`
- [ ] 3.3 Unit test: a partially-downloaded item preserves per-file
      `downloaded` state and keeps `status: Cloud` after
      `apply_check_result`
- [ ] 3.4 Confirm existing `apply_check_result` tests (identity/membership
      preservation, `is_available`, not-found, transient error) still pass
      unmodified
- [ ] 3.5 Add or extend a test around `start_item_check`'s completion path
      confirming `section_counts` is recomputed after a check (if the
      existing test harness supports driving `start_item_check`'s async
      completion; otherwise cover the section-counts recompute at the unit
      level alongside 3.1-3.3 and note the integration gap)

## 4. Build and Quality

- [ ] 4.1 `cargo check --workspace`
- [ ] 4.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 4.3 `cargo test --workspace`
- [ ] 4.4 `cargo +nightly fmt --all -- --check`

## 5. Manual Verification

- [ ] 5.1 Download an item, open its details to trigger an on-demand check
      (or wait for the periodic batch), confirm it still shows as
      Downloaded and the "On This Device" count still includes it
- [ ] 5.2 Confirm the "On This Device" section no longer shows a stuck
      loading indicator once its filtered item count is genuinely non-zero
