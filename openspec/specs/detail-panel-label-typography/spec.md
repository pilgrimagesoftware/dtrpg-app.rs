# detail-panel-label-typography Specification

## Purpose
TBD - created by archiving change detail-panel-value-font. Update Purpose after archive.
## Requirements
### Requirement: Detail tab metadata labels MUST render in the app's value font
The expanded detail tab's `DescriptionList`-based metadata labels SHALL render in the app's dedicated value-font role, not the default body font. Values, section headers, and prose SHALL be unaffected.

#### Scenario: Entry-tier metadata labels use the value font
- **WHEN** the user opens the expanded detail tab for a catalog entry
- **THEN** the System, Released, Format, File Size, Category, Pages, Added, and Updated labels render in the value font, while their values render in the default body font

#### Scenario: Item-tier and disclosure labels use the value font
- **WHEN** the user views a multi-item entry's item list, or expands the file-detail or "Other details" disclosure sections
- **THEN** every label shown (file name, format, file size, file id, download location, stable id, numeric id, order product id, product id, cover color) renders in the value font, while its corresponding value renders in the default body font
