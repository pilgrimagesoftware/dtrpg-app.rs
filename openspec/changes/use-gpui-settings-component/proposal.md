## Why

The settings panel is currently built with a hand-rolled tab strip, per-section rendering functions, and `SettingsTab` state in `SettingsController`. `gpui-component` already ships a `Settings` component (`SettingPage`, `SettingGroup`, `SettingItem`) that provides sidebar navigation, built-in search, and a consistent layout — all capabilities the hand-rolled implementation is missing or duplicating.

## What Changes

- Replace `render_tab_strip` and the `SettingsTab`-driven `render_active_section` in `settings_view.rs` with a `Settings` component from `gpui_component::setting`.
- Each existing section (Account, Storage, File Openers) becomes a `SettingPage`; section content is wrapped in `SettingItem::render` custom elements so existing render logic is preserved.
- Drop `SettingsTab` enum, `active_tab` field, `set_tab()`, and `active_tab()` from `SettingsController`. Replace `open_to(tab, cx)` with an `open(cx)` call that selects an initial page by index via `Settings::default_selected_index`.
- Add built-in search to the settings sidebar (provided for free by `Settings`).
- Increase the settings modal width from 560 px to 720 px to accommodate the sidebar.

## Capabilities

### New Capabilities

- `settings-panel`: Settings panel using the gpui-component `Settings` component with sidebar navigation, page-based layout, and search.

### Modified Capabilities

<!-- none -->

## Impact

- `dtrpg-ui`: `settings_view.rs` (replaced), `controllers/settings.rs` (SettingsTab removed).
- All callers of `render_settings_panel` that pass `active_tab` will need updating (primary caller is the main window view).
- All callers of `open_to`, `set_tab`, `active_tab` in `SettingsController` will need updating.
- No new crate dependencies — `gpui_component::setting` is already a transitive dependency.
