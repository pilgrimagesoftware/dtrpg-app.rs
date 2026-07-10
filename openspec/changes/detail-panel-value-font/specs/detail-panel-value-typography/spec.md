## ADDED Requirements

### Requirement: Detail tab metadata values MUST render in the app's value font
The expanded detail tab's `DescriptionList`-based metadata values SHALL render in the app's dedicated value font (`VALUE_FONT`), not the default body font. Labels, section headers, and prose SHALL be unaffected.

#### Scenario: Entry-tier metadata values use the value font
- **WHEN** the user opens the expanded detail tab for a catalog entry
- **THEN** the System, Released, Format, File Size, Category, Pages, Added, and Updated values render in the value font, while their labels render in the default body font

#### Scenario: Item-tier and disclosure values use the value font
- **WHEN** the user views a multi-item entry's item list, or expands the file-detail or "Other details" disclosure sections
- **THEN** every value shown (file name, format, file size, file id, download location, stable id, numeric id, order product id, product id, cover color) renders in the value font
