## ADDED Requirements

### Requirement: Cross-platform package build
The CI system SHALL build an installable application package for macOS, Linux, and Windows from a single source revision whenever a release build is triggered (nightly or tagged).

#### Scenario: All three platforms build in one run
- **WHEN** a release build is triggered for a given commit
- **THEN** CI runs a build matrix producing a macOS `.dmg`, a Linux `.AppImage` and/or `.deb`, and a Windows `.msi` from that same commit

#### Scenario: A single platform build failure fails the run
- **WHEN** the Windows, macOS, or Linux packaging step fails
- **THEN** the overall release workflow run SHALL be marked failed and no GitHub Release SHALL be published or updated for that trigger

### Requirement: Nightly pre-release from develop
The CI system SHALL publish a rolling `nightly` GitHub pre-release built from the latest `develop` commit whenever `develop` is updated.

#### Scenario: Push to develop publishes nightly
- **WHEN** a commit is pushed to `develop` that touches application source or build configuration
- **THEN** CI builds all platform packages from that commit and publishes/updates a GitHub pre-release tagged `nightly` with those packages attached

#### Scenario: Nightly replaces previous nightly assets
- **WHEN** a new nightly build completes successfully
- **THEN** the previous `nightly` release's package assets SHALL be removed and replaced by the new build's assets, and the release body SHALL reflect the source commit it was built from

### Requirement: Tagged release from master
The CI system SHALL publish a GitHub Release with all platform packages attached whenever a new version tag is created on `master`.

#### Scenario: Tag creation triggers packaging and release
- **WHEN** a new semantic version tag is pushed to `master`
- **THEN** CI builds all platform packages from the tagged commit and publishes a GitHub Release for that tag with the packages attached as release assets

#### Scenario: Release is not a pre-release
- **WHEN** a tagged release is published from `master`
- **THEN** the GitHub Release SHALL be marked as a full release, not a pre-release

### Requirement: Linux keyring runtime dependency
The Linux packaging build SHALL install the `libsecret` development package before building, since the packaged binary depends on it at runtime for credential storage.

#### Scenario: Linux build installs libsecret
- **WHEN** the Linux leg of the packaging matrix runs
- **THEN** the CI job installs `libsecret-1-dev` (or the equivalent runtime package) before compiling the release binary
