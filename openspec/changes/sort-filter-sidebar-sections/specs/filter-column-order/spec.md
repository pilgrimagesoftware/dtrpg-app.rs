# Spec Delta

## Purpose

Gives the filters column a predictable order, for its sections and for the values in each section, that does not change while the user is choosing filters.

## ADDED Requirements

### Requirement: Sections appear in a fixed order
The filters column SHALL list its sections in this order: Availability, Type, System, Genre, Format, Location, Tags. A section with no values SHALL be left out without changing the order of the others.

#### Scenario: All sections have values
- **WHEN** every facet has at least one value
- **THEN** the sections appear as Availability, Type, System, Genre, Format, Location, Tags

#### Scenario: A section has no values
- **WHEN** the catalog has no genres
- **THEN** the Genre section is absent and the others keep their relative order

### Requirement: Values are ordered alphabetically
Within a section other than Availability, the system SHALL order values alphabetically, ignoring case, using the value text as the catalog holds it. Ties SHALL be broken by the exact text.

#### Scenario: Mixed case values
- **WHEN** a section holds "pathfinder", "Dungeon Crawl Classics", and "Alpha Edition"
- **THEN** they appear as "Alpha Edition", "Dungeon Crawl Classics", "pathfinder"

### Requirement: Availability keeps its natural order
The Availability section SHALL list Downloaded before Cloud, whichever has more items.

#### Scenario: More cloud items than downloaded
- **WHEN** the catalog has more Cloud items than Downloaded items
- **THEN** Downloaded is still listed first

### Requirement: Order does not change while the user interacts
A value SHALL keep its position when counts change because another filter was checked or cleared, and when the value itself is checked or cleared. The order SHALL be the same after every launch and every catalog load for the same catalog.

#### Scenario: Checking a value
- **WHEN** the user checks "PDF" in Format and the counts of other values change
- **THEN** no value in any section moves to a different position relative to the values that remain

#### Scenario: Restart
- **WHEN** the app is restarted with the same catalog and no filters
- **THEN** every section lists its values in the same order as before

### Requirement: Counts remain visible beside each value
Each value SHALL show the number of items it would match, as before. The count SHALL NOT affect the value's position.

#### Scenario: A count drops to a lower number than its neighbor
- **WHEN** a value's count falls below the count of the value after it
- **THEN** the values do not swap places
