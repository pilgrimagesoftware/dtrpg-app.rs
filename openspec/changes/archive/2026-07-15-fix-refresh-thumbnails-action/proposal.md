## Why

The "Refresh Thumbnails" catalog menu item dispatches `RefreshThumbnails` to
`LibraryController::refresh_all_thumbnails`, which does correctly re-queue every item with
a `cover_url` for re-fetch. In practice the action appears to do nothing: it silently
re-downloads images that are visually identical to what's already cached, surfaces no
completion feedback beyond the easy-to-miss status bar activity glyph, and gives no
indication of failure if the re-fetch errors. From the user's perspective, clicking the
menu item produces no observable change.

## What Changes

- `refresh_all_thumbnails` triggers a toast notification (or activity-panel-visible entry
  with a clear label, e.g. "Refreshing N thumbnails\u{2026}") when it starts, and a
  completion toast ("Refreshed N thumbnails" / "Refresh failed for N items") when the
  queue drains, so the action has an observable start and end.
- If the catalog has no items with a `cover_url` (the current early-return case), the
  action surfaces a brief "No thumbnails to refresh" notice instead of silently returning.
- Verify end-to-end that `refresh_all_thumbnails` actually overwrites cached bytes (it
  does, via `CoverCache::insert`) and that the catalog re-renders with the newly-fetched
  image once each fetch completes.

## Capabilities

### New Capabilities

- `refresh-thumbnails-feedback`: The "Refresh Thumbnails" action surfaces start,
  completion, and no-op feedback (toast or activity entry) so the user can observe that it
  ran.

### Modified Capabilities

_(none)_

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: `refresh_all_thumbnails` posts an activity
  entry / toast on start and completion; early-return path posts a "nothing to refresh"
  notice instead of returning silently.
- `crates/dtrpg-ui/src/ui/views/root_view.rs` or toast infrastructure from
  `adopt-gpui-component-primitives`'s `Notification` work, if landed, is the delivery
  mechanism for the toasts.
