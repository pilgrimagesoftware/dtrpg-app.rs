## Why

A triage pass against the app's informal bug list turned up four unrelated but
independently verifiable defects: the list view's column headers didn't match the visual
alignment of their own cells, a cache-clear could race with an in-flight catalog load and
let stale data win, the detail view showed a misleading "0" or blank value for fields the
API doesn't always populate, and a spurious "auth issue"-looking alert appeared in the
alert history on every cold start. None of these needed new capabilities — each is a
correctness fix to already-specified behavior.

## What Changes

- List view column header text is now vertically centered in its cell, matching every
  data row (`gpui-component`'s default header renderer left it top-aligned).
- `LibraryController` now guards catalog loads with a generation counter so a load already
  in flight when "Clear Cache" (or any reload) fires cannot overwrite the fresher one;
  queued-but-unstarted thumbnail fetches are dropped at the same time.
- The detail view's "Pages" row is omitted entirely when the API reports no page count
  (previously showed a misleading "0"); the "System" field falls back to an em dash when
  the API doesn't report a game line, instead of rendering blank.
- A catalog load that fails only because auth hasn't completed yet (expected at startup,
  before sign-in finishes) no longer raises a user-facing alert — it already logged at
  `debug!` with a comment saying as much, but still called the alert-raising path.

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

- `rust-main-window-library-layout`: list view header cells are vertically centered like
  data cells; detail view field display rules for pages and system fields
- `catalog-auto-load-policy`: catalog loads are generation-guarded against being clobbered
  by a superseded in-flight load; expected pre-auth session errors do not raise an alert

## Impact

- `dtrpg-ui/src/ui/views/catalog_view.rs` — `CatalogListDelegate::render_th` override
- `dtrpg-ui/src/controllers/library.rs` — `load_generation` counter, guarded catalog
  update closures, `clear_and_reload` drops queued thumbnail fetches, session-error path
  calls `activity.complete` instead of `activity.error`
- `dtrpg-ui/src/ui/views/detail_panel_view.rs` — conditional "Pages" row, `value_or_dash`
  helper for the "System" row (with unit tests)
- No new dependencies, no data model changes
