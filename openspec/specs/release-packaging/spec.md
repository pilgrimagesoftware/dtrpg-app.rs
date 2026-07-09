# release-packaging Specification

## Purpose
TBD - created by archiving change add-release-packaging-workflow. Update Purpose after archive.
## Requirements
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

### Requirement: Nightly pre-release from develop
The CI system SHALL publish a rolling `nightly` GitHub pre-release built from the latest `develop` commit once per day, on a fixed schedule targeting midnight Pacific Time, with sufficient `contents: write` permission for the calling workflow to publish or update the release.

#### Scenario: Scheduled run publishes nightly
- **WHEN** the daily schedule fires
- **THEN** CI builds all platform packages from the latest `develop` commit and publishes/updates a GitHub pre-release tagged `nightly` with those packages attached

#### Scenario: Nightly replaces previous nightly assets
- **WHEN** a new nightly build completes successfully
- **THEN** the previous `nightly` release's package assets SHALL be removed and replaced by the new build's assets, and the release body SHALL reflect the source commit it was built from

#### Scenario: Calling workflow grants write permission to the packaging job
- **WHEN** `nightly.yaml` invokes the reusable `package.yaml` workflow
- **THEN** the job SHALL be granted `contents: write` permission so the `publish` job in `package.yaml` can create or update the GitHub Release, and the run SHALL NOT fail with a permissions-related `startup_failure`

#### Scenario: Schedule targets midnight Pacific Time
- **WHEN** the cron trigger is evaluated
- **THEN** it SHALL fire at a UTC time corresponding to midnight Pacific Time (accounting for the fact that GitHub Actions cron has no timezone support and may drift by up to one hour across the Pacific Daylight/Standard Time transition)

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

