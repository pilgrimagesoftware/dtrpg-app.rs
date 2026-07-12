## ADDED Requirements

### Requirement: Each font role offers a curated list of named choices
The app SHALL define three independent font roles — body, value, and monospace — each with a curated, platform-appropriate list of named font choices. Font selection SHALL NOT accept freeform text entry.

#### Scenario: Body font choices are serif-leaning
- **WHEN** the user opens the body font picker
- **THEN** the offered choices include the current default (Hoefler Text) and other serif options

#### Scenario: Value font choices are sans-serif
- **WHEN** the user opens the value font picker
- **THEN** the offered choices include the default (Gotham) and other sans-serif options

#### Scenario: Monospace font choices are fixed-width
- **WHEN** the user opens the monospace font picker
- **THEN** the offered choices include the current default (Menlo) and other monospace options

### Requirement: Selected fonts apply to their respective roles app-wide
The selected body font SHALL apply to all body text via the app's default text style. The selected value font SHALL apply everywhere the value-font role is used (e.g. Advanced settings' "Cache details" values). The selected monospace font SHALL apply everywhere the monospace-font role is used (e.g. the masked API key hint).

#### Scenario: Body font applies to general text
- **WHEN** the user selects a non-default body font
- **THEN** general UI text (labels, descriptions, headings) throughout the app renders in that font

#### Scenario: Value font applies to cache detail values
- **WHEN** the user selects a non-default value font
- **THEN** the Advanced settings "Cache details" row values render in that font

#### Scenario: Monospace font applies to the API key hint
- **WHEN** the user selects a non-default monospace font
- **THEN** the masked API key hint in Account settings renders in that font

### Requirement: Font preferences degrade gracefully when unresolvable
If a persisted font selection does not match any option in that role's curated list (e.g. after an app update removes an option, or a preferences file is shared across platforms), the app SHALL fall back to that role's default font rather than failing or leaving the role unset.

#### Scenario: Unrecognized persisted font id falls back to default
- **WHEN** the app starts with a persisted font id that is not present in that role's curated list
- **THEN** that role's default font is applied instead, with no error shown to the user
