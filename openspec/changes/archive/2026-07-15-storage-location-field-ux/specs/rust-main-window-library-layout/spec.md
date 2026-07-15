## ADDED Requirements

### Requirement: Storage settings action buttons are inline with the path field
The storage settings section SHALL display the "Change…" and "Show in Finder/Explorer/Files" action buttons to the right of the path display field, in the same horizontal row, rather than in a separate row below. Each button SHALL display only an icon; the text label SHALL be exposed as a tooltip. The icon for "Change…" SHALL be a folder-open or edit symbol; the icon for "Show in Finder/Explorer/Files" SHALL be a reveal or external-link symbol.

#### Scenario: Action buttons appear inline with path field
- **WHEN** the user opens the Storage settings section
- **THEN** the path display field and both action buttons appear in a single horizontal row, with the buttons to the right of the field

#### Scenario: Button labels visible as tooltips
- **WHEN** the user hovers over an action button in the storage settings section
- **THEN** a tooltip appears with the full text label ("Change…" or "Show in Finder", etc.)

#### Scenario: Icon-only button has no visible text label
- **WHEN** the storage settings section is rendered
- **THEN** each action button shows only an icon with no visible text label
