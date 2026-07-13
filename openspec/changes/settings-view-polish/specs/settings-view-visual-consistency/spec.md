## ADDED Requirements

### Requirement: Account settings row values MUST be left-aligned
The Account settings section's Email and API Key rows SHALL left-align their values immediately after the label column, with the label itself right-aligned against the label column's right edge.

#### Scenario: Viewing the Email row
- **WHEN** the user views the Account settings section while signed in
- **THEN** the Email label is right-aligned within its label column and the email value is left-aligned on the same row

#### Scenario: Viewing the API Key row
- **WHEN** the user views the Account settings section while signed in with an API key configured
- **THEN** the API Key label is right-aligned within its label column and the masked key value is left-aligned on the same row

### Requirement: Downloads settings action buttons MUST use the shared icon set
The Downloads settings section's "Change…" and reveal-location buttons SHALL use icons from the app's shared `gpui-component` icon set rather than raw text/emoji glyphs. Each button SHALL retain its existing tooltip and click behavior.

#### Scenario: Change button uses a folder icon
- **WHEN** the user views the Downloads settings section
- **THEN** the "Change…" button shows a folder icon (not an emoji glyph) and its tooltip still reads the existing "Change…" tooltip text

#### Scenario: Reveal button uses an open-folder icon
- **WHEN** the user views the Downloads settings section
- **THEN** the reveal-location button shows an open-folder icon (not an emoji glyph) and its tooltip still reads the platform-appropriate "Show in Finder/Explorer/Files" text

### Requirement: About settings MUST show build information
The About settings section SHALL display, in addition to the existing app name/version/description, the git commit short hash, build date, and target platform of the running build.

#### Scenario: Viewing build information
- **WHEN** the user views the About settings section
- **THEN** the section shows the version, git commit short hash, build date, and target platform, each as a labeled value

#### Scenario: Git commit unavailable at build time
- **WHEN** the app was built without `git` available or outside a git checkout
- **THEN** the commit hash field shows a placeholder value (e.g. "unknown") rather than an empty or missing field

### Requirement: About settings values MUST be right-aligned
The About settings section's version and build information values SHALL right-align against their row's right edge, using `DescriptionList` in its borderless configuration.

#### Scenario: Viewing About section rows
- **WHEN** the user views the About settings section
- **THEN** each label/value row's value is right-aligned, and no borders are drawn around or between the rows
