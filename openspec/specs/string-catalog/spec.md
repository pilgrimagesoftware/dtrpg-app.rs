# string-catalog Specification

## Purpose
TBD - created by archiving change i18n-localization. Update Purpose after archive.
## Requirements
### Requirement: All user-visible strings are keyed in locale YAML files
Every hardcoded user-facing string in `crates/dtrpg-ui/src/ui/views/` and the filter-label utility SHALL be replaced by a `t!("module.key")` call. The corresponding key SHALL exist in `en.yaml`.

#### Scenario: Sidebar navigation labels
- **WHEN** the sidebar renders
- **THEN** the labels "All Titles", "Recently Updated", "On This Device", "In the Cloud", "Publishers", and "Collections" are produced by `t!()` calls, not string literals

#### Scenario: Detail panel metadata labels
- **WHEN** the detail panel renders for any item
- **THEN** the field labels ("System", "Category", "Format", "Pages", "File size", "Released", "Status", "Added") are produced by `t!()` calls

#### Scenario: Catalog empty-state messages
- **WHEN** the catalog is empty or has no search matches
- **THEN** the empty-state messages are produced by `t!()` calls

#### Scenario: Action button labels
- **WHEN** any view renders buttons (Read, Download, Downloaded, Show in Finder, Load Thumbnail, Log Out, Sign In, etc.)
- **THEN** each label is produced by a `t!()` call

### Requirement: Non-English locale files are scaffolded
`fr.yaml` and `de.yaml` SHALL exist with the same keys as `en.yaml`. Their values SHALL be identical to the English values and marked with a header comment indicating they are untranslated stubs.

#### Scenario: Locale file completeness
- **WHEN** `fr.yaml` or `de.yaml` is loaded
- **THEN** every key present in `en.yaml` is also present in the non-English file (values may be English stubs)

### Requirement: String keys follow dot-notation module convention
All keys SHALL use `<module>.<snake_case_description>` format. Module names SHALL match the view or utility they belong to (e.g. `sidebar`, `toolbar`, `catalog`, `detail`, `settings`, `activity`).

#### Scenario: Key format validation
- **WHEN** a new `t!()` call is added
- **THEN** the key string contains exactly one dot separator and both segments are non-empty

