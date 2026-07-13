# configurable-fonts Specification

## Purpose
TBD - created by archiving change settings-appearance-fonts. Update Purpose after archive.
## Requirements
### Requirement: Each font role offers every font installed on the user's system
The app SHALL define four independent font roles — body, value, label, and monospace — each selectable from every font family the user's system reports as installed (via the platform text system's font enumeration), presented through a searchable picker. Font selection SHALL NOT be limited to a fixed, curated list.

#### Scenario: Any installed font is selectable for a role
- **WHEN** the user opens a font role's picker
- **THEN** every font family installed on the user's system appears as a selectable option, not just a fixed subset

#### Scenario: The picker is searchable
- **WHEN** the user types into a font role's picker
- **THEN** the option list filters to font names matching the typed text

### Requirement: Selected fonts apply to their respective roles app-wide
The selected body font SHALL apply to all body text via the app's default text style. The selected value font SHALL apply everywhere the value-font role is used (e.g. detail-view field values, Advanced settings' "Cache details" values). The selected label font SHALL apply everywhere the label-font role is used (e.g. detail-view field labels, settings row labels, table headers). The selected monospace font SHALL apply everywhere the monospace-font role is used (e.g. the masked API key hint).

#### Scenario: Body font applies to general text
- **WHEN** the user selects a non-default body font
- **THEN** general UI text (labels, descriptions, headings) throughout the app renders in that font

#### Scenario: Value font applies to cache detail values
- **WHEN** the user selects a non-default value font
- **THEN** the Advanced settings "Cache details" row values render in that font

#### Scenario: Label font applies to field labels and table headers
- **WHEN** the user selects a non-default label font
- **THEN** detail-view field labels, settings row labels, and table column headers render in that font

#### Scenario: Monospace font applies to the API key hint
- **WHEN** the user selects a non-default monospace font
- **THEN** the masked API key hint in Account settings renders in that font

### Requirement: Font preferences degrade gracefully when unresolvable
If a persisted font selection does not name a font the user's system currently reports as installed (e.g. the font was uninstalled, or a preferences file is shared across systems), the app SHALL fall back to that role's default font rather than failing or leaving the role unset.

#### Scenario: Unrecognized persisted font name falls back to default
- **WHEN** the app starts with a persisted font name that is not present in the user's system font list
- **THEN** that role's default font is applied instead, with no error shown to the user
