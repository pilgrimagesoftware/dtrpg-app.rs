## Context

`ui/app/mod.rs`'s `setup()` registers global key bindings once via `cx.bind_keys([...])`,
currently covering only App-level (`Quit`, `ShowSettings`, `HideApplication`, `HideOthers`)
and Window-level (`Minimize`, `ToggleFullscreen`) actions. Catalog menu actions
(`AddCollection`, `ReloadCatalog`, `RefreshThumbnails`) are `gpui::Action` types already
dispatched via `.on_action` handlers on `LibraryRootView` (`ui/views/root_view.rs`) — the
handlers fire identically whether triggered by a menu click or a bound key, since GPUI
routes actions to whichever handler in the focused view tree claims the type. No handler
changes are needed; this change only adds the missing `KeyBinding` registrations.

On macOS, GPUI's native menu bar automatically renders a bound action's shortcut next to
its `MenuItem::action` label (already observable for `Quit` → `cmd-q` in the App menu) —
no separate menu-item-level shortcut field to set.

## Goals / Non-Goals

**Goals:**
- Bind a keyboard shortcut to each of the three Catalog menu actions.
- Pick shortcuts that don't collide with existing bindings and that follow recognizable
  OS/app conventions so they're easy to remember.
- Keep the Catalog menu's displayed shortcuts in sync automatically (via GPUI's native
  menu integration) rather than hand-maintaining shortcut text in menu labels.

**Non-Goals:**
- No new menu items, actions, or behavior changes to what Add Collection / Reload /
  Refresh Thumbnails do.
- No user-configurable keybindings — these are fixed, matching every other binding in
  the app today.
- Not extending shortcuts to the Collection context menu's parameterized actions
  (`ReloadCollection`, `DeleteCollection`) — those carry a per-collection `id` field and
  have no single static target to bind a global shortcut to.

## Decisions

- **`cmd-shift-n` → Add Collection.** Mirrors Finder's "New Folder" (`cmd-shift-n`)
  convention — a collection is conceptually the closest analog to a folder in this app.
  Plain `cmd-n` was considered but rejected: it's conventionally "new window/document" in
  macOS apps, which doesn't match "add a collection", and reserving it avoids a future
  collision if a "new window" action is ever added.
- **`cmd-r` → Reload.** Standard "reload/refresh" shortcut (browsers, many macOS apps).
  Free in the current binding set.
- **`cmd-shift-r` → Refresh Thumbnails.** Mirrors the browser "hard reload" convention
  (`cmd-shift-r` = reload bypassing cache) — Refresh Thumbnails is exactly that: a forced
  re-fetch that bypasses the normal thumbnail cache (see `refresh_all_thumbnails` in
  `controllers/library.rs`). Grouping it under the same modifier family as Reload also
  visually/mnemonically pairs the two "refresh" actions in the menu.
- **Global bindings via `cx.bind_keys`, not per-view.** Matches the existing pattern for
  every other bound action in `setup()`; Catalog actions are already handled at the
  `LibraryRootView` level regardless of trigger source, so no new dispatch plumbing is
  needed — this is purely additive registration.

## Risks / Trade-offs

- [Risk] `cmd-shift-n` may be intercepted by the OS or a future in-app feature before it
  reaches the app (e.g. some macOS versions reserve `cmd-shift-n` system-wide in certain
  contexts) → Mitigation: verify manually on the target macOS version during review; low
  likelihood since this is a per-app menu shortcut, not a system-wide one, and Finder
  itself uses this exact binding without conflict.
- [Risk] Future menu items may want `cmd-r`/`cmd-shift-r` for unrelated reload-like
  actions → Mitigation: none needed now; the modifier-family grouping documented above
  gives future contributors a clear pattern to extend consistently.
