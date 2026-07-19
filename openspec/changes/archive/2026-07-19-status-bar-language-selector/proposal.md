## Why

The app already supports three locales (`en`, `fr`, `de` — `fr`/`de` currently stubs) but only ever picks one automatically at startup, from the OS locale, with no way to see or change it while running. A user whose system locale isn't what they want the app in (or who wants to preview a translation) has no way to switch without changing their OS-wide language setting. The theme picker already establishes the exact UI pattern (status bar button + dropdown menu) for this kind of low-frequency, always-visible setting.

## What Changes

- Add a language/locale picker button to the status bar, positioned next to the existing theme picker, showing the active locale's name.
- Clicking it opens a dropdown menu (matching the theme picker's `dropdown_menu`/`PopupMenuItem` pattern) listing all supported locales; selecting one switches the app's active locale immediately, without a restart.
- The selected locale is persisted (alongside other lightweight UI preferences) and restored on next launch, taking precedence over the OS-detected locale.
- Language names in the dropdown are shown in their own language (endonym), e.g. "English", "Français", "Deutsch" — not translated into the currently active locale.

## Capabilities

### New Capabilities

- `status-bar-language-selector`: a status bar control that displays and switches the app's active locale at runtime, with the selection persisted across restarts.

### Modified Capabilities

- `status-bar-button-group-dividers`: the divider requirements between right-side status bar controls extend to cover the new language picker's position relative to the theme picker.

## Impact

- `crates/dtrpg-ui/src/i18n/mod.rs`: locale initialization gains a persisted-preference check ahead of OS-locale detection, and a runtime `switch_locale`-style entry point separate from startup `init`.
- `crates/dtrpg-ui/src/data/ui_prefs.rs`: `UiPrefsFile` gains a `locale: Option<String>` field with load/save accessors, following the existing `sidebar_width`/`settings_page_ix` pattern.
- `crates/dtrpg-ui/src/controllers/library.rs`: gains a `set_locale`-style mutation alongside the existing `set_theme`/`set_density`, to switch the active locale and trigger a re-render.
- `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`: `StatusBarSnapshot` gains the current locale; a new language picker button is added next to `theme_picker`, reusing the same dropdown-menu construction.
- No API or data model changes — this is a runtime UI preference, analogous to theme.
