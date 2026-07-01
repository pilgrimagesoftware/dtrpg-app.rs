## Why

Several interactions in the app are broken or incomplete: collections don't appear on startup, UI state isn't persisted, reload creates duplicate items, and multiple controls are non-functional or missing. These are blocking day-to-day usability.

## What Changes

- Fix collections load: the service is unauthenticated at startup; collections must load only after auth completes via `replace_service`, not from `start_load`
- Fix the "Remove" button on file opener items in the detail view (currently a no-op)
- Fix Account view and Avatar menu: user display name, email, and avatar are not populated after sign-in
- Add "First" and "Last" buttons to pagination controls alongside existing Prev/Next
- Remove the resizable splitter between the catalog and the right edge; the catalog pane fills from the sidebar to the window edge (no detail-panel resize handle when detail is hidden)
- Fix the detail view close button visibility (currently not rendering)
- Persist collapsed/expanded state for Collections and Publishers sidebar sections across restarts
- Fix catalog reload via menu: `reload_catalog` appends to existing items instead of replacing them, doubling the catalog

## Capabilities

### New Capabilities

- `pagination-first-last`: First and Last page buttons in the pagination bar
- `sidebar-section-collapse-state`: Persist collapsed/expanded state for Collections and Publishers across restarts

### Modified Capabilities

- `rust-main-window-library-layout`: Remove right-side resizable splitter; catalog fills to window edge

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs` - reload clear, collections load timing
- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs` - close button fix, file opener remove button
- `crates/dtrpg-ui/src/ui/views/root_view.rs` - remove detail panel resize handle
- `crates/dtrpg-ui/src/ui/views/catalog_view.rs` - pagination first/last
- `crates/dtrpg-ui/src/ui/views/sidebar_view.rs` - persist section collapse state
- `crates/dtrpg-ui/src/controllers/settings.rs` or equivalent - account/avatar info after sign-in
- `crates/dtrpg-ui/src/data/ui_prefs.rs` - new sidebar section collapse fields
