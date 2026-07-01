## Context

The UI layer contains ~150 hardcoded English strings across 10 view files and a handful of utility modules. GPUI render functions receive `cx: &mut App`; any GPUI global is accessible there. Startup is wired through `crates/dtrpg-ui/src/util/init.rs` and `src/ui/windows/app.rs`.

## Goals / Non-Goals

**Goals:**
- One call site pattern (`t!("dot.key")`) to replace every hardcoded string
- System locale auto-detected at startup; English fallback when no translation exists
- Locale files embedded in the binary (no runtime file I/O)
- Scaffolded locale files for `fr` and `de` so translators have a template to fill

**Non-Goals:**
- Actual translations (values in `fr.yaml` / `de.yaml` start as English copies)
- Runtime locale switching without restart
- Pluralization rules beyond what `rust-i18n` provides out of the box
- Right-to-left layout support

## Decisions

### Use `rust-i18n` rather than Fluent or a custom solution

`rust-i18n` provides a `t!()` proc-macro, compile-time key extraction, YAML locale files, and a `set_locale()` / `get_locale()` global. The call-site ergonomics (`t!("key")`) match what we need and the proc-macro validates keys at compile time. Fluent (Mozilla's FTL format) is more expressive but far more complex to set up and overkill for ~150 static strings with minimal pluralization. A custom solution would require reinventing the same wheel.

### YAML locale file location: `crates/dtrpg-ui/i18n/<locale>.yaml`

`rust-i18n`'s `i18n!()` macro scans a directory specified at the call site. Placing files in `crates/dtrpg-ui/i18n/` keeps them next to the crate that owns all UI strings.

### Key naming convention: `<module>.<snake_case_description>`

Examples: `sidebar.all_titles`, `detail.read_button`, `catalog.no_matches`. Module prefixes keep keys organized and avoid collisions. Snake_case matches Rust conventions.

### Locale detection via `sys-locale`

`sys-locale::get_locale()` returns the OS locale string (e.g. `"fr-FR"`). Strip the region tag to get the language code (`"fr"`), then call `rust_i18n::set_locale("fr")`. Fall back to `"en"` if the detected locale has no corresponding YAML file.

### Initialization point: `i18n::init()` called from `LibraryRootView::new()` (or app startup)

`rust-i18n` uses a process-global locale. It must be set before the first `t!()` call. `i18n::init()` reads the system locale, selects the best available locale, and calls `rust_i18n::set_locale()`. Called once at app startup from the existing init path in `util/init.rs`.

### No `LocaleManager` GPUI global needed

Since `rust-i18n` already manages a process-global locale (accessed via `rust_i18n::locale()`), wrapping it in a GPUI global adds no value. `t!("key")` works anywhere -- in render functions, closures, or outside GPUI -- without `cx`.

## Risks / Trade-offs

- [Key typos] `rust-i18n` validates keys at compile time if the `i18n!()` macro is set up correctly. Any missing key falls back to the key string itself at runtime, which is visible but not a crash. Mitigation: CI step `cargo test` exercises all view code paths.
- [Binary size] Embedding YAML for three locales at ~150 strings each is negligible (a few KB).
- [YAML duplication] `fr.yaml` and `de.yaml` start as English copies. Untranslated entries are indistinguishable from translated ones without a completeness check. Mitigation: add a `#` comment header in each non-English file noting it is incomplete.

## Open Questions

- None blocking implementation. Actual translation work is deferred explicitly.
