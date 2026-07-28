## ADDED Requirements

### Requirement: Max concurrent downloads field is directly editable and bounded 1-5
The "Max concurrent downloads" field in the Storage settings section SHALL render as an editable number input (not a display-only value with separate +/- buttons). The field SHALL reject, via clamping, any value outside 1-5 inclusive.

#### Scenario: User types a valid value directly
- **WHEN** the user clicks the "Max concurrent downloads" field and types "4"
- **THEN** the field accepts the typed digits and, once the value is committed, `max_concurrent_downloads` is set to 4

#### Scenario: User uses the field's built-in step controls
- **WHEN** the user clicks the field's increment or decrement control
- **THEN** the value adjusts by 1, matching the previous minus/value/plus button behavior

#### Scenario: Typed value above the maximum is clamped
- **WHEN** the user types a value greater than 5 and the field loses focus
- **THEN** the field's value is clamped to 5

#### Scenario: Typed value below the minimum is clamped
- **WHEN** the user types a value less than 1 and the field loses focus
- **THEN** the field's value is clamped to 1
