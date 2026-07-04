## ADDED Requirements

### Requirement: Sentry initializes only when compiled in and configured
The application SHALL initialize the Sentry client at startup if and only if the binary was
compiled with the `sentry` Cargo feature enabled AND the `DTRPG_SENTRY_DSN` environment variable
is set to a non-empty value. Any other combination SHALL result in Sentry remaining
uninitialized, with the rest of application startup proceeding unchanged.

#### Scenario: Feature enabled, DSN present
- **WHEN** the binary is compiled with `--features sentry` and `DTRPG_SENTRY_DSN` is set at
  process startup
- **THEN** the application initializes the Sentry client and attaches a `tracing` layer that
  forwards ERROR-level events as Sentry issues and WARN-level events as breadcrumbs

#### Scenario: Feature enabled, DSN absent
- **WHEN** the binary is compiled with `--features sentry` but `DTRPG_SENTRY_DSN` is not set (or
  is empty) at process startup
- **THEN** the application does not initialize the Sentry client and does not attempt any network
  connection to Sentry

#### Scenario: Feature not compiled in
- **WHEN** the binary is compiled without the `sentry` feature (the default for `cargo build`,
  `cargo test`, and `cargo run` with no explicit feature flags)
- **THEN** the Sentry crate's initialization code is not present in the compiled binary,
  regardless of any environment variables set at runtime

### Requirement: Startup logs Sentry status
The application SHALL log exactly one INFO-level message during startup describing whether
Sentry reporting is active, and if not, why.

#### Scenario: Sentry active
- **WHEN** Sentry initializes successfully
- **THEN** the startup log includes an INFO line stating Sentry reporting is active

#### Scenario: Sentry disabled due to missing DSN
- **WHEN** the `sentry` feature is compiled in but `DTRPG_SENTRY_DSN` is not set
- **THEN** the startup log includes an INFO line stating Sentry reporting is disabled because no
  DSN was configured

#### Scenario: Sentry disabled due to feature not compiled in
- **WHEN** the binary was built without the `sentry` feature
- **THEN** the startup log includes an INFO line stating Sentry reporting is disabled because the
  build does not include Sentry support

### Requirement: Optional Sentry environment and release overrides
The application SHALL read `DTRPG_SENTRY_ENVIRONMENT` and `DTRPG_SENTRY_RELEASE` as optional
overrides for the Sentry environment name and release identifier, defaulting to `"production"`
and the crate's compiled version (`CARGO_PKG_VERSION`) respectively when not set.

#### Scenario: Overrides not set
- **WHEN** Sentry initializes and neither `DTRPG_SENTRY_ENVIRONMENT` nor `DTRPG_SENTRY_RELEASE`
  is set
- **THEN** events are tagged with environment `"production"` and release equal to the compiled
  crate version

#### Scenario: Overrides set
- **WHEN** Sentry initializes and both `DTRPG_SENTRY_ENVIRONMENT` and `DTRPG_SENTRY_RELEASE` are
  set to non-empty values
- **THEN** events are tagged with the provided environment and release values instead of the
  defaults

### Requirement: CI release builds inject Sentry configuration, PR builds do not
The release build workflow SHALL compile the application with the `sentry` feature enabled and
inject `DTRPG_SENTRY_DSN` from a repository secret. The PR validation workflow SHALL NOT enable
the `sentry` feature or inject any Sentry secret.

#### Scenario: Release build workflow
- **WHEN** the release build workflow runs on a push to `master` or `develop`
- **THEN** it builds the application with `--features sentry` and the `SENTRY_DSN` repository
  secret available in the build environment

#### Scenario: PR validation workflow
- **WHEN** the PR validation workflow runs against a pull request
- **THEN** it builds and tests the application without the `sentry` feature and without any
  Sentry secret present, producing a binary equivalent to a plain source build
