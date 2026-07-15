# settings-editable-fields Specification

## Purpose
TBD - created by archiving change settings-view-interactive-controls. Update Purpose after archive.
## Requirements
### Requirement: Extension field in the Add flow accepts keyboard input
The extension input shown after picking an app in the File Openers Add flow SHALL accept free-text keyboard input. The user types without a leading dot; the system normalizes (trims whitespace, lower-cases) before creating the entry.

#### Scenario: User types a valid extension
- **WHEN** the user types "pdf" into the extension field and confirms
- **THEN** a `FileOpenerEntry` is created with `extension: "pdf"` (lower-cased, no leading dot)

#### Scenario: User types an extension with a leading dot
- **WHEN** the user types ".epub" into the extension field
- **THEN** the system strips the leading dot and creates the entry with `extension: "epub"`

#### Scenario: User submits an empty extension
- **WHEN** the user submits the extension field while it is empty or contains only whitespace
- **THEN** no entry is added and an inline validation message is displayed

### Requirement: Storage path field is manually editable
The storage path display in the Storage settings section SHALL render as an editable text input rather than a static label. The typed value is held in transient UI state but SHALL NOT be persisted until `catalog-storage-location` is connected.

#### Scenario: User edits the storage path text
- **WHEN** the user clicks the path field in the Storage section and types a new path
- **THEN** the field reflects the typed text immediately and retains the value while the settings panel remains open

#### Scenario: Storage path field loses focus without saving
- **WHEN** the user edits the path field and then closes the settings panel
- **THEN** the path reverts to the placeholder value on next open (no persistence side-effect)

