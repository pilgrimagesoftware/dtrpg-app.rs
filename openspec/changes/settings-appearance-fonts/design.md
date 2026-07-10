## Context

Three independent pieces of "appearance" state exist today, none of them user-configurable from Settings:

- **Body font**: `theme.font_family = "Hoefler Text"` is set once in `app::setup` on the `gpui_component::Theme` global. Nothing else ever touches it.
- **Value/monospace fonts**: `VALUE_FONT` (`Optima`/`Segoe UI`/`DejaVu Sans`) and `MONOSPACE_FONT` (`Menlo`/`Consolas`/`Liberation Mono`) in `data/constants.rs` are `#[cfg(target_os = ...)]`-gated `const &str`s, resolved at compile time. Call sites (`settings_advanced_view.rs`'s `stat_row`/`timestamp_row`, `settings_account_view.rs`'s API key hint) reference them directly.
- **Theme**: `LibriTheme` (a `gpui::Global`) holds `key: ThemeKey`, `colors: ColorTokens`, `density`, `density_constants`. `LibraryController::set_theme` already exists and is wired to a status-bar popover menu (`status_bar_view.rs`) offering the four existing themes (Parchment, Slate, Sage, Ink). It updates the global and re-syncs `gpui_component::Theme`'s table colors, but never persists — `init_globals` (`util/init.rs`) unconditionally calls `LibriTheme::default_theme()` (Parchment) on every launch.

`UiPrefs` (`data/ui_prefs.rs`) is the existing lightweight persistence mechanism: a `UiPrefsFile` TOML struct under `{app_preferences_dir}/ui_prefs.toml`, loaded fresh (`UiPrefs::load()`) and flushed on every mutation — no long-lived instance. It already persists `settings_page_ix` for the Settings window's active page, which is the template this change follows for the new Appearance page and its selections.

The Settings window (`settings_view.rs`) has a fixed `PAGE_COUNT = 5` (Account, Downloads, File Openers, Advanced, About) driven by `SettingsController::active_page_ix`, itself persisted via `UiPrefs::save_settings_page_ix`.

## Goals / Non-Goals

**Goals:**

- A new Appearance page in Settings with pickers for the three font roles (body, value, monospace) and the active theme.
- All four selections persist across restarts and apply live (no restart required).
- Two new named themes, so the picker isn't just the same four options relocated.
- The existing status-bar theme quick-switcher keeps working and stays in sync with Settings.

**Non-Goals:**

- Freeform font entry or a full system font list — pickers offer a short curated, per-role list of platform-appropriate names, the same "fixed enum of named choices" pattern themes already use. Arbitrary user-typed font names risk silently falling back to a system default with no feedback.
- Per-element font overrides beyond the three existing roles (body/value/monospace) — no new font roles are introduced.
- Custom user-defined color themes (a color picker/theme editor) — the two new themes are curated palettes shipped with the app, same as the existing four.
- Persisting density — it's adjacent state on the same `LibriTheme` global but wasn't requested and has its own quick-switcher already; left untouched.

## Decisions

### Font choices are curated, per-role, ID-keyed enums — not raw strings

Each role gets a small fixed list of `(stable_id, display_label, platform_family_name)` options, mirroring how `ThemeKey` is a fixed enum rather than a free-text color spec:

```rust
pub struct FontOption {
    pub id: &'static str,           // persisted key, stable across platforms/releases
    pub label_key: &'static str,    // i18n key for the picker's display label
    pub family: &'static str,       // actual platform font family name passed to .font_family()
}
```

Curated lists (macOS names shown; Windows/Linux variants substitute the existing per-platform equivalents from the current `VALUE_FONT`/`MONOSPACE_FONT` constants where applicable):

- **Body** (`BODY_FONT_OPTIONS`): Hoefler Text (default), Georgia, Palatino, New York.
- **Value** (`VALUE_FONT_OPTIONS`): Optima (default), Helvetica Neue, Avenir, Verdana.
- **Monospace** (`MONO_FONT_OPTIONS`): Menlo (default), SF Mono, Monaco, Courier New.

Persisting `id` (not the platform-specific `family` string) means a preference saved on macOS degrades gracefully if the same prefs file is ever read on another platform (falls back to that role's default rather than requesting a font name that doesn't exist there) — same reasoning `ThemeKey` already gets for free by being an enum instead of a stored hex palette.

_Alternative considered:_ a single free-text font-name input. Rejected — no validation feedback if the name is wrong/unavailable (silent fallback to the OS default), and it's inconsistent with the theme picker's existing fixed-choice pattern.

### `LibriTheme` gains the three resolved font names; `LibraryController` gains the setters

`LibriTheme` already is the one global read by every view for appearance state (colors, density). Extending it with `body_font: &'static FontOption`, `value_font: &'static FontOption`, `mono_font: &'static FontOption` (resolved from the persisted `id`s at construction time, defaulting to each role's first/default option if the stored `id` is missing or unrecognized) keeps a single source of truth instead of a second global that has to stay in sync with `LibriTheme`.

`LibraryController` gains `set_body_font`, `set_value_font`, `set_mono_font` (mirroring the existing `set_theme`/`set_density` shape): each rebuilds the `LibriTheme` global with the new font resolved, updates `gpui_component::Theme.font_family` for the body-font case (the actual mechanism `app::setup` already uses), and persists via `UiPrefs`. `set_theme` gains a `UiPrefs` write it doesn't have today.

Call sites in `settings_advanced_view.rs` (`stat_row`, `timestamp_row`) and `settings_account_view.rs` (API key hint) switch from `VALUE_FONT`/`MONOSPACE_FONT` constants to `cx.global::<LibriTheme>().value_font.family` / `.mono_font.family`.

### Startup applies persisted appearance state instead of always defaulting

`init_globals` (`util/init.rs`) currently does `cx.set_global(LibriTheme::default_theme())` unconditionally. It changes to read `UiPrefs::load()` once for `theme_key`, `body_font_id`, `value_font_id`, `mono_font_id`, resolve each (falling back to defaults for anything missing/unrecognized — e.g. first launch, or a prefs file from before this change), and construct `LibriTheme` from that. This is the same "read persisted state at startup, default gracefully" shape `SettingsController::new` already uses for `settings_page_ix`.

### New themes: Moss and Rosewood

Two new `ThemeKey` variants following the existing `ColorTokens` structure and hex-based palette-function pattern (`parchment_colors()` etc.):

- **Moss**: a dark, cool forest-green palette — a second dark option alongside Ink, distinguished by a green rather than warm-brown cast (desktop background near `#0F1712`, surface `#16201A`, accent a muted gold-green `#9CB06A`).
- **Rosewood**: a warm burgundy/wine light palette — a second warm-light option alongside Parchment, leaning into deep reds/browns evoking leather binding (desktop background `#C9A8A0`, surface `#FBF3F1`, accent a deep wine red `#7A2C2C`).

Exact hex values are a starting point for implementation review, not final — tuned the same way the existing four were (checked for sufficient text/background contrast), not treated as load-bearing spec.

### Appearance page placement: appended as page index 5

`active_page_ix` is a persisted raw `usize` (`UiPrefs.settings_page_ix`). Inserting Appearance before Advanced/About would shift their indices and silently move users who have one of those pages pinned to a different page after upgrade. Appearance is appended after About (`PAGE_COUNT: 5 → 6`, new index `5`) instead — existing indices 0–4 keep their current meaning; `page_title`'s existing out-of-range fallback (defensive default to Account for a persisted index from a version with fewer pages) already covers a prefs file written by a version before this change.

## Risks / Trade-offs

- **[Risk]** A curated list can't cover every user's preferred font. → Mitigation: matches the existing theme picker's scope (curated, not open-ended); the list can grow in a future change without a migration (new `FontOption`s are additive, `id`-keyed).
- **[Risk]** Extending `LibriTheme` with font fields touches a widely-read global. → Mitigation: purely additive fields; every existing reader of `LibriTheme.colors`/`.density` is unaffected.
- **[Trade-off]** Two new themes are a meaningful design/testing surface (contrast, readability) beyond the code change itself. → Reviewed the same way the existing four were: primary/secondary/tertiary text against surface and desktop background for adequate contrast.
