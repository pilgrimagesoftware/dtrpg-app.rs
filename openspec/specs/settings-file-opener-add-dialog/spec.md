# settings-file-opener-add-dialog Specification

## Purpose
TBD - created by archiving change settings-view-interactive-controls. Update Purpose after archive.
## Requirements
### Requirement: File Opener Add button opens a native application-picker dialog
When the user clicks the Add button in the File Openers section, the system SHALL open a native macOS open-file dialog filtered to `.app` bundles, then prompt for a file extension, and add a `FileOpenerEntry` to the list on confirmation.

#### Scenario: User picks an app and types an extension
- **WHEN** the user clicks the Add button
- **THEN** a native file dialog appears filtered to Applications
- **WHEN** the user selects a `.app` bundle and confirms
- **THEN** the extension text field becomes active (or an inline prompt appears) so the user can type a file extension
- **WHEN** the user submits the extension
- **THEN** a new row appears in the File Openers list with the chosen extension and app name, and the entry is persisted to disk

#### Scenario: User cancels the app-picker dialog
- **WHEN** the user clicks the Add button and then dismisses the native file dialog without selecting a file
- **THEN** no new entry is added and the File Openers list is unchanged

#### Scenario: Duplicate extension is replaced
- **WHEN** the user completes the Add flow with an extension that already exists in the list
- **THEN** the existing entry for that extension is replaced by the new one (same behavior as `FileOpenerConfig::add()`)

