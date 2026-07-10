## Why

The main window already supports multiple open tabs (the always-open Catalog tab plus
closable expanded detail tabs opened by double-clicking a catalog item), but the only way
to switch between them is clicking a tab in the tab strip. Every native macOS tabbed app
binds `cmd-1` through `cmd-9` (and often `cmd-0` for a fixed "home" tab) to jump directly to
a tab by position — users expect that shortcut family to work here too.

## What Changes

- Bind `cmd-0` to always activate the Catalog tab (the first, non-closable tab), regardless
  of how many detail tabs are open.
- Bind `cmd-1` through `cmd-9` to activate the 1st through 9th open *detail* tab (the
  closable tabs opened by double-clicking a catalog item) — Catalog is never a target of
  `cmd-1` through `cmd-9`, only of `cmd-0`.
- Add a "Select Tab" submenu to the native "Window" menu holding ten items — position `0`
  through `9` (labels reflect each tab's actual title where one is open) — each disabled
  when no tab is open at that position. Nested in a submenu rather than listed flat in the
  Window menu, so the ten fixed slots don't lengthen the Window menu itself.
- The submenu item for the currently active tab is check-marked.
- Menu items and key bindings stay in sync with the live tab strip: opening or closing a
  detail tab, or activating one via the tab strip, immediately updates which numbered slots
  are enabled, what their labels read, and which one is check-marked.
- No changes to tab-opening, tab-closing, or tab-strip click-to-activate behavior.

## Capabilities

### New Capabilities

- `catalog-tab-cmd-number-shortcuts`: `cmd-0`...`cmd-9` keyboard shortcuts and a matching
  Window-menu "Select Tab" submenu that activates an open main-window tab by position,
  disabled when no tab occupies that position.

### Modified Capabilities

- `window-menu`: The Window menu gains a "Select Tab" submenu holding ten items (positions
  `0` through `9`) for jumping to a tab by position, in addition to its existing
  Minimize/Zoom/Show Activity/Show Alert History items.

## Impact

- `crates/dtrpg-ui/src/ui/actions.rs`: ten new unit actions (e.g. `SelectTab0` ...
  `SelectTab9`) via the `actions!(libri, [...])` macro.
- `crates/dtrpg-ui/src/ui/app/mod.rs`: `setup()`'s `cx.bind_keys([...])` gains ten
  `KeyBinding::new("cmd-<n>", SelectTab<n>, None)` entries; `build_menus(...)` gains a
  Window-menu section built from tab state (new parameter needed, since `build_menus`
  currently only takes `ViewMenuState`).
- `crates/dtrpg-ui/src/ui/views/root_view.rs`: ten new `.on_action` handlers dispatching to
  `TabsController::activate`; the existing `cx.subscribe(&tabs, ...)` on `TabsChanged` must
  also trigger a menu rebuild (today it only calls `cx.notify()`), mirroring the existing
  `LibraryChanged` -> `build_menus` rebuild pattern.
- `crates/dtrpg-ui/src/controllers/tabs.rs`: `TabsSnapshot`/`TabsController` are read-only
  from this change's perspective — `open_tabs` (ordered, Catalog first) and `titles` already
  provide everything needed to build the menu and resolve `cmd-<n>` targets. No changes
  expected here unless a snapshot helper (e.g. "tab at position N") is convenient to add.
