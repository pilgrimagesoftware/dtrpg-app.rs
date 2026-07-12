## Why

The library window always opens at the OS's default placement and size — resizing or moving it has no effect on the next launch. `UiPrefs` already persists other layout preferences (sidebar/detail panel widths, settings page index), but never the window's own position or size, so every session starts from the same default instead of wherever the user last left it.

## What Changes

- Persist the library window's position and size to `UiPrefs` whenever the window is about to close, and restore them on next launch instead of always using the OS default placement.
- If the saved bounds no longer intersect any currently connected display (e.g. an external monitor was disconnected), fall back to the default placement rather than opening off-screen or on the wrong monitor.
- Scoped to the main library window only. The settings window is fixed-size, always centers on open (see `settings-appearance-fonts`), and has no independent position/size worth remembering.

## Capabilities

### New Capabilities

- `remember-window-bounds`: the library window's position and size persist across restarts, with a validated fallback to the default placement when the saved bounds are no longer usable.

### Modified Capabilities

- None

## Impact

- `dtrpg-ui/src/ui/app/mod.rs`: `open_library_window` reads a persisted `WindowBounds` (falling back to the OS default) instead of using `WindowOptions::default()` for `window_bounds`; registers a `window.on_window_should_close` hook (mirroring `open_settings_window`'s existing pattern) that captures `window.bounds()` and saves it before the window closes.
- `dtrpg-ui/src/data/ui_prefs.rs`: `UiPrefsFile` gains a persisted window-bounds field; `UiPrefs` gains matching load/save methods.
- No controller or view-rendering changes — this is startup/shutdown plumbing only.
