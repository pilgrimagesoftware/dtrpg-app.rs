## ADDED Requirements

### Requirement: System locale is detected and applied at startup
The app SHALL read the OS locale at startup via `sys-locale` and activate the best matching locale from the available translations. If no match exists, it SHALL fall back to `en`.

#### Scenario: Matching locale available
- **WHEN** the OS locale is `"fr-FR"` and `fr.yaml` exists in the string catalog
- **THEN** the active locale is set to `"fr"` before any UI string is rendered

#### Scenario: No matching locale available
- **WHEN** the OS locale is `"ja-JP"` and no `ja.yaml` exists
- **THEN** the active locale falls back to `"en"`

#### Scenario: Locale set once at startup
- **WHEN** the app starts
- **THEN** `i18n::init()` is called exactly once before any view renders

### Requirement: Untranslated keys fall back to the key string
The translation system SHALL return the key name itself (e.g. `"sidebar.all_titles"`) if a key is missing from the active locale's catalog, rather than panicking.

#### Scenario: Missing key at runtime
- **WHEN** a `t!("some.missing_key")` call is made and the key is absent from the active locale file
- **THEN** the rendered string is `"some.missing_key"` (the key itself), not an empty string or panic
