## Why

The toolbar has two separate controls for related browsing configuration — a sort dropdown and a standalone "Group" toggle button — which is more cluttered than it needs to be. Consolidating the group toggle into the sort menu reduces control count and makes it clearer that sort and grouping are related browsing options. The sort button also currently has no visual indicator that it opens a menu, which is inconsistent with how interactive controls appear elsewhere.

## What Changes

- Remove the standalone "Group" toggle button from the toolbar
- Add a separator and a "Group by Publisher" toggleable item to the sort dropdown menu, below the existing sort items
- Add `.dropdown_caret(true)` to the sort button so it displays a chevron indicator

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `rust-main-window-library-layout`: Sort control adds a dropdown caret indicator; "Group by Publisher" moves from standalone button to toggleable item inside the sort menu with a separator above it

## Impact

- `toolbar_view.rs`: `render_sort_selector` gains `grouped` and `entity` for grouped state; `render_group_toggle` function and its call site in `render_toolbar` are removed; sort button gains `.dropdown_caret(true)`; the sort menu gains a `PopupMenuItem::separator()` and a checkable "Group by Publisher" item
- No controller or data model changes (grouping is already a `set_grouped` call on the controller)
