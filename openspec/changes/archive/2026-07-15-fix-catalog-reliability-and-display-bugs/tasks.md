## 1. List view header vertical alignment

- [x] 1.1 Root-cause: `gpui-component`'s default `TableDelegate::render_th` renders a
      non-flex `div`, unlike every `render_td` cell which uses `.h_full().flex().items_center()`
- [x] 1.2 Add a `render_th` override to `CatalogListDelegate` (`catalog_view.rs`) matching
      the `render_td` cell style

## 2. Catalog-load race on cache clear / reload

- [x] 2.1 Add `load_generation: u64` field to `LibraryController`, bumped at the start of
      `start_load_inner`
- [x] 2.2 Capture the generation in `start_load_inner`'s background task; guard the
      collections pre-populate, catalog cache pre-populate, fresh-cache-skip, and final
      `set_catalog` update closures against a superseded generation
- [x] 2.3 `clear_and_reload` drops queued-but-unstarted thumbnail fetches from
      `thumbnail_queue` and clears their `CoverCache` in-flight markers

## 3. Detail view field display

- [x] 3.1 Omit the "Pages" row in `render_metadata_table` when `item.pages == 0`, matching
      the existing conditional pattern used for `date_added`
- [x] 3.2 Add a `value_or_dash` helper (with unit tests) that falls back to an em dash for
      empty/whitespace-only values; apply it to the "System" (`field_system`) row

## 4. Spurious session-error alert on cold start

- [x] 4.1 `LibraryController::start_load`'s error path now calls `activity.complete(...)`
      instead of `activity.error(...)` when the failure kind is
      `LibraryServiceErrorKind::Session` (expected pre-auth), so no alert is raised;
      non-Session errors are unaffected

## 5. Verify

- [x] 5.1 Run `cargo test --all-features --workspace`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check`; all pass
- [x] 5.2 Manually launch the app and confirm the list view's column headers are
      vertically centered
- [x] 5.3 Manually trigger "Clear Cache" while a catalog load is in flight and confirm the
      fresh reload's data is what's displayed, not a stale one
- [x] 5.4 Manually confirm an item with no page count omits the "Pages" row, and an item
      with no game line shows an em dash for "System"
- [x] 5.5 Manually confirm no "auth issue"-looking alert appears in the alert history on a
      cold start
