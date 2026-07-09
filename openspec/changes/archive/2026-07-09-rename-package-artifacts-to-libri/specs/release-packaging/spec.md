## MODIFIED Requirements

### Requirement: Cross-platform package build
The CI system SHALL build an installable application package for macOS, Linux, and Windows from a single source revision whenever a release build is triggered (nightly or tagged), and every produced artifact filename SHALL be based on the `libri` product name rather than the internal `dtrpg-core` Cargo package name.

#### Scenario: All three platforms build in one run
- **WHEN** a release build is triggered for a given commit
- **THEN** CI runs a build matrix producing a macOS `.dmg`, a Linux `.AppImage` and/or `.deb`, and a Windows `.msi` from that same commit

#### Scenario: A single platform build failure fails the run
- **WHEN** the Windows, macOS, or Linux packaging step fails
- **THEN** the overall release workflow run SHALL be marked failed and no GitHub Release SHALL be published or updated for that trigger

#### Scenario: Artifact filenames use the libri name on all platforms
- **WHEN** cargo-packager produces an artifact for any platform (macOS `.dmg`, Linux `.AppImage`/`.deb`, or Windows `.msi`/NSIS installer)
- **THEN** the artifact filename SHALL start with `libri` (or the macOS display equivalent `Libri` for the `.app`/`.dmg`, which is already correct), not `dtrpg-core`
