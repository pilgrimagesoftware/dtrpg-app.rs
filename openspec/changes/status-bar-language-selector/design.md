## Context

Locale is currently a one-shot decision: `i18n::init()` (`crates/dtrpg-ui/src/i18n/mod.rs`) runs once at startup via `util::init::init_globals`, detects the OS locale through `sys_locale::get_locale()`, matches it against the supported set (`en`, `fr`, `de`), and calls `rust_i18n::set_locale(...)`. `rust_i18n::set_locale` mutates a process-wide static, not a GPUI `Global` — every `t!("...")` call anywhere in the crate reads that static at the moment it's evaluated, so changing it takes effect on the very next render pass with no propagation needed beyond triggering GPUI to actually re-render.

The theme picker (`status_bar_view.rs::render_status_bar`, `theme_picker`) is the closest existing analog: a status bar `Button` with `.dropdown_menu(...)` populated with `PopupMenuItem`s, one per `ThemeKey`, calling `LibraryController::set_theme` on click. `set_theme` updates the `LibriTheme` GPUI `Global` and calls `cx.notify()` on the controller entity, which is what actually causes the app to re-render with the new theme — the same `cx.notify()` call is what a locale switch needs, since GPUI has no way to know a process-wide static changed on its own.

`UiPrefs` (`data/ui_prefs.rs`) is the existing small-preferences store — sidebar width, panel widths, section-open flags, last settings page — backed by `{app_preferences_dir}/ui_prefs.toml`, loaded fresh via `UiPrefs::load()` wherever needed (no caching) and flushed to disk synchronously on every `save_*` call. Theme itself is *not* persisted today (it resets to Parchment on every launch) — this change does not fix that; it only establishes locale persistence, which the proposal explicitly calls for.

## Goals / Non-Goals

**Goals:**
- A status bar button next to the theme picker shows the active locale's endonym (e.g. "English") and opens a dropdown of all supported locales on click.
- Selecting a locale switches the app's displayed language immediately, with no restart.
- The chosen locale persists across restarts, read before the OS-locale fallback.

**Non-Goals:**
- No changes to which locales are supported (`en`, `fr`, `de` stay as-is; `fr`/`de` remain translation stubs — this change is about the switcher, not translation completeness).
- No RTL layout support (not needed for any currently supported locale).
- No per-window or per-document locale — one process-wide active locale, same as today.
- Not fixing theme's lack of persistence — out of scope, unrelated to this change's requirements.

## Decisions

### New `Locale` enum in `crate::i18n`, mirroring `ThemeKey`'s shape

A `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` enum `Locale { En, Fr, De }` with a `code(&self) -> &'static str` (`"en"`/`"fr"`/`"de"`, fed to `rust_i18n::set_locale`) and a `label(&self) -> &'static str` returning the endonym ("English"/"Français"/"Deutsch"). Endonyms are hardcoded Rust string constants, not translation keys — a language's own name doesn't get translated into whatever locale is currently active (showing "Anglais" while the menu is in French would be confusing when the user is trying to find their way back to English). This mirrors `theme_label`'s shape in `status_bar_view.rs` but deliberately skips `t!()`.

_Alternative considered_: derive supported locales from the existing `["en", "fr", "de"]` array in `i18n::init`. Rejected — that array is a runtime detection filter, not a UI-facing enum; a typed enum gives the status bar dropdown compile-time exhaustiveness (matching how `ThemeKey`'s four variants are matched exhaustively in `theme_picker`), and avoids stringly-typed plumbing through the controller and view layers.

### Persist locale in `UiPrefsFile`, read at startup ahead of OS detection

Add `locale: Option<String>` to `UiPrefsFile`, with `UiPrefs::locale() -> Option<String>` and `UiPrefs::save_locale(&mut self, code: &str)` following the exact shape of `settings_page_ix`/`save_settings_page_ix`. `i18n::init()` changes to accept an optional override: check `UiPrefs::load().locale()` first; if present and still in the supported set, use it; otherwise fall back to the existing OS-locale detection. This keeps `i18n::init()` as the single startup entry point (called once from `util::init::init_globals`) rather than adding a second locale-resolution path.

_Alternative considered_: store locale as a `LibriTheme`-style `Global` instead of/alongside a preference file, mirroring how theme state itself lives in a `Global`. Rejected for the persisted value specifically — theme's `Global` is exactly why theme doesn't survive restarts today (nothing ever reads or writes it to disk); locale explicitly needs to survive restarts per the proposal, so it needs a file-backed store from the start, matching `UiPrefs`'s existing job. The *runtime* active locale still doesn't need its own `Global` because `rust_i18n`'s static already serves that role — GPUI only needs a `cx.notify()` trigger, not a value to read back (the current `Locale` is derived from `rust_i18n::locale()` when the status bar needs to display it, not stored redundantly in a second place).

### `LibraryController::set_locale`, calling `rust_i18n::set_locale` + persisting + `cx.notify()`

New method alongside `set_theme`/`set_density`:
```rust
pub fn set_locale(&self, locale: Locale, cx: &mut Context<Self>) {
    rust_i18n::set_locale(locale.code());
    UiPrefs::load().save_locale(locale.code());
    cx.notify();
}
```
Same shape as `set_theme`, called the same way from the status bar's dropdown menu item `on_click`. `cx.notify()` is what actually causes GPUI to re-render the tree with the new locale's strings — `rust_i18n::set_locale` alone has no visible effect until something triggers a re-render, exactly analogous to how `set_theme`'s `cx.set_global` alone wouldn't repaint without the trailing `cx.notify()`.

_Alternative considered_: read/derive the current locale from `rust_i18n::locale()` directly in the status bar render function instead of adding a `StatusBarSnapshot` field. Considered but rejected only for symmetry — `StatusBarSnapshot::theme_key` already exists as an explicit snapshot field rather than reading the `LibriTheme` global inline inside `render_status_bar`, so `current_locale` follows the same convention for consistency with the rest of the snapshot's fields, even though `rust_i18n::locale()` would technically work either way.

### Status bar placement: language picker immediately left of the theme picker, same divider treatment

`render_status_bar`'s right side currently reads: `theme_picker | divider | activity_panel | divider | notification_panel`. This adds `language_picker | divider | theme_picker | divider | activity_panel | divider | notification_panel` — the new control goes on the *outer* (leftmost of the right-aligned group) side of the theme picker rather than between theme and activity, so the two "appearance" controls (language, theme) stay adjacent to each other as a pair, separated from the "activity" controls by their own divider. This is what the proposal's modified-capability note on `status-bar-button-group-dividers` covers: that spec's requirement text enumerates the specific button pairs a divider sits between, and needs a delta adding the language/theme pair.

## Risks / Trade-offs

- [Risk] `fr`/`de` are still translation stubs (see `i18n/mod.rs` doc comment) — switching to them will show mostly-English or partially-translated UI. → Mitigation: this is a pre-existing, known condition unrelated to the switcher itself; the proposal doesn't claim to complete translations, and the switcher is equally useful today for previewing stub coverage.
- [Trade-off] `UiPrefs::load()` re-reads the TOML file on every call (no caching), same as every other `UiPrefs` accessor in this codebase today (`root_view.rs`'s resize-state subscription calls `UiPrefs::load()` fresh per resize event). A locale switch is a rare, explicit user action, so the extra disk read is not a measurable concern — consistent with the existing pattern, not a new cost class.
- [Risk] If the persisted `locale` value in `ui_prefs.toml` is no longer in the supported set (e.g. a future downgrade removes a locale that was once available), `i18n::init()`'s override check must fall through to OS detection rather than passing an unsupported code to `rust_i18n::set_locale`. → Mitigation: the override check explicitly re-validates the persisted code against the same `supported` array used for OS-locale matching, so this degrades to today's default behavior rather than silently doing nothing.
