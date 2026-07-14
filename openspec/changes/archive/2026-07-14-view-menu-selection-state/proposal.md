## Why

The application menu bar's "View" menu offers catalog presentation (List/Thumbs/Grid),
sort (field + direction), grouping, and "Find in Library" — but none of these were ever
documented as an OpenSpec capability, and the menu items never reflected the current
selection. A user opening the View menu could not tell which presentation mode or sort
was currently active, and toggling the Group option gave no visual confirmation from the
menu itself.

## What Changes

- Document the existing View menu structure (Presentation submenu, Sort submenu, Group
  toggle, Find in Library) as a first-class `view-menu` capability, since it was
  implemented previously without a corresponding change.
- Add checkmarks to the Presentation and Sort submenu items and the Group toggle,
  reflecting the catalog's current state. `gpui::MenuItem::checked(bool)` already
  supports this; nothing previously called it.
- Rebuild the menu bar (`cx.set_menus`) whenever the catalog's presentation, sort, or
  grouping changes, since GPUI's `set_menus` replaces the whole bar rather than patching
  individual items.

## Capabilities

### New Capabilities

- `view-menu`: The "View" menu's Presentation and Sort submenus and Group toggle SHALL
  show a checkmark next to the item matching the catalog's current state.

### Modified Capabilities

<!-- none -->

## Impact

- `dtrpg-ui/src/ui/app/mod.rs` — extracted the inline `cx.set_menus([...])` call in
  `setup` into a reusable `build_menus(&ViewMenuState) -> Vec<Menu>`; added the
  `ViewMenuState` struct (presentation, sort, sort_direction, grouped)
- `dtrpg-ui/src/ui/views/root_view.rs` — the existing `LibraryChanged` subscription now
  also rebuilds and re-applies the menu bar with the controller's current state
- No new dependencies, no data model changes
