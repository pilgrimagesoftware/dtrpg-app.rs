## Why

"Remove Download" (in the catalog context menu, item popover, and detail panel) currently deletes a downloaded item's local content immediately on click, with no confirmation step. A single misclick discards a file the user may have to re-download, with no undo. Every other destructive action in the app (clearing the cache, removing a collection) asks first; this one doesn't.

## What Changes

- Show a confirmation dialog before `remove_download` runs, for all three entry points (catalog context menu, item popover, detail panel).
- The dialog names the item and states that its local content will be removed; confirming proceeds, cancelling leaves the item untouched.
- No change to `remove_download`'s own behavior once confirmed — it still just reverts status to `Cloud` (no real file deletion exists yet, per `download-queue-concurrency-control`'s scope).

## Capabilities

### New Capabilities

- `download-removal-confirmation`: a confirmation step required before a downloaded item's local content is removed, covering all UI entry points that trigger removal.

### Modified Capabilities

<!-- none -->

## Impact

- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: the two `action_remove_download` context-menu handlers gain a confirmation dialog before calling `remove_download`.
- `crates/dtrpg-ui/src/ui/views/item_popover_view.rs`: the item popover's toggle button gains a confirmation dialog on the "downloaded" branch.
- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: the detail panel's download button gains a confirmation dialog on the "downloaded" branch.
- Likely reuses `gpui_component`'s existing modal/dialog primitive (survey during design) rather than a new custom component.
