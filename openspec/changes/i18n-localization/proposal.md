## Why

All user-visible strings are hardcoded English literals scattered across the view layer. There is no mechanism to serve the app in any other language. Adding i18n infrastructure now, while the string count is manageable (~150 strings across 10 view files), costs far less than retrofitting it after the UI matures.

## What Changes

- Add `rust-i18n` as a workspace dependency for compile-time key extraction and a `t!()` macro at call sites; add `sys-locale` to detect the system locale at startup
- Add `crates/dtrpg-ui/src/i18n/` module containing a `LocaleManager` GPUI global, locale initialization on startup, and the embedded YAML locale files (`en.yaml`, scaffolded `fr.yaml`, `de.yaml`)
- Replace all hardcoded user-facing strings in `ui/views/` and `util/` with `t!("key")` calls, keyed by dot-notation (e.g., `t!("sidebar.all_titles")`)
- Wire locale detection into app startup: read system locale via `sys-locale`, fall back to `en` if no translation exists

**BREAKING**: None -- behavior is identical for English users on first deploy.

## Capabilities

### New Capabilities

- `locale-manager`: GPUI global that holds the active locale string, initialized from the system locale at startup with English fallback
- `string-catalog`: YAML locale files under `crates/dtrpg-ui/i18n/` holding all translatable strings, embedded in the binary at compile time via `rust-i18n`

### Modified Capabilities

- None

## Impact

- `Cargo.toml` (workspace): add `rust-i18n`, `sys-locale`
- `crates/dtrpg-ui/Cargo.toml`: opt in to both dependencies
- `crates/dtrpg-ui/src/i18n/mod.rs`: new module (`LocaleManager` global, init function)
- `crates/dtrpg-ui/i18n/en.yaml`: ~150 English string keys
- `crates/dtrpg-ui/i18n/fr.yaml`, `de.yaml`: scaffolded with same keys, values copied from English (translation work is out of scope)
- `crates/dtrpg-ui/src/lib.rs`: call `i18n::init()` on startup
- `crates/dtrpg-ui/src/ui/views/*.rs`: replace ~150 string literals with `t!()` calls
- `crates/dtrpg-ui/src/util/filter.rs` and `util/datetime.rs`: any locale-sensitive formatting
