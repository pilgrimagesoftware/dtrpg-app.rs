## 1. Font option data

- [ ] 1.1 In `data/constants.rs`, define a `FontOption { id: &'static str, label_key: &'static str, family: &'static str }` struct
- [ ] 1.2 Define `BODY_FONT_OPTIONS: &[FontOption]` (Hoefler Text default, Georgia, Palatino, New York on macOS; existing-equivalent per-platform substitutes on Windows/Linux)
- [ ] 1.3 Define `VALUE_FONT_OPTIONS: &[FontOption]` (Optima default, Helvetica Neue, Avenir, Verdana on macOS; replaces the current per-platform `VALUE_FONT` const)
- [ ] 1.4 Define `MONO_FONT_OPTIONS: &[FontOption]` (Menlo default, SF Mono, Monaco, Courier New on macOS; replaces the current per-platform `MONOSPACE_FONT` const)
- [ ] 1.5 Add a small lookup helper, e.g. `fn resolve_font(options: &[FontOption], id: Option<&str>) -> &'static FontOption`, returning the first (default) option when `id` is `None` or unrecognized
- [ ] 1.6 Remove the now-unused `VALUE_FONT`/`MONOSPACE_FONT` constants once all call sites are migrated (see section 4)

## 2. New themes: Moss and Rosewood

- [ ] 2.1 Add `ThemeKey::Moss` and `ThemeKey::Rosewood` variants in `data/theme.rs`
- [ ] 2.2 Implement `moss_colors() -> ColorTokens` (dark forest green palette) following the existing hex-based palette function pattern
- [ ] 2.3 Implement `rosewood_colors() -> ColorTokens` (warm burgundy palette) following the same pattern
- [ ] 2.4 Wire both into `LibriTheme::new`'s match arm
- [ ] 2.5 Add `theme.moss` / `theme.rosewood` i18n labels (en/de/fr) and wire them into `status_bar_view.rs`'s `theme_label` and theme list
- [ ] 2.6 Verify text/background contrast for both new palettes (primary/secondary/tertiary text against surface and desktop background)

## 3. Persistence

- [ ] 3.1 Add `theme_key: Option<String>`, `body_font_id: Option<String>`, `value_font_id: Option<String>`, `mono_font_id: Option<String>` fields to `UiPrefsFile`
- [ ] 3.2 Add matching getter/setter methods to `UiPrefs` (`theme_key()`/`save_theme_key()`, etc.), following the existing `settings_page_ix` pattern
- [ ] 3.3 In `util/init.rs`'s `init_globals`, replace the unconditional `LibriTheme::default_theme()` with a `UiPrefs::load()` read that resolves the persisted theme key and three font ids (falling back to defaults for anything missing/unrecognized) before constructing the initial `LibriTheme`

## 4. `LibriTheme` and controller wiring

- [ ] 4.1 Add `body_font: &'static FontOption`, `value_font: &'static FontOption`, `mono_font: &'static FontOption` fields to `LibriTheme`
- [ ] 4.2 Update `LibriTheme::new`/`default_theme` to accept/resolve font selections (or add a separate constructor used only at startup with the persisted ids, keeping `set_theme`/`set_density` call sites simple by preserving current font selections when only theme/density changes)
- [ ] 4.3 Add `LibraryController::set_body_font`, `set_value_font`, `set_mono_font`, mirroring `set_theme`'s shape: rebuild the `LibriTheme` global, update `gpui_component::Theme.font_family` for the body-font case, persist via `UiPrefs`, `cx.notify()`
- [ ] 4.4 Update `LibraryController::set_theme` to also persist the theme key via `UiPrefs`
- [ ] 4.5 Update `settings_advanced_view.rs`'s `stat_row`/`timestamp_row`/`row_frame` value-font usage to read `cx.global::<LibriTheme>().value_font.family` instead of the `VALUE_FONT` constant (threading `cx` or the resolved family name through as needed)
- [ ] 4.6 Update `settings_account_view.rs`'s API key hint to read `.mono_font.family` instead of the `MONOSPACE_FONT` constant

## 5. Appearance settings page

- [ ] 5.1 Create `ui/views/settings_appearance_view.rs` with `render_appearance_section(entity: Entity<LibraryController>, colors: &ColorTokens) -> impl IntoElement`, rendering four picker rows (body font, value font, monospace font, theme) using a dropdown/select component consistent with existing settings-page patterns
- [ ] 5.2 Increase `settings_view.rs`'s `PAGE_COUNT` from 5 to 6; add page index 5 → Appearance in the `page_title` match and the `render_settings_panel` content match (appended after About, per design.md's index-stability decision — do not renumber existing indices)
- [ ] 5.3 Add `settings.appearance_title` and picker-label i18n keys (en/de/fr): body font, value font, monospace font, theme, plus per-option display labels for the new `FontOption` entries
- [ ] 5.4 Wire each picker's selection callback to the corresponding `LibraryController` setter from section 4

## 6. Status-bar sync

- [ ] 6.1 Confirm `status_bar_view.rs`'s theme quick-switcher already reads `cx.global::<LibriTheme>()` (it does) so it automatically reflects Settings-driven theme changes with no additional wiring
- [ ] 6.2 Add the two new `ThemeKey` variants to the quick-switcher's theme list (`[ThemeKey::Parchment, ThemeKey::Slate, ThemeKey::Sage, ThemeKey::Ink]` → include `Moss`, `Rosewood`)

## 7. Verification

- [ ] 7.1 Run `cargo check --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo +nightly fmt -- --check`, `cargo test --all-features --workspace`
- [ ] 7.2 Manually verify: change each of the four Appearance selections; confirm each applies immediately without restart
- [ ] 7.3 Manually verify: quit and relaunch after changing all four selections; confirm all four persist
- [ ] 7.4 Manually verify: change theme from the status-bar quick-switcher; confirm Settings > Appearance reflects it, and vice versa
- [ ] 7.5 Manually verify: Moss and Rosewood themes render with adequate text/background contrast throughout the main window, detail panel, and settings window
