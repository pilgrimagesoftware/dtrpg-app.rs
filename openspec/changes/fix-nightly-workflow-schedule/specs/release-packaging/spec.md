## MODIFIED Requirements

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
