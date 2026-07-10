## Why

Font choices (`Hoefler Text` for body text, `Optima` for data values, `Menlo` for monospace data) and the color theme are all currently either hardcoded constants or, for theme, changeable only from a status-bar quick-switcher with no persistence across restarts. Users have no way to pick a font that suits their taste or system, and every relaunch silently resets to the Parchment theme. A dedicated Appearance page in Settings gives both a persistent, discoverable home.

## What Changes

- Add a new "Appearance" page to the Settings window (alongside Account, Downloads, File Openers, Advanced, About), with font pickers and a theme picker.
- Convert the three font roles — body font, data-value font, monospace font — from compile-time platform constants (`data/constants.rs`) into a curated, user-selectable, persisted preference. Each role offers a short list of platform-appropriate named choices (not freeform text entry), matching how theme selection already works as a fixed enum rather than arbitrary user input.
- Move theme selection into the new Appearance page as the primary control surface; the existing status-bar quick-switcher continues to work and stays in sync with whatever is selected in Settings.
- Persist the active theme, which currently resets to Parchment on every launch — `set_theme` updates the GPUI global but never writes to `UiPrefs`.
- Add two new named color themes (in addition to the existing Parchment, Slate, Sage, and Ink), giving the theme picker more real choices rather than just relocating the same four.
- Persist the three font preferences the same way (`UiPrefs`), restoring them at startup instead of always applying the hardcoded constants.

## Capabilities

### New Capabilities

- `settings-appearance-page`: A new Settings page presenting font-role pickers (body, value, monospace) and a theme picker, each showing the current selection and applying changes live.
- `configurable-fonts`: The three font roles are user-selectable from a curated, platform-appropriate list and persist across restarts, replacing the current hardcoded `VALUE_FONT`/`MONOSPACE_FONT`/body `font_family` constants.

### Modified Capabilities

- `libri-theme`: adds two new named color themes; theme selection persists across restarts instead of resetting to Parchment on every launch.

## Impact

- `dtrpg-ui/src/data/theme.rs`: add two new `ThemeKey` variants and their palette functions; no change to the existing four palettes.
- `dtrpg-ui/src/data/constants.rs`: `VALUE_FONT`/`MONOSPACE_FONT` per-platform constants are replaced by a curated enum of selectable font names (still platform-gated for which options are offered), used as defaults only.
- `dtrpg-ui/src/data/ui_prefs.rs`: `UiPrefsFile` gains fields for the active theme key and the three font selections; `UiPrefs` gains matching load/save methods.
- `dtrpg-ui/src/controllers/library.rs`: `set_theme` persists to `UiPrefs`; new `set_body_font`/`set_value_font`/`set_monospace_font` methods (or equivalent) update the live `LibriTheme`/font state and persist the choice.
- `dtrpg-ui/src/ui/views/settings_view.rs`: `PAGE_COUNT` increases by one; new page routes to the Appearance section.
- New `dtrpg-ui/src/ui/views/settings_appearance_view.rs`: renders the font and theme pickers.
- `dtrpg-ui/src/ui/views/settings_advanced_view.rs`, `settings_account_view.rs`: value/monospace font call sites read the live preference instead of the `VALUE_FONT`/`MONOSPACE_FONT` constants.
- `dtrpg-ui/src/ui/views/status_bar_view.rs`: theme quick-switcher unchanged in behavior, now reflects/drives the same persisted state as the Settings page.
- i18n: new `settings.appearance_title` page title, font/theme picker labels, and two new theme name keys (`theme.<new1>`, `theme.<new2>`) across `en`/`de`/`fr`.
