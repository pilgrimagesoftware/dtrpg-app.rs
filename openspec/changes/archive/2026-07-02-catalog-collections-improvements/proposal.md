## Why

The catalog UI and collections feature have several gaps: collections fail to load, the sidebar lacks item counts, auto-load logic is naive, there are no catalog or window menu entries, and the catalog title doesn't reflect filtered vs. total state. These compound to make the app feel incomplete and unreliable.

## What Changes

- Fix collections failing to load from the API
- Display item counts next to Collections and Publishers section titles in the sidebar
- Update catalog auto-load logic: trigger a load when the local cache is empty, when the last load was more than 1 week ago, or when the local cache count differs from the remote count
- Add a "Catalog" menu with "Add Collection" and "Reload" actions
- Add a "Window" menu with "Show Activity" and "Show Alert History" actions
- Show a count in the catalog title: total item count when unfiltered; "filtered / total" when a filter is active
- Add a context menu on collection sidebar items with "Reload" and "Delete" actions

## Capabilities

### New Capabilities

- `catalog-menu`: Top-level "Catalog" menu with "Add Collection" and "Reload" entries
- `window-menu`: Top-level "Window" menu with "Show Activity" and "Show Alert History" entries
- `catalog-title-count`: Count display in the catalog view title bar (total when unfiltered, filtered/total when filtered)
- `sidebar-section-counts`: Item counts rendered next to Collections and Publishers section headers in the sidebar
- `collection-context-menu`: Right-click context menu on collection sidebar entries with Reload and Delete actions
- `catalog-auto-load-policy`: Updated policy for when the catalog triggers an automatic load (cache empty, stale by age, or count mismatch)

### Modified Capabilities

- `rust-main-window-library-layout`: Sidebar section headers and catalog title area gain count annotations; menu bar gains Catalog and Window menus

## Impact

- `src/ui/sidebar/` - section header rendering, collections loading fix, context menu
- `src/ui/catalog/` - title count display, auto-load trigger logic
- `src/ui/menu.rs` (or equivalent) - new Catalog and Window menu definitions
- `src/ui/activity/` and `src/ui/alerts/` - window show/hide actions wired to Window menu
- No API contract changes; uses existing SDK calls for collection reload/delete
