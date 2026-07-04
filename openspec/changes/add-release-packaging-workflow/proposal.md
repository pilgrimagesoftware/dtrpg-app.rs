## Why

The app has no cross-platform release process. `build.yaml` compiles and tests on Linux and macOS on every push to `develop`/`master`, but it packages nothing (`cargo package` only produces a `.crate`, not an installable app), never builds Windows, and auto-bumps a patch version on every push instead of following Conventional Commits. There is no nightly channel and no GitHub Release with downloadable binaries for any platform.

## What Changes

- Add a new GitHub Actions workflow (`release.yaml`) that builds installable packages for macOS (`.dmg`), Linux (`.AppImage` and/or `.deb`), and Windows (`.msi` or `.zip`) from the `dtrpg-app/rust` gpui application.
- Add a Windows build target (`x86_64-pc-windows-msvc`) to the build matrix; the app currently only builds for Linux and macOS.
- Nightly builds: on every push to `develop` (or on a schedule if `develop` is unchanged), build all three platform packages and publish/update a `nightly` pre-release on GitHub with the built artifacts, replacing the previous nightly assets.
- Tagged releases: when `develop` is merged into `master`, compute the next semantic version from Conventional Commits since the last tag (major/minor/patch from `feat`/`fix`/`BREAKING CHANGE`), create and push that tag, then build all three platform packages and publish a GitHub Release with the packaged artifacts attached.
- **BREAKING**: Replace the existing patch-only auto-bump-and-commit step in `build.yaml` with Conventional-Commits-driven versioning for tagged releases; `build.yaml` no longer commits version bumps back to the branch on every push.
- Retire or narrow the existing `bump-version.yaml` manual workflow now that tagging is automated from commit history for `master`; keep it only if manual out-of-band version bumps are still desired.

## Capabilities

### New Capabilities
- `release-packaging`: Building signed/unsigned installable packages for macOS, Linux, and Windows from the Rust gpui app, and uploading them to GitHub Releases (both nightly pre-releases from `develop` and tagged releases from `master`).
- `conventional-commit-versioning`: Deriving the next semantic version from Conventional Commits history when `develop` merges to `master`, and tagging the resulting commit accordingly.

### Modified Capabilities
(none — no existing `openspec/specs/` capability governs CI/CD packaging or release behavior)

## Impact

- Affected files: `.github/workflows/build.yaml` (scope narrowed to CI build/test only), new `.github/workflows/release.yaml`, possibly `.github/workflows/bump-version.yaml` (retired or scoped down).
- New build dependencies: a packaging tool per platform (e.g. `cargo-bundle` or `cargo-packager` for `.dmg`/`.msi`/`.AppImage`), plus a Windows runner in CI.
- New repository state: a `nightly` tag/release that gets force-updated, and semver git tags on `master`.
- No changes to application source code behavior; this is CI/CD and packaging infrastructure only.
