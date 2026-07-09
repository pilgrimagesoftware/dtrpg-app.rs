## ADDED Requirements

### Requirement: Version bump derived from Conventional Commits
When `develop` is merged into `master`, the CI system SHALL compute the next semantic version by inspecting Conventional Commit messages made since the last semantic version tag, and SHALL NOT require manual selection of the version bump type.

#### Scenario: Fix-only commits bump the patch version
- **WHEN** all commits since the last tag use the `fix:` prefix (and none use `feat:` or contain a breaking-change marker)
- **THEN** CI computes the next version as a patch increment over the last tag

#### Scenario: Feature commits bump the minor version
- **WHEN** at least one commit since the last tag uses the `feat:` prefix and none contain a breaking-change marker
- **THEN** CI computes the next version as a minor increment over the last tag

#### Scenario: Breaking-change commits bump the major version
- **WHEN** at least one commit since the last tag contains a `BREAKING CHANGE:` footer or a `!` after the type/scope (e.g. `feat!:`)
- **THEN** CI computes the next version as a major increment over the last tag

### Requirement: Tag creation and Cargo.toml sync on master
The CI system SHALL create and push a new git tag for the computed version on `master`, and SHALL update `Cargo.toml`'s workspace version to match that tag in the same automated commit.

#### Scenario: Merge to master creates a tag
- **WHEN** a merge commit lands on `master` (i.e. `develop` was merged in)
- **THEN** CI computes the next version, creates an annotated tag for it, and pushes the tag to the remote

#### Scenario: Cargo.toml version matches the new tag
- **WHEN** a new version tag is created on `master`
- **THEN** `Cargo.toml`'s `[workspace.package] version` field is updated to match the new tag and the change is committed to `master` as part of the same automated run

### Requirement: Version scan excludes the nightly tag
The version-computation step SHALL determine "the last tag" using the most recent semantic version tag, ignoring the rolling `nightly` pre-release tag.

#### Scenario: Nightly tag does not affect version computation
- **WHEN** the repository has both semantic version tags (e.g. `1.2.3`) and a rolling `nightly` tag
- **THEN** CI computes the next version relative to the latest semantic version tag, not relative to `nightly`
