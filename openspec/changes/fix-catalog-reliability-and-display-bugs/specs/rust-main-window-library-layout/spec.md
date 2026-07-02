## MODIFIED Requirements

### Requirement: List view header cells match data cell alignment
The catalog list view's column header text SHALL be vertically centered within its header cell, matching the vertical centering already used by every data row cell.

#### Scenario: Header text is vertically centered
- **WHEN** the catalog is displayed in the ungrouped list presentation
- **THEN** each column header's text is vertically centered in its cell rather than aligned to the top

### Requirement: Detail view omits fields the API cannot populate
The detail view's metadata table SHALL omit the "Pages" row entirely when the item's page count is zero, rather than displaying a misleading "0". Other metadata fields that can be an explicit empty value (such as "System") SHALL display an em dash instead of blank text when empty.

#### Scenario: Zero page count omits the row
- **WHEN** an item's page count is `0` (the API did not report one)
- **THEN** the "Pages" row does not appear in the detail view's metadata table

#### Scenario: Non-zero page count shows the row
- **WHEN** an item's page count is greater than `0`
- **THEN** the "Pages" row appears with that count

#### Scenario: Empty system/line value shows an em dash
- **WHEN** an item's game line/system value is empty or whitespace-only
- **THEN** the "System" row displays an em dash ("—") rather than a blank value
