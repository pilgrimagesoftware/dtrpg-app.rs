## ADDED Requirements

### Requirement: A "Create collections" storage setting controls symlink creation
The app SHALL provide a "Create collections" storage setting, defaulting to disabled, that a user can toggle in Settings alongside the existing storage location controls. The setting SHALL be persisted across app restarts.

#### Scenario: Setting defaults to disabled
- **WHEN** a user has never configured this setting
- **THEN** "Create collections" is off and no `collections/` symlinks are created on download

#### Scenario: Setting persists across restarts
- **WHEN** a user enables "Create collections" and restarts the app
- **THEN** the setting remains enabled

### Requirement: Completing a download creates a symlink per collection membership when enabled
When "Create collections" is enabled, the app SHALL create a symlink for each collection the downloaded item belongs to, at `{storage root}/collections/{sanitized collection name}/{filename}`, pointing at the real downloaded file under `{storage root}/items/{sanitized publisher}/{filename}`.

#### Scenario: Downloaded item belongs to one collection
- **WHEN** "Create collections" is enabled and a download completes for an item that belongs to exactly one collection
- **THEN** a symlink to the downloaded file is created at `{storage root}/collections/{that collection's sanitized name}/{filename}`

#### Scenario: Downloaded item belongs to multiple collections
- **WHEN** "Create collections" is enabled and a download completes for an item that belongs to more than one collection
- **THEN** a symlink is created under each collection's `collections/{collection name}/` directory, all pointing at the same downloaded file

#### Scenario: Downloaded item belongs to no collection
- **WHEN** "Create collections" is enabled and a download completes for an item that does not belong to any collection
- **THEN** no symlink is created for that download

#### Scenario: Setting is disabled
- **WHEN** "Create collections" is disabled and a download completes
- **THEN** no `collections/` symlink is created, regardless of the item's collection memberships

### Requirement: Symlink creation is best-effort and never fails the download
A failure to create a symlink (permissions, unsupported filesystem, missing OS privilege) SHALL be logged and SHALL NOT change the download's own success/failure outcome or the activity panel entry's state.

#### Scenario: Symlink creation fails
- **WHEN** the app attempts to create a `collections/` symlink and the operation fails
- **THEN** the failure is logged, the download's item status still updates to Downloaded, and the activity panel entry still shows the download as complete

#### Scenario: Symlink target already exists
- **WHEN** the app attempts to create a `collections/` symlink at a path that already exists (e.g. re-downloading a previously downloaded item)
- **THEN** symlink creation is skipped at that path without logging an error

### Requirement: Symlink creation uses the OS-native mechanism
The app SHALL create symlinks using each platform's native primitive: a Unix symbolic link on macOS and Linux, and a Windows file symbolic link on Windows.

#### Scenario: Created on macOS or Linux
- **WHEN** "Create collections" is enabled and the app runs on macOS or Linux
- **THEN** the created `collections/` entry is a Unix symbolic link resolvable by standard filesystem tools

#### Scenario: Created on Windows
- **WHEN** "Create collections" is enabled and the app runs on Windows
- **THEN** the created `collections/` entry is a Windows file symbolic link

### Requirement: Collection names are sanitized before use as a directory path
A collection name SHALL be sanitized before being joined into the `collections/` path so that a name containing a path separator cannot escape the storage root or be misinterpreted as an absolute path component.

#### Scenario: Collection name contains a path separator
- **WHEN** a collection's display name contains a `/` character
- **THEN** the symlink directory created under `collections/` replaces that character rather than creating a nested or absolute-escaping path
