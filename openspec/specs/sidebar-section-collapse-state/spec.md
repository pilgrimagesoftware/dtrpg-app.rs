# sidebar-section-collapse-state Specification

## Purpose
TBD - created by archiving change ui-layout-fixes. Update Purpose after archive.
## Requirements
### Requirement: Sidebar section collapsed state persists across restarts
The collapsed/expanded state of the Collections and Publishers sidebar sections SHALL be persisted to `ui_prefs.toml` whenever it changes and restored on the next launch.

#### Scenario: Collapsed state is saved when user collapses a section
- **WHEN** the user collapses the Collections or Publishers section in the sidebar
- **THEN** the new collapsed state is written to `ui_prefs.toml` immediately

#### Scenario: Collapsed state is restored on launch
- **WHEN** the app launches and `ui_prefs.toml` has a saved collapsed state for Collections or Publishers
- **THEN** the sidebar renders those sections in their saved state (collapsed or expanded)

#### Scenario: Default state is expanded when no preference is saved
- **WHEN** `ui_prefs.toml` has no entry for a section's collapsed state
- **THEN** the section defaults to expanded

