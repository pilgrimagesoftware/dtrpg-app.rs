## ADDED Requirements

### Requirement: Settings MUST provide an Appearance page
The Settings window SHALL provide an "Appearance" page, reachable from the settings sidebar alongside Account, Downloads Location, File Openers, Advanced, and About, presenting a picker for each of the three font roles (body, value, monospace) and a picker for the active color theme.

#### Scenario: Opening the Appearance page
- **WHEN** the user selects "Appearance" in the settings sidebar
- **THEN** the page shows the current body font, value font, monospace font, and theme selections, each as a distinct picker control

### Requirement: Appearance selections apply immediately
Changing any font role or the theme in the Appearance page SHALL apply the change to the running app immediately, without requiring a restart or window reopen.

#### Scenario: Changing the body font
- **WHEN** the user selects a different body font in the Appearance page
- **THEN** text throughout the app immediately renders in the newly selected font

#### Scenario: Changing the theme from Settings
- **WHEN** the user selects a different theme in the Appearance page
- **THEN** the app's colors update immediately, the same as when the theme is changed from the status-bar quick-switcher

### Requirement: Appearance selections persist across restarts
The selected body font, value font, monospace font, and theme SHALL persist across app restarts.

#### Scenario: Reopening the app after changing a font
- **WHEN** the user selects a non-default value font, then quits and relaunches the app
- **THEN** the previously selected value font is applied on launch, without the user needing to reselect it

#### Scenario: Reopening the app after changing the theme
- **WHEN** the user selects a non-default theme, then quits and relaunches the app
- **THEN** the previously selected theme is applied on launch instead of resetting to the default theme

### Requirement: Status-bar theme quick-switcher stays in sync with Settings
The existing status-bar theme quick-switcher SHALL continue to function and SHALL reflect and drive the same persisted theme state as the Appearance page's theme picker.

#### Scenario: Changing theme from the status bar reflects in Settings
- **WHEN** the user changes the theme via the status-bar quick-switcher, then opens Settings > Appearance
- **THEN** the Appearance page's theme picker shows the theme selected from the status bar as active

#### Scenario: Changing theme from Settings reflects in the status bar
- **WHEN** the user changes the theme via the Appearance page, then opens the status-bar quick-switcher
- **THEN** the status-bar quick-switcher shows the theme selected in Settings as active
