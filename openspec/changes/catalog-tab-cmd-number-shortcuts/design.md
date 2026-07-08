## Context

`TabsController` (`crates/dtrpg-ui/src/controllers/tabs.rs`) already owns the ordered list of
open tabs (`open_tabs: Vec<TabTarget>`, Catalog always first and non-closable) and the
currently active tab. `tab_strip_view.rs` renders it and calls `TabsController::activate` on
click. The native menu bar is rebuilt via `build_menus(&ViewMenuState)` in `ui/app/mod.rs`,
called once at startup and re-called from `root_view.rs` whenever catalog state relevant to
menu checkmarks changes (`LibraryChanged` subscription tracks `last_menu_state` to avoid
redundant `cx.set_menus` calls, since replacing the menu bar on macOS can interrupt a menu
that's mid-interaction). `TabsChanged` is already emitted by every `TabsController` mutation
but today only triggers `cx.notify()` in `root_view.rs`, not a menu rebuild.

Key bindings are process-global (`cx.bind_keys`, called once in `setup()`), dispatched to
whichever view's `.on_action::<T>` claims the type — see the existing `ReloadCatalog`,
`RefreshThumbnails`, `AddCollection` bindings, all handled on `LibraryRootView` in
`root_view.rs`.

## Goals / Non-Goals

**Goals:**
- `cmd-0` always activates the Catalog tab.
- `cmd-1` through `cmd-9` activate the 1st through 9th open *detail* tab
  (`open_tabs[1]`..`open_tabs[9]`) — Catalog (`open_tabs[0]`) is only ever a target of
  `cmd-0`, never of `cmd-1`..`cmd-9`.
- A Window-menu item exists for each of `cmd-0`..`cmd-9`, labeled with the tab's title when a
  tab occupies that position, disabled (not hidden) when it doesn't.
- Menu item labels/enabled-state and the effective keyboard targets both stay live as tabs
  open and close — no manual refresh needed.

**Non-Goals:**
- No change to how tabs are opened, closed, or reordered.
- No user-configurable shortcuts.
- No support for more than 9 detail-tab positions (`cmd-9` is the last slot); tabs beyond
  position 9 remain reachable only via the tab strip's overflow "more" menu, matching how
  browsers handle `cmd-9` (commonly "last tab", but here it stays literal position 9 since
  `TabsController` doesn't cap `open_tabs.len()` and pinning "last tab" semantics is out of
  scope).

## Decisions

- **Ten actions, not one parameterized action.** `SelectTab0`..`SelectTab9` as unit actions
  (via `actions!(libri, [...])`) rather than one `SelectTab { index: u8 }` action. `gpui`
  key bindings map one `KeyBinding` to one concrete `Action` type; a single parameterized
  action would need 10 `KeyBinding::new("cmd-<n>", SelectTab { index: n }, None)` entries
  anyway (no shorter to write), but keeping them as distinct unit actions matches every
  other action in `actions.rs` that isn't inherently per-instance (contrast with
  `ReloadCollection { id }`, which is parameterized because the id isn't known until
  runtime and there's no fixed small set of collections to enumerate at compile time).
- **Resolve position -> `TabTarget` in the action handler, not in `TabsController`.** The
  handler reads `tabs.read(cx).snapshot().open_tabs.get(index)` and calls
  `TabsController::activate` with the resolved target (or no-ops if `None`). No new method
  needed on `TabsController` — `activate` already no-ops safely if given a stale/closed
  target, and `open_tabs` already exposes catalog-first order (index `0` is always Catalog,
  so position `n` for `n` in `0..=9` maps directly to `open_tabs[n]` with no offset
  arithmetic needed — `cmd-0` -> `open_tabs[0]` = Catalog, `cmd-1..9` -> `open_tabs[1..=9]` =
  the 1st through 9th detail tab).
- **Window menu, not Catalog menu.** Matches the macOS convention that a tabbed window's
  numbered tab shortcuts live in the Window menu (see Safari, Terminal, Xcode — "Window >
  <tab name>" with `cmd-1..9`). The existing `window-menu` capability already covers
  Minimize/Zoom/Show Activity/Show Alert History; this extends it rather than overloading
  the `catalog-menu` capability (which covers catalog-content actions: Add Collection,
  Reload, Refresh Thumbnails — not window/tab chrome).
- **`build_menus` takes a `TabsSnapshot` parameter.** `build_menus(state: &ViewMenuState)`
  becomes `build_menus(state: &ViewMenuState, tabs: &TabsSnapshot)`. Both call sites
  (`setup()` at startup, `root_view.rs`'s `LibraryChanged` subscription) already have or can
  cheaply obtain a `TabsController` snapshot. `ViewMenuState` itself is left alone rather
  than folding tab data into it, since `ViewMenuState` is specifically "catalog presentation
  state for checkmarks" and tabs are a different concern with different derivation
  (`Copy`-friendly small struct vs. a `Vec`-bearing snapshot).
- **Rebuild the menu on `TabsChanged`, mirroring the existing `LibraryChanged` pattern.**
  `root_view.rs`'s `cx.subscribe(&tabs, ...)` handler currently only calls `cx.notify()`;
  it gains a `cx.set_menus(build_menus(...))` call using the same `TabsSnapshot` the tab
  strip renders from. No `last_menu_state`-style de-duplication is added for tabs — unlike
  `LibraryChanged` (which fires on high-frequency events like per-thumbnail progress),
  `TabsChanged` only fires on actual open/close/activate, so redundant rebuilds aren't a
  concern here.
- **Disabled via `MenuItem::disabled(true)`, not omitted.** `gpui`'s `MenuItem::disabled`
  (confirmed present in the pinned `zed` revision's `app_menu.rs`) keeps all ten items
  visible at fixed positions in the Window menu at all times, so the shortcut list is
  discoverable even when few tabs are open — matches the requirement that "the menu item
  should be disabled" (not "hidden") for unoccupied positions.

## Risks / Trade-offs

- [Risk] Ten near-identical `.on_action` closures in `root_view.rs` add repetition versus a
  single parameterized handler → Mitigation: accepted; matches the file's existing style for
  unit actions (e.g. the four `SortBy*` handlers), and a small local helper (e.g. a
  `select_tab_action(tabs: &Entity<TabsController>, index: usize)` closure factory used at
  each of the ten `.on_action` call sites) keeps the bodies to one line each without a new
  action type.
- [Risk] Rebuilding the whole native menu bar on every tab open/close could interrupt an
  in-progress menu interaction, same class of issue the `LibraryChanged` de-duplication was
  built to avoid → Mitigation: tab open/close is a discrete user action (double-click a
  catalog item, click a tab's close button), not a background/high-frequency event, so a
  rebuild coinciding with an open native menu is unlikely; revisit with the same
  `last_menu_state`-style guard if it proves disruptive in manual testing.
- [Risk] `cmd-0`/`cmd-1..9` could collide with a future feature's shortcut → Mitigation: none
  needed now; `cmd-0..9` are otherwise unused today (verified against the current
  `cx.bind_keys([...])` list in `setup()`).
