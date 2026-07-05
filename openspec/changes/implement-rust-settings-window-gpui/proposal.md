## Why

The app meta-repository now defines shared, language-agnostic settings window presentation behavior in `dtrpg-app/openspec/changes/define-shared-settings-window-presentation`: settings open in a separate, non-modal window instead of an in-window overlay. The Rust frontend needs a child implementation change to realize that behavior using gpui-specific window mechanics.

Today, settings render as an in-window overlay stacked on top of `LibraryRootView`'s content, blocking interaction with the catalog until closed.

## What Changes

- **BREAKING**: `SettingsController::open`/`close`/`is_open` and `render_settings_panel` no longer describe an in-window overlay; opening settings creates a separate `cx.open_window` window instead of toggling a backdrop layer in `LibraryRootView`.
- `Cmd-,` (`ShowSettings`) opens the settings window if not already open, or brings it to front if it is (single-instance reuse, per the shared spec).
- Closing the settings window (native close button, `Esc`, or an explicit close action) hides the window without quitting the app; reopening reuses the existing `SettingsController` entity so drafts/tab/scroll state persist rather than rebuilding from scratch.
- The settings backdrop/focus-trap logic in `LibraryRootView` (`settings_focus`, the `is_open` conditional overlay child) is removed since there's no longer an overlay to manage inside the main window.

## Capabilities

### New Capabilities

- `rust-settings-window-implementation`: gpui-specific implementation of the shared settings window presentation behavior — how the settings window is opened via `cx.open_window`, how a single open instance is tracked and reused, and how `SettingsController` state is shared across the main and settings windows.

### Modified Capabilities

(none)

## Impact

- `dtrpg-ui/src/ui/app/mod.rs`: add a window-opening function analogous to `open_library_window` for the settings window, plus tracking of whether a settings window is already open (so `ShowSettings` can bring it to front instead of opening a duplicate).
- `dtrpg-ui/src/ui/views/root_view.rs`: remove the `settings_snap.is_open` overlay branch, the `settings_focus` focus-trap handling, and the `render_settings_panel` call from `LibraryRootView`'s render path; `ShowSettings` action now triggers window creation/focus instead of `ctrl.open(cx)`.
- `dtrpg-ui/src/ui/views/settings_view.rs`: `render_settings_panel` becomes the root view content of the new window (its own `Root`/window chrome) rather than a child overlay composited into the library window.
- `dtrpg-ui/src/controllers/settings.rs`: `SettingsController::open`/`close`/`is_open` semantics shift from "overlay visibility" to "settings window lifecycle"; the controller itself is unchanged in shape but is now shared between two windows via its entity handle.
- No changes to persisted settings data, credential storage, or SDK/service layers.
- Depends on `dtrpg-app/openspec/changes/define-shared-settings-window-presentation`.
