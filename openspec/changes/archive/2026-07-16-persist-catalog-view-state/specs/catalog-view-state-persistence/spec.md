## ADDED Requirements

### Requirement: Persisted view state fields
The app SHALL persist the following catalog view state values across sessions: sidebar filter, sort method, grouped flag, and catalog presentation mode. The search query SHALL NOT be persisted and SHALL always begin empty.

#### Scenario: Default state on first launch
- **WHEN** no catalog view preferences are stored
- **THEN** the catalog opens with: `SidebarFilter::AllTitles`, `SortMethod::Title`, ungrouped, `CatalogPresentation::Grid`, and an empty search query

#### Scenario: Persisted state restored on subsequent launch
- **WHEN** the user sets sort to Publisher, enables grouping, and switches to List view, then quits and relaunches
- **THEN** the catalog opens with Publisher sort, grouped by publisher, and List presentation on the next launch

#### Scenario: Search query always starts empty
- **WHEN** the user types a search query and quits the app
- **THEN** on next launch, the search field is empty and all items matching the restored filter/sort are shown

### Requirement: Save on each mutation
The app SHALL save catalog view preferences to disk immediately after any of the following mutations: sidebar filter change, sort method change, grouped toggle, presentation mode change. Saves SHALL be best-effort: I/O failures SHALL be logged at `WARN` level and silently ignored.

#### Scenario: Filter change is saved
- **WHEN** the user selects "On Device" in the sidebar
- **THEN** the preference `filter = "OnDevice"` is written to disk before the next user interaction

#### Scenario: Save failure is non-fatal
- **WHEN** the app config directory is not writable and the user changes the sort order
- **THEN** the sort change is applied in-session; the app logs a warning but continues normally

### Requirement: Publisher filter fallback
When restoring a `Publisher` filter, the app SHALL verify that the stored publisher name exists in the loaded library. If the publisher is not found, the app SHALL fall back to `AllTitles`.

#### Scenario: Known publisher restored
- **WHEN** the stored filter is `Publisher("Paizo")` and Paizo items are in the library
- **THEN** the sidebar opens with the Paizo publisher filter active

#### Scenario: Unknown publisher falls back to AllTitles
- **WHEN** the stored filter is `Publisher("Defunct Press")` and no items from that publisher are in the library
- **THEN** the sidebar filter is set to `AllTitles` silently (no error shown to the user)

### Requirement: Storage location
Catalog view preferences SHALL be stored in the same TOML config file used by existing app settings (`~/.config/dtrpg/app_config.toml` on macOS/Linux, `%APPDATA%\dtrpg\app_config.toml` on Windows), under a `[catalog_view]` section. The file SHALL remain valid TOML if the section is absent (treated as defaults).

#### Scenario: Config file with no catalog_view section is valid
- **WHEN** `app_config.toml` exists but contains no `[catalog_view]` section
- **THEN** the app loads without error and uses defaults for all catalog view state

#### Scenario: Unknown field values fall back to defaults
- **WHEN** `app_config.toml` contains `sort = "UnrecognizedValue"` in `[catalog_view]`
- **THEN** the sort method defaults to `Title` and a `WARN` log is emitted
