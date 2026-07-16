## 1. File Presence Module

- [x] 1.1 Create `crates/dtrpg-ui/src/util/file_presence.rs` with
      `resolved_file_path(storage: &StorageConfig, item: &LibraryItem, file: &LibraryItemFile) -> PathBuf`,
      matching `dispatch_download`'s existing
      `path_for_publisher(&item.publisher).join(file.name.as_ref())`
      computation
- [x] 1.2 Add `verify_item_downloads(item: &mut LibraryItem, storage: &StorageConfig) -> bool`
      that sets each file's `downloaded` to `resolved_file_path(...).exists()`,
      calls `item.recompute_status()`, and returns whether any file's flag
      or the item's status actually changed
- [x] 1.3 Register the new module in `crates/dtrpg-ui/src/util/mod.rs` (or
      equivalent module declaration file)
- [x] 1.4 In `dispatch_download` (`crates/dtrpg-ui/src/controllers/library.rs`),
      replace the inline destination-path computation with a call to
      `resolved_file_path`

## 2. Catalog-Wide Verification on Load

- [x] 2.1 Add an async helper (e.g. `verify_catalog_downloads`) that, given
      the controller's weak entity and async context: clones `ctrl.catalog`,
      loads `StorageConfig::load()` once, runs `verify_item_downloads` over
      every item on the background executor, and — if anything changed —
      applies the updated catalog back via `ctrl.update`, recomputing
      `section_counts` and calling `invalidate_cache()`
- [x] 2.2 Call this helper at the auto-load-skip point in `start_load_inner`
      (~`library.rs:847-868`, before the final `return;`)
- [x] 2.3 Call this helper at the partial-fetch-success point in
      `start_load_inner` (~`library.rs:900-928`, before the final `return;`)
- [x] 2.4 Call this helper at the full-fetch-success point in
      `start_load_inner` (inside the `Ok(())` arm of `match fetch.await`,
      after `set_catalog` and `save_catalog_cache`)

## 3. On-Demand Verification on Selection

- [x] 3.1 In `select_item` (`crates/dtrpg-ui/src/controllers/library.rs`),
      spawn a background task (parallel to the existing
      `maybe_check_item` call) that loads `StorageConfig::load()`, calls
      `verify_item_downloads` on the selected item, and — if changed —
      applies the result back via `ctrl.update`, recomputing
      `section_counts` and calling `invalidate_cache()`

## 4. Tests

- [x] 4.1 Unit test: `resolved_file_path` returns the expected path for a
      known publisher/file name pair (matching `dispatch_download`'s
      existing behavior)
- [x] 4.2 Unit test: `verify_item_downloads` marks a file `downloaded: true`
      when it exists on disk but the flag was `false` (use a tempdir)
- [x] 4.3 Unit test: `verify_item_downloads` marks a file `downloaded: false`
      when it does not exist on disk but the flag was `true`
- [x] 4.4 Unit test: `verify_item_downloads` recomputes `status` correctly
      for a multi-file item with mixed presence
- [x] 4.5 Unit test: `verify_item_downloads` returns `false` when nothing
      changed (all flags already matched disk state)

## 5. Build and Quality

- [x] 5.1 `cargo check --workspace`
- [x] 5.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 5.3 `cargo test --workspace`
- [x] 5.4 `cargo +nightly fmt --all -- --check`

## 7. Pending-Verification UI State

- [x] 7.1 Add `LibraryController::verifying_downloads: HashSet<Arc<str>>`
      (distinct from `checking_items`, the network-bound availability-check
      set) plus `is_verifying_downloads(id)` and
      `verifying_downloads_snapshot()` query methods
- [x] 7.2 `verify_catalog_downloads` marks every catalog item id as verifying
      for the duration of its background pass; `verify_selected_item_download`
      marks just the selected id — both clear their ids once the pass
      completes, regardless of whether anything changed
- [x] 7.3 Add `catalog_view::render_verifying_indicator` (tinted spinner +
      `detail.tooltip_verifying_download` tooltip), visually distinct from
      `render_checking_indicator`'s availability-check spinner
- [x] 7.4 Thread `is_verifying`/`verifying_downloads` through
      `render_status`, `render_list_item_cell`, `render_thumb_row`,
      `render_grid_card`, `render_grid`, and all list/thumbs/grid call sites
      (virtualized and grouped) so the catalog status glyph shows the
      pending indicator instead of Downloaded/Cloud while verifying
- [x] 7.5 Item popover (`item_popover_view::render_item_popover`) and detail
      tab (`detail_panel_view::render_detail_tab_content`,
      `render_status_icon`, per-file item-tier status cell) download
      buttons/labels: disable and show a loading/pending state with the
      `tooltip_verifying_download` tooltip while verifying
- [x] 7.6 Add `detail.tooltip_verifying_download` to `en.yaml`, `de.yaml`,
      `fr.yaml`

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
