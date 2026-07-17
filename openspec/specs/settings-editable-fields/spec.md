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

### Requirement: Max concurrent downloads field is directly editable and bounded 1-5
The "Max concurrent downloads" field in the Storage settings section SHALL render as an editable number input (not a display-only value with separate +/- buttons). The field SHALL reject, via clamping, any value outside 1-5 inclusive.

#### Scenario: User types a valid value directly
- **WHEN** the user clicks the "Max concurrent downloads" field and types "4"
- **THEN** the field accepts the typed digits and, once the value is committed, `max_concurrent_downloads` is set to 4

#### Scenario: User uses the field's built-in step controls
- **WHEN** the user clicks the field's increment or decrement control
- **THEN** the value adjusts by 1, matching the previous minus/value/plus button behavior

#### Scenario: Typed value above the maximum is clamped
- **WHEN** the user types a value greater than 5 and the field loses focus
- **THEN** the field's value is clamped to 5

#### Scenario: Typed value below the minimum is clamped
- **WHEN** the user types a value less than 1 and the field loses focus
- **THEN** the field's value is clamped to 1

