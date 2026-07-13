## MODIFIED Requirements

### Requirement: App MUST support six named color themes
The app MUST implement six color themes — parchment, slate, sage, ink, moss, and rosewood — each defined as a complete set of semantic color tokens. All views MUST read colors from the active theme rather than using hardcoded values.

The token set for each theme MUST include at minimum: desktop background, window background, surface, surface-alt, hover, text-primary, text-secondary, text-tertiary, border, border-strong, accent, accent-soft, accent-on, shadow, and scrim.

#### Scenario: Parchment theme applies warm cream tones
- **WHEN** the active theme is parchment
- **THEN** the window background is a warm cream color, text is dark brown, and accent hue is in the warm red/terracotta range

#### Scenario: Slate theme applies cool gray tones
- **WHEN** the active theme is slate
- **THEN** the window background is a cool near-white, text is dark blue-gray, and accent hue is in the blue-gray range

#### Scenario: Sage theme applies muted green tones
- **WHEN** the active theme is sage
- **THEN** the window background is a muted green-tinted off-white, text is dark green-gray, and accent hue is in the sage green range

#### Scenario: Ink theme applies a dark inverted palette
- **WHEN** the active theme is ink
- **THEN** the window background is near-black, text is light cream, and accent hue is warm gold

#### Scenario: Moss theme applies a dark green palette
- **WHEN** the active theme is moss
- **THEN** the window background is a dark, cool forest green, text is light, and accent hue is in the muted gold-green range — a second dark option distinguished from ink by its green rather than warm-brown cast

#### Scenario: Rosewood theme applies a warm burgundy palette
- **WHEN** the active theme is rosewood
- **THEN** the window background is a warm, light burgundy-tinted tone, text is dark, and accent hue is a deep wine red — a second light option distinguished from parchment by its red rather than tan cast

### Requirement: Theme selection MUST persist across restarts
The active theme SHALL be saved to persistent app preferences whenever it changes, from either the status-bar quick-switcher or the Settings Appearance page, and SHALL be restored on the next app launch instead of always defaulting to parchment.

#### Scenario: Theme persists after changing and relaunching
- **WHEN** the user selects a non-default theme, then quits and relaunches the app
- **THEN** the app launches with the previously selected theme active, not parchment

#### Scenario: First launch defaults to parchment
- **WHEN** the app launches with no persisted theme preference (e.g. first launch, or a preferences file predating this requirement)
- **THEN** the app defaults to the parchment theme
