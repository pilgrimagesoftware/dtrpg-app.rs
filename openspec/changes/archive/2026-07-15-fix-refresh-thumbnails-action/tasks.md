## 1. Verify Current Behavior

- [x] 1.1 Manually confirm `refresh_all_thumbnails` re-fetches and overwrites cached
      images end-to-end (network call succeeds, `CoverCache::insert` overwrites, catalog
      re-renders)
- [x] 1.2 Confirm the no-op early-return path (no items with `cover_url`) and identify
      where user feedback is currently missing

## 2. Start/Completion Feedback

- [x] 2.1 Post an activity entry or toast when `refresh_all_thumbnails` begins, with a
      label distinguishing it from the automatic per-page thumbnail load
      ("Refreshing N thumbnails\u{2026}")
- [x] 2.2 Post a completion toast/notice summarizing success/failure counts once the
      refresh queue drains
- [x] 2.3 Post a "No thumbnails to refresh" notice on the early-return no-op path

## 3. Build and Quality

- [x] 3.1 `cargo check --workspace`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [x] 4.1 Trigger "Refresh Thumbnails" from the Catalog menu and confirm a start
      notification appears
- [x] 4.2 Confirm a completion notification appears once refresh finishes
- [x] 4.3 Trigger it on a catalog with no cover URLs and confirm the "nothing to refresh"
      notice appears instead of silent no-op
