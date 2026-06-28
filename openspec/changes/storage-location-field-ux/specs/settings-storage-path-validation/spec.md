## ADDED Requirements

### Requirement: Storage path existence is checked in the background
The app SHALL check whether the configured catalog storage path exists on disk whenever the path is set or changed. The check SHALL run on a background thread and SHALL NOT block the UI. The result SHALL be reflected in the settings controller snapshot as `storage_path_exists: bool`.

#### Scenario: Path exists — no warning shown
- **WHEN** the background check completes and the configured path exists on disk
- **THEN** `storage_path_exists` is `true` and no warning is displayed in the storage settings section

#### Scenario: Path does not exist — warning shown
- **WHEN** the background check completes and the configured path does not exist on disk
- **THEN** `storage_path_exists` is `false` and a warning row is visible in the storage settings section

#### Scenario: Check runs on initial load
- **WHEN** the app starts and the settings controller initializes
- **THEN** a background existence check is performed for the current storage path before the first render completes

#### Scenario: Check re-runs after path change
- **WHEN** the user selects a new storage folder via the folder picker
- **THEN** a background existence check is performed for the newly selected path

### Requirement: Storage path warning is styled with theme warning colors
When the storage path does not exist, the warning row SHALL use the theme's `warning_text` color for text and SHOULD use the theme's `warning_bg` color for the row background. The warning SHALL include a warning icon and a short human-readable message.

#### Scenario: Warning row appearance
- **WHEN** the storage path does not exist
- **THEN** the warning row displays a warning icon followed by text colored with `warning_text`, with the row using `warning_bg` as its background

#### Scenario: Warning row hidden when path is valid
- **WHEN** the storage path exists on disk
- **THEN** the warning row is not rendered in the settings storage section
