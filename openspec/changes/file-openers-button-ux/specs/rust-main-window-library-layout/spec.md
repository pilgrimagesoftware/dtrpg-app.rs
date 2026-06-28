## ADDED Requirements

### Requirement: File openers action buttons are icon-only with tooltips
The "Add" button in the File Openers header and the "Remove" button on each entry row SHALL display only an icon with no visible text label. Each button SHALL expose its action label as a tooltip that appears on hover. The "Add" button SHALL use a "+" icon with tooltip "Add file opener". The remove button on each entry SHALL use a "×" icon with tooltip "Remove".

#### Scenario: Add button shows icon and tooltip
- **WHEN** the user views the File Openers settings section
- **THEN** the Add button shows a "+" icon with no visible text, and hovering the button shows the tooltip "Add file opener"

#### Scenario: Remove button shows icon and tooltip
- **WHEN** the user views a file opener entry row
- **THEN** the remove button shows a "×" icon with no visible text, and hovering the button shows the tooltip "Remove"
