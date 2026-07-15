## Why

The Catalog menu's three items (Add Collection, Reload, Refresh Thumbnails) have no keyboard
shortcuts today — every other frequently-used menu (App, Window) already exposes `cmd-*`
bindings for its actions via `cx.bind_keys`, but `AddCollection`, `ReloadCatalog`, and
`RefreshThumbnails` were never registered. Users have to reach for the mouse and open the
menu bar for actions they'd otherwise reach for constantly while curating a library.

## What Changes

- Bind `cmd-shift-n` to "Add Collection" (matches the Finder "New Folder" convention; no
  existing `cmd-n` binding to collide with).
- Bind `cmd-r` to "Reload" (standard "reload/refresh" convention).
- Bind `cmd-shift-r` to "Refresh Thumbnails" (mirrors the browser "hard reload" convention —
  a forced refresh that bypasses the normal cache, which is exactly what this action does).
- No new menu items, no behavior changes to the actions themselves — purely wiring existing
  `gpui::Action` types to `KeyBinding`s, the same mechanism already used for `Quit`,
  `ShowSettings`, `Minimize`, etc.

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

- `catalog-menu`: Add Collection and Reload gain keyboard shortcut requirements; Refresh
  Thumbnails (previously undocumented in this spec) is added along with its shortcut.

## Impact

- `crates/dtrpg-ui/src/ui/app/mod.rs`: `setup()`'s `cx.bind_keys([...])` call gains three
  `KeyBinding::new(...)` entries for `AddCollection`, `ReloadCatalog`, `RefreshThumbnails`.
- No changes to `ui/actions.rs` (action types already exist) or to the action handlers in
  `ui/views/root_view.rs` (already wired via `.on_action`) — keybindings dispatch to the
  same handlers regardless of trigger source (menu click vs. shortcut).
