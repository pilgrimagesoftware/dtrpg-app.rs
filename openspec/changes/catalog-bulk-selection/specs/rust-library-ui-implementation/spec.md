## ADDED Requirements

### Requirement: Toolbar exposes a selection mode toggle button
The Rust UI toolbar SHALL include a button that toggles catalog selection mode. The button SHALL reflect the current selection mode state (active vs. inactive) via its visual variant.

#### Scenario: Toggle button shown in toolbar
- **WHEN** the library screen is rendered
- **THEN** a selection mode toggle button is present in the toolbar alongside the existing layout and settings controls

#### Scenario: Toggle button reflects active state
- **WHEN** selection mode is active
- **THEN** the toggle button is rendered in its active/selected visual state

#### Scenario: Toggle button reflects inactive state
- **WHEN** selection mode is inactive
- **THEN** the toggle button is rendered in its default/ghost visual state

### Requirement: Toolbar shows a bulk-action bar when selection is non-empty
When selection mode is active and at least one item is selected, the Rust UI toolbar area SHALL display a bulk-action bar in place of (or below) the normal toolbar row. The bar SHALL contain: a selection count label, Select All, Deselect All, a pattern-match selection control, and the six bulk-action buttons (Download, Remove Download, Fetch Thumbnail, Add to Collection, Remove from Collection, Open).

#### Scenario: Bulk-action bar replaces or supplements toolbar in selection mode
- **WHEN** selection mode is active and the selection count is greater than zero
- **THEN** the bulk-action bar is visible and all six action buttons are present

#### Scenario: Bulk-action bar hidden when no items selected
- **WHEN** selection mode is active but the selection set is empty
- **THEN** the bulk-action bar is not visible (or renders as hidden)

### Requirement: Pattern-match selection control is accessible from the bulk-action bar
The bar SHALL include a pattern-match control (text input + field selector) that allows users to add items to the selection by matching on title, publisher, or system.

#### Scenario: Pattern-match control is present in bar
- **WHEN** the bulk-action bar is visible
- **THEN** a text input and a field-selector dropdown are present for pattern-match selection

#### Scenario: Invoking pattern-match from bar adds matching items
- **WHEN** the user enters text and selects a field in the pattern-match control
- **THEN** items matching the pattern in the chosen field are added to the selection set
