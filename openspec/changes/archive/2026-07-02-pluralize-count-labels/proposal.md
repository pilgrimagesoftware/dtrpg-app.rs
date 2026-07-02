## Why

Count strings throughout the UI currently use fixed plural forms regardless of the actual count (e.g., "1 publisher items" instead of "1 publisher item"). This is grammatically incorrect and sets a poor foundation for the localization work planned for a future milestone.

## What Changes

- Introduce a `pluralize` utility function that returns the correct singular or plural noun form for a given count.
- Replace all hard-coded plural count strings in the UI with calls to this utility.
- Affected surfaces: toolbar count label, sidebar section suffix counts (Publishers, Collections), sidebar footer title count, and any other count-bearing labels.

## Capabilities

### New Capabilities

- `count-pluralization`: A `pluralize(count, singular, plural)` helper in `crates/dtrpg-ui/src/util/` that returns `"1 item"` vs `"2 items"` correctly, and is designed to be the single replacement point for a future i18n/l10n layer.

### Modified Capabilities

(none - no existing spec-level behavior changes)

## Impact

- `crates/dtrpg-ui/src/util/` - new `pluralize.rs` module
- `crates/dtrpg-ui/src/ui/views/toolbar_view.rs` - count label construction
- `crates/dtrpg-ui/src/ui/views/sidebar_view.rs` - section suffix counts and footer title count
- No API changes, no breaking changes, no dependency additions
