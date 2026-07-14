## Why

Several UI surfaces in the app are hand-rolled when gpui-component already ships purpose-built primitives. This creates drift: hand-rolled widgets diverge from theme tokens, don't pick up upstream accessibility or animation fixes, and cost maintenance effort. This change sweeps the remaining applicable components into the app.

Two components from the request are excluded from this change: `Sidebar` is covered by `use-gpui-sidebar-components` and `Settings` is covered by `use-gpui-settings-component`. `List` (the delegate-based searchable picker) has no current use case in this app and is deferred.

## What Changes

- **Alert** (`notification_banner_view.rs`): Replace the hand-rolled yellow warning banner with `Alert::new().variant(AlertVariant::Warning)`. One `Alert` per active `Notice`, with title, message, action button, and dismiss wired to the existing `AuthStateController` methods.
- **Avatar** (`settings_account_view.rs`): Replace `render_avatar_circle` with `Avatar`. The `Avatar` component handles the image-with-fallback-initial pattern the hand-rolled function reproduces.
- **Badge** (`catalog_view.rs`): Wrap the download-status indicator on thumbs/grid cards with `Badge::dot()` or `Badge::count()` to surface download status at a glance.
- **DescriptionList** (`detail_panel_view.rs`): Replace `render_metadata_table` with `DescriptionList`. Each label/value pair becomes a `DescriptionItem`.
- **Notification** (new `toast_notification_view.rs` + `LibraryController`): Display auto-dismissing toast notifications (via `gpui_component::notification`) when a download completes or fails. The notification list lives in the root view, keyed to `ActivityController` state changes.
- **Pagination** (`catalog_view.rs` + `LibraryController`): Add `current_page: usize` and `page_size: usize` fields to `LibraryController`. `visible_items_slice` respects the current page window. The catalog footer shows a `Pagination` component; page navigation and page-size changes go through controller methods.
- **Popover** (`catalog_view.rs` or toolbar): Add a page-size picker `Popover` in the pagination bar. Clicking opens a small menu of size options (10, 25, 50, 100, 200); the selected value updates `page_size` via `LibraryController::set_page_size`.
- **Resizable** (`root_view.rs`): Replace the fixed-width sidebar `div()` container with `h_resizable`. Three panels: sidebar (left, min 180 px), catalog content (flex), detail panel (right, min 240 px, shown only when an item is selected).
- **Spinner** (`catalog_view.rs`): Show a centred `Spinner` when the catalog is in the loading state (before any items arrive from the background fetch) instead of an empty pane.

## Capabilities

### New Capabilities

- `catalog-pagination`: Paginated catalog view with user-configurable page size (10, 25, 50, 100, 200 items per page).
- `catalog-loading-state`: Spinner placeholder shown while the catalog loads for the first time.
- `item-download-badge`: Visual download-status badge on thumbs and grid catalog cards.
- `toast-notifications`: Auto-dismissing toast notifications for download complete and download error events.
- `resizable-layout`: Draggable resize handles between sidebar, catalog, and detail panel.

### Modified Capabilities

- `rust-main-window-library-layout`: Layout changes from fixed-width columns to a resizable panel group.

## Impact

- `dtrpg-ui`: `catalog_view.rs`, `detail_panel_view.rs`, `notification_banner_view.rs`, `settings_account_view.rs`, `root_view.rs` (modified); new `toast_notification_view.rs`.
- `LibraryController`: adds `current_page`, `page_size`, `is_catalog_loading` fields; adds `set_page`, `set_page_size`, `total_pages` methods; updates `visible_items_slice` to be page-aware.
- No new crate dependencies. All components are in the existing `gpui_component` import.
