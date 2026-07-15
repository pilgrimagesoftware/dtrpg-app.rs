# count-pluralization Specification

## Purpose
TBD - created by archiving change pluralize-count-labels. Update Purpose after archive.
## Requirements
### Requirement: Pluralize utility function
The system SHALL provide a `pluralize(count: usize, singular: &str, plural: &str) -> String` function in `crates/dtrpg-ui/src/util/pluralize.rs` that returns `"{count} {singular}"` when `count == 1` and `"{count} {plural}"` otherwise. This function MUST be the single site for count-noun formatting so future i18n work has one replacement point.

#### Scenario: Singular form for count of one
- **WHEN** `pluralize(1, "item", "items")` is called
- **THEN** it returns `"1 item"`

#### Scenario: Plural form for count of zero
- **WHEN** `pluralize(0, "item", "items")` is called
- **THEN** it returns `"0 items"`

#### Scenario: Plural form for count greater than one
- **WHEN** `pluralize(42, "title", "titles")` is called
- **THEN** it returns `"42 titles"`

### Requirement: Toolbar count label uses pluralize
The toolbar count label SHALL use `pluralize` for all count nouns so that singular counts read correctly.

#### Scenario: Total items label is singular
- **WHEN** the catalog contains exactly 1 item and no search or publisher filter is active
- **THEN** the count label reads `"1 item"`

#### Scenario: Publisher items label is singular
- **WHEN** a publisher filter is active and that publisher has exactly 1 item
- **THEN** the count label reads `"1 publisher item, N total items"`

#### Scenario: Filtered label is singular
- **WHEN** a search term is active and exactly 1 item matches
- **THEN** the count label reads `"N items (1 filtered)"`

### Requirement: Sidebar section suffix counts use pluralize
Sidebar section suffix counts (Publishers section and Collections section child items) SHALL use `pluralize` for their item count nouns.

#### Scenario: Collections section shows singular count
- **WHEN** the Collections section suffix displays a count of 1
- **THEN** it reads `"1 collection"` not `"1 collections"`

#### Scenario: Publishers section shows singular count
- **WHEN** the Publishers section suffix displays a count of 1
- **THEN** it reads `"1 publisher"` not `"1 publishers"`

### Requirement: Sidebar footer title count uses pluralize
The sidebar footer total title count SHALL use `pluralize`.

#### Scenario: Footer shows singular title count
- **WHEN** the catalog contains exactly 1 title
- **THEN** the footer reads `"1 title"` not `"1 titles"`

### Requirement: Status bar count labels use pluralize
The status bar's library total and active-tab summary labels SHALL use `pluralize` for their item counts, consistent with the toolbar count label.

#### Scenario: Library total is plural
- **WHEN** the library contains 128 items
- **THEN** the status bar's library summary reads `"128 items • {size}"`

#### Scenario: Library total is singular
- **WHEN** the library contains exactly 1 item
- **THEN** the status bar's library summary reads `"1 item • {size}"`

#### Scenario: Active tab count is plural
- **WHEN** the active catalog tab matches 42 items
- **THEN** the active-tab summary reads `"{tab label} • 42 items"`

