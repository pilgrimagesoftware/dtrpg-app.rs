## 1. File Presence Module

- [ ] 1.1 Create `crates/dtrpg-ui/src/util/file_presence.rs` with
      `resolved_file_path(storage: &StorageConfig, item: &LibraryItem, file: &LibraryItemFile) -> PathBuf`,
      matching `dispatch_download`'s existing
      `path_for_publisher(&item.publisher).join(file.name.as_ref())`
      computation
- [ ] 1.2 Add `verify_item_downloads(item: &mut LibraryItem, storage: &StorageConfig) -> bool`
      that sets each file's `downloaded` to `resolved_file_path(...).exists()`,
      calls `item.recompute_status()`, and returns whether any file's flag
      or the item's status actually changed
- [ ] 1.3 Register the new module in `crates/dtrpg-ui/src/util/mod.rs` (or
      equivalent module declaration file)
- [ ] 1.4 In `dispatch_download` (`crates/dtrpg-ui/src/controllers/library.rs`),
      replace the inline destination-path computation with a call to
      `resolved_file_path`

## 2. Catalog-Wide Verification on Load

- [ ] 2.1 Add an async helper (e.g. `verify_catalog_downloads`) that, given
      the controller's weak entity and async context: clones `ctrl.catalog`,
      loads `StorageConfig::load()` once, runs `verify_item_downloads` over
      every item on the background executor, and — if anything changed —
      applies the updated catalog back via `ctrl.update`, recomputing
      `section_counts` and calling `invalidate_cache()`
- [ ] 2.2 Call this helper at the auto-load-skip point in `start_load_inner`
      (~`library.rs:847-868`, before the final `return;`)
- [ ] 2.3 Call this helper at the partial-fetch-success point in
      `start_load_inner` (~`library.rs:900-928`, before the final `return;`)
- [ ] 2.4 Call this helper at the full-fetch-success point in
      `start_load_inner` (inside the `Ok(())` arm of `match fetch.await`,
      after `set_catalog` and `save_catalog_cache`)

## 3. On-Demand Verification on Selection

- [ ] 3.1 In `select_item` (`crates/dtrpg-ui/src/controllers/library.rs`),
      spawn a background task (parallel to the existing
      `maybe_check_item` call) that loads `StorageConfig::load()`, calls
      `verify_item_downloads` on the selected item, and — if changed —
      applies the result back via `ctrl.update`, recomputing
      `section_counts` and calling `invalidate_cache()`

## 4. Tests

- [ ] 4.1 Unit test: `resolved_file_path` returns the expected path for a
      known publisher/file name pair (matching `dispatch_download`'s
      existing behavior)
- [ ] 4.2 Unit test: `verify_item_downloads` marks a file `downloaded: true`
      when it exists on disk but the flag was `false` (use a tempdir)
- [ ] 4.3 Unit test: `verify_item_downloads` marks a file `downloaded: false`
      when it does not exist on disk but the flag was `true`
- [ ] 4.4 Unit test: `verify_item_downloads` recomputes `status` correctly
      for a multi-file item with mixed presence
- [ ] 4.5 Unit test: `verify_item_downloads` returns `false` when nothing
      changed (all flags already matched disk state)

## 5. Build and Quality

- [ ] 5.1 `cargo check --workspace`
- [ ] 5.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 5.3 `cargo test --workspace`
- [ ] 5.4 `cargo +nightly fmt --all -- --check`

## 6. Manual Verification

- [ ] 6.1 Download an item, delete its file outside the app, restart or
      trigger a reload, confirm it no longer shows as Downloaded
- [ ] 6.2 Place a file at an item's expected path manually (without using
      the app's download flow), select that item, confirm it becomes
      marked Downloaded without a full catalog reload
- [ ] 6.3 Confirm a fresh-cache/skip-fetch restart still catches an
      externally deleted file (task 6.1's scenario, but via the skip-fetch
      path specifically — restart quickly enough that the cache is still
      within its freshness window)
