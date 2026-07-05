## Context

`dtrpg-app/openspec/changes/define-shared-settings-window-presentation` defines the language-agnostic behavior this change implements: settings open in a separate, non-modal window; a single instance is reused on repeat invocation; state persists across close/reopen; closing settings never quits the app. This document covers only the gpui-specific mechanics for satisfying that behavior in the Rust frontend.

Settings currently render inside `LibraryRootView::render` as a conditional overlay: when `settings_snap.is_open` is true, `render_settings_panel(...)` is composited as a child of the main content `div`, and a dedicated `settings_focus` handle is focused so `Esc` can close it. This mirrors the pattern gpui-component's `open_dialog` uses for lightweight confirmation dialogs, but settings is a full panel with tabs (account, storage, file openers, advanced), not a transient prompt.

`SettingsController` (an `Entity<SettingsController>`) already holds all settings state (auth snapshot, file openers, storage path, drafts) independent of the view. The main window already knows how to open additional native windows: `open_library_window` in `dtrpg-ui/src/ui/app/mod.rs` calls `cx.open_window(WindowOptions { .. }, |window, cx| { .. })` and wraps the root view in `gpui_component::Root`. Settings can follow the same pattern.

## Goals / Non-Goals

**Goals:**
- Settings open in a real OS window (its own titlebar, independently movable/closable/resizable) rather than an in-window overlay.
- The settings window is non-modal: the main library window stays interactive while settings are open (no focus-trap, no backdrop blocking clicks).
- `Cmd-,` / the Settings menu item continues to work as the single entry point; invoking it again while the window is open brings that window to front instead of creating a second one.
- Closing the settings window doesn't destroy `SettingsController` state (draft text fields, scroll position, etc. persist for the session) and doesn't quit the app.

**Non-Goals:**
- No change to what settings contain (account, storage, file openers, advanced tabs) or how they're persisted.
- No change to credential storage or the SDK/service layer.
- Not adding multi-window support as a general framework feature — this is a single additional window, following the existing `open_library_window` pattern, not a generalized window manager.

## Decisions

- **One settings window, tracked by a global/handle.** Store the open settings window's `WindowHandle` (or an `Option<AnyWindowHandle>`) alongside `ServiceFactory` et al. as an app-level global (or a field the `ShowSettings` handler closes over), set when the window opens and cleared when it closes. `ShowSettings` checks this handle first: if present and the window is still open, call `cx.activate_window`/bring-to-front; otherwise open a new window.
  - Alternative considered: let `SettingsController::is_open` continue to be the source of truth and have the window's presence driven reactively from it. Rejected because window lifecycle (open/close/focus) is a `cx`-level, not entity-level, concern in gpui — the handle needs to live where `cx.open_window` was called.
- **Reuse the existing `SettingsController` entity across window opens.** The controller entity is created once (as today, presumably alongside `LibraryRootView`) and passed into the settings window's view constructor by clone, same as `settings_entity` is passed into `render_settings_panel` today. This preserves in-progress edits (e.g. a half-typed storage path) across close/reopen without re-fetching from services.
- **`SettingsController::open`/`close` become window-lifecycle hooks, not visibility flags consumed by a conditional render.** `open(cx)` is called right before/when `cx.open_window` succeeds; `close(cx)` is called from the new window's close handler (`on_close_window` or equivalent) so `is_open` still accurately reflects "is the settings window currently open" for any other code that reads it (e.g. disabling the menu item).
- **Window is resizable and non-modal by construction** — gpui's `cx.open_window` produces an independent top-level window by default; there's no modal/sheet flag to set. "Non-modal" is achieved simply by *not* routing settings through `window.open_dialog` (which layers within the same window) and instead using a sibling `WindowOptions` window, matching `open_library_window`.
- **Remove `settings_focus` and the overlay branch from `LibraryRootView`.** The focus-trap and `Esc`-to-close behavior specific to the overlay no longer apply; the settings window will get its own `Esc`-to-close binding scoped to that window's root focus handle instead, following the pattern used for other top-level windows in this app.

## Risks / Trade-offs

- [Risk] Two entities (main window's root view and the new settings window) both hold references into the same `SettingsController` entity; a bug in one window's update could unexpectedly re-render the other. → Mitigation: `SettingsController` already exists as a shared `Entity<T>` today (passed into `render_settings_panel`), so this sharing pattern is not new — only the fact that it's now rendered inside a second *window* rather than a second *view in the same window* is new. gpui's entity update/notify model already handles cross-view subscriptions correctly for this.
- [Risk] Users could invoke `Cmd-,` rapidly before the window-open future resolves, opening duplicate windows. → Mitigation: set the tracked handle synchronously at the point `cx.open_window` is called (it returns a handle immediately, not just on first paint), so a second `Cmd-,` within the same tick sees the handle populated.
- [Risk] Closing the main library window (e.g. quitting via `Cmd-Q`) while the settings window remains open could leave an orphaned window or a settings window with no underlying data. → Mitigation: no behavior change needed here — `Cmd-Q` already calls `cx.quit()`, which tears down all windows; this is existing app-quit behavior, not something this change introduces.

## Migration Plan

- Implemented as a single PR within `dtrpg-app/rust`: add the settings-window-opening function, rewire `ShowSettings`, remove the overlay branch, verify manually that settings open/close/reopen preserves state and that the main window stays interactive throughout.
- No data migration; no feature flag needed since this is a pure UI presentation change with no persisted-state schema impact.
- Rollback is a straight revert of the PR if issues surface.

## Open Questions

- Should the settings window remember its last screen position/size across app restarts, or always open at a default position/size? Deferred to implementation; default to a fixed initial size (matching the overlay's current width) unless the user asks for persistence.
