## ADDED Requirements

### Requirement: Collection create/delete activity labels are localized
The Activity Panel labels shown while creating or deleting a collection SHALL be produced via
`t!()` with locale-specific text, not hardcoded English.

#### Scenario: Creating a collection under a non-English locale
- **WHEN** the active locale is `de` or `fr` and the user creates a collection named "Foo"
- **THEN** the Activity Panel entry text renders in that locale, not the hardcoded English
  `"Creating collection 'Foo'..."`

#### Scenario: Deleting a collection under a non-English locale
- **WHEN** the active locale is `de` or `fr` and the user deletes a collection
- **THEN** the Activity Panel entry text renders in that locale, not the hardcoded English
  `"Deleting collection…"`

### Requirement: Thumbnail loading activity labels are localized
The Activity Panel labels shown while loading thumbnails (both the initial label and the
"(N remaining)" progress variant) SHALL be produced via `t!()`.

#### Scenario: Thumbnail batch starts under a non-English locale
- **WHEN** the active locale is `de` or `fr` and a thumbnail load batch begins
- **THEN** the Activity Panel entry text renders in that locale, not the hardcoded English
  `"Loading thumbnails…"`

#### Scenario: Thumbnail batch progresses under a non-English locale
- **WHEN** the active locale is `de` or `fr` and a thumbnail load batch has items remaining
- **THEN** the Activity Panel entry text renders the localized "(N remaining)" variant, not
  the hardcoded English `"Loading thumbnails… (N remaining)"`

### Requirement: File download activity labels are localized
The Activity Panel label shown while downloading a file SHALL be produced via `t!()`,
covering both the single-file and multi-file-entry ("title — file_name") forms.

#### Scenario: Downloading a file under a non-English locale
- **WHEN** the active locale is `de` or `fr` and a file download starts
- **THEN** the Activity Panel entry text renders in that locale, not the hardcoded English
  `"Downloading {title}..."` or `"Downloading {title} — {file_name}..."`

### Requirement: Post-sign-in session-setup failure message is localized
The error message shown when session setup fails immediately after a successful sign-in
SHALL have its fixed (non-interpolated) portion produced via `t!()`.

#### Scenario: Session setup fails under a non-English locale
- **WHEN** the active locale is `de` or `fr` and session setup fails after sign-in with
  service error text `E`
- **THEN** the displayed message's fixed prefix renders in that locale (interpolated error
  detail `E` is unaffected), not the hardcoded English `"Session setup failed after
  sign-in: {E}"`
