## Why

"Delete" in a collection's sidebar context menu calls `delete_collection`, which deletes the product list on the server — a real, irreversible remote action, not just local UI state. It fires immediately on click with no confirmation, so a single misclick permanently destroys a collection and its membership list.

## What Changes

- Show a confirmation dialog naming the collection before `delete_collection` runs, when "Delete" is selected from a collection's sidebar context menu.
- Confirming proceeds exactly as today (server delete, activity panel entry, sidebar/filter update); cancelling leaves the collection untouched and makes no request.

## Capabilities

### New Capabilities

- `collection-delete-confirmation`: a confirmation step required before a collection is deleted from the sidebar context menu.

### Modified Capabilities

<!-- none -->

## Impact

- `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`: `render_collection_row`'s context menu "Delete" handler (~line 280) gains a confirmation dialog before calling `ctrl.delete_collection(col_id, cx)`, reusing the `window.open_alert_dialog` + `.confirm()` pattern already used for file-opener removal in `settings_file_openers_view.rs`.
- No change to `LibraryController::delete_collection` itself, or to the collections service/API call it makes.
