## 1. Locale type and persistence

- [ ] 1.1 In `crates/dtrpg-ui/src/i18n/mod.rs`, add `Locale` enum (`En`, `Fr`, `De`) with `code(&self) -> &'static str` and `label(&self) -> &'static str` (endonyms: "English", "Français", "Deutsch"), plus `Locale::ALL: [Locale; 3]` for iteration
- [ ] 1.2 In `crates/dtrpg-ui/src/data/ui_prefs.rs`, add `locale: Option<String>` to `UiPrefsFile`, with `UiPrefs::locale(&self) -> Option<String>` and `UiPrefs::save_locale(&mut self, code: &str)` following the `settings_page_ix`/`save_settings_page_ix` pattern
- [ ] 1.3 In `crate::i18n::init()`, check `UiPrefs::load().locale()` first; if `Some(code)` and `code` is in the supported set, use it, otherwise fall back to the existing OS-locale detection (unchanged)
- [ ] 1.4 Unit tests for `Locale::code`/`Locale::label` round-tripping and for `init()`'s override-vs-fallback precedence (persisted valid code wins, persisted invalid/unsupported code falls through to OS detection, no persisted value falls through unchanged)

## 2. Controller mutation

- [ ] 2.1 In `crates/dtrpg-ui/src/controllers/library.rs`, add `LibraryController::set_locale(&self, locale: Locale, cx: &mut Context<Self>)` alongside `set_theme`/`set_density`: calls `rust_i18n::set_locale(locale.code())`, then `UiPrefs::load().save_locale(locale.code())`, then `cx.notify()`

## 3. Status bar UI

- [ ] 3.1 In `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`, add `current_locale: Locale` to `StatusBarSnapshot`
- [ ] 3.2 Add a `language_picker` `Button` in `render_status_bar`, mirroring `theme_picker`'s construction: `.label(snap.current_locale.label())`, `.tooltip(...)`, `.dropdown_menu(...)` iterating `Locale::ALL`, each `PopupMenuItem` checked when it equals `snap.current_locale` and calling `LibraryController::set_locale` on click
- [ ] 3.3 Insert `language_picker` and a `Separator::vertical().h_5()` immediately before `theme_picker` in the `StatusBar` right-side chain (`language_picker | divider | theme_picker | divider | activity_panel | divider | notification_panel`)
- [ ] 3.4 In `crates/dtrpg-ui/src/ui/views/root_view.rs`, populate `StatusBarSnapshot::current_locale` at the `render_status_bar(StatusBarSnapshot { ... })` call site (derive from `rust_i18n::locale()`, mapped back to a `Locale`)

## 4. Localization strings

- [ ] 4.1 Add a `status_bar.language_tooltip` key to `crates/dtrpg-ui/i18n/en.yaml`, `fr.yaml`, and `de.yaml`, following the existing `status_bar.theme_tooltip` phrasing/placeholder convention

## 5. Verification

- [ ] 5.1 `cargo build --workspace --all-features`
- [ ] 5.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 5.3 `cargo test --workspace --all-features`
- [ ] 5.4 Launch app: open the language picker, switch locales, confirm status bar and other visible text update immediately without restart; relaunch and confirm the selected locale persists
