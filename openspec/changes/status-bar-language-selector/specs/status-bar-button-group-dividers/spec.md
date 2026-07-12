## MODIFIED Requirements

### Requirement: Status bar right-side controls are visually separated by dividers
The status bar's right-hand side SHALL display a vertical divider between the language picker and the theme picker, a vertical divider between the theme picker and the activity indicator, and another vertical divider between the activity indicator and the notifications button.

#### Scenario: Dividers appear between all four right-side controls
- **WHEN** the status bar renders
- **THEN** a vertical divider appears between the language picker and the theme picker, a second vertical divider appears between the theme picker and the activity indicator, and a third vertical divider appears between the activity indicator and the notifications button

#### Scenario: Right-side divider style matches the existing left-side divider
- **WHEN** the status bar renders both its left-side and right-side dividers
- **THEN** the right-side dividers use the same vertical separator component already used between the library total and active-tab summary on the left side
