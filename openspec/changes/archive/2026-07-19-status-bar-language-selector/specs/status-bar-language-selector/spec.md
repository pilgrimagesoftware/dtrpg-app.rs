## ADDED Requirements

### Requirement: Status bar SHALL display a language picker showing the active locale
The status bar SHALL display a button, positioned next to the theme picker, labeled with the endonym (native name) of the currently active locale.

#### Scenario: Status bar shows the active locale's endonym
- **WHEN** the status bar renders with English active
- **THEN** the language picker button's label reads "English"

#### Scenario: Label updates after switching locale
- **WHEN** the user switches the active locale to French
- **THEN** the language picker button's label reads "Français" on the next render

### Requirement: Clicking the language picker SHALL open a menu of all supported locales
The language picker SHALL open a dropdown menu, on click, listing every supported locale by its endonym, with the currently active locale marked as checked.

#### Scenario: Dropdown lists all supported locales
- **WHEN** the user clicks the language picker
- **THEN** a dropdown menu opens listing English, Français, and Deutsch

#### Scenario: Active locale is marked checked in the dropdown
- **WHEN** the user opens the language picker while German is active
- **THEN** the "Deutsch" menu item shows a checked state and the others do not

### Requirement: Selecting a locale SHALL switch the app's displayed language immediately
Selecting a locale from the dropdown SHALL change the active locale and cause all visible translated text to reflect the new locale on the next render, without requiring an app restart.

#### Scenario: Selecting a new locale updates visible text without restart
- **WHEN** the user selects "Français" from the language picker's dropdown
- **THEN** translated UI text throughout the app updates to French on the next render, with no restart

### Requirement: The selected locale SHALL persist across app restarts
The app SHALL persist the user's selected locale and use it on the next launch in preference to OS-locale detection.

#### Scenario: Persisted locale is restored on relaunch
- **WHEN** the user selects "Deutsch" and then relaunches the app
- **THEN** the app starts with German active, regardless of the OS locale setting

#### Scenario: No persisted locale falls back to OS detection
- **WHEN** the user has never changed the locale
- **THEN** the app starts using OS-locale detection, unchanged from current behavior
