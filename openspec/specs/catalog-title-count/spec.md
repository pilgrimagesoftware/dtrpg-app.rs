# catalog-title-count Specification

## Purpose
TBD - created by archiving change catalog-collections-improvements. Update Purpose after archive.
## Requirements
### Requirement: Catalog title displays total count when unfiltered
The catalog view title area SHALL display the total number of items in the catalog when no filter is active.

#### Scenario: Total count shown without filter
- **WHEN** the catalog is loaded and no filter is active
- **THEN** the catalog title area shows "N titles" where N is the total item count

#### Scenario: Count updates after reload
- **WHEN** the catalog is reloaded and the item count changes
- **THEN** the displayed count updates to reflect the new total

### Requirement: Catalog title displays filtered and total count when a filter is active
The catalog title area SHALL display both the matching item count and the total item count when a filter is active.

#### Scenario: Filtered count shown with active filter
- **WHEN** a publisher filter, collection filter, or search term is active
- **THEN** the catalog title area shows "M of N" where M is the number of visible items and N is the total item count

#### Scenario: Count returns to total when filter is cleared
- **WHEN** a filter is cleared and no filter remains active
- **THEN** the catalog title area returns to showing only the total count

