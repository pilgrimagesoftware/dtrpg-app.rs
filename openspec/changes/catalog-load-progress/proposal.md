## Why

The catalog load currently shows a spinner with no indication of how many items remain. Users with large libraries see indeterminate progress for an extended period and have no way to gauge how long the load will take.

## What Changes

- The SDK client fetches a count of total library items before loading pages, then delivers pages via the existing paginated channel.
- The `ActivityController` gains a progress-bearing entry type (total + completed item count) alongside the existing label-only entries.
- The activity panel renders a progress bar for entries that carry total/completed counts.
- `LibraryController::start_load` seeds the activity entry with the total count and increments it after each page arrives.

## Capabilities

### New Capabilities

- `catalog-load-progress`: SDK-informed progress tracking during paginated catalog load, surfaced as a progress bar in the activity panel.

### Modified Capabilities

- `rust-library-ui-implementation`: The catalog load flow now includes a pre-fetch count step and per-page progress updates. The activity panel gains a progress bar UI element for count-bearing entries.

## Impact

- `dtrpg-core/src/services/sdk.rs`: Add `item_count` call before paged fetch; thread total through to the progress update.
- `dtrpg-ui/src/controllers/activity.rs`: Add `total` / `completed` fields to activity entries; add `set_progress` method.
- `dtrpg-ui/src/ui/views/activity_view.rs` (or equivalent): Render a progress bar when `total` is set.
- `dtrpg-ui/src/controllers/library.rs`: Seed entry with total; increment after each page.
- `dtrpg-ui/src/services/mod.rs`: Extend `LibraryService` trait with `item_count() -> Result<usize, LibraryServiceError>` (default returns 0 meaning unknown).
