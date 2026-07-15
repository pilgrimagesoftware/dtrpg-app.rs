## MODIFIED Requirements

### Requirement: Status bar count labels use pluralize

The status bar's library total and active-tab summary labels SHALL use `pluralize` for
their item counts, consistent with the toolbar count label.

#### Scenario: Library total is plural

- **WHEN** the library contains 128 items
- **THEN** the status bar's library summary reads "128 items \u{2022} {size}"

#### Scenario: Library total is singular

- **WHEN** the library contains exactly 1 item
- **THEN** the status bar's library summary reads "1 item \u{2022} {size}"

#### Scenario: Active tab count is plural

- **WHEN** the active catalog tab matches 42 items
- **THEN** the active-tab summary reads "{tab label} \u{2022} 42 items"
