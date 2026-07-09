## Context

`dtrpg-app/rust` is a gpui desktop app targeted at macOS, Linux, and Windows (per `docs/git-repos.md`). Today `.github/workflows/build.yaml` only builds/tests on Linux (musl) and macOS, runs `cargo package` (produces a `.crate`, not an installable app), and auto-bumps `Cargo.toml` to the next patch version on every push to `develop`/`master`. There is no Windows build, no installable package for any platform, and no GitHub Release. `bump-version.yaml` offers a separate manual `workflow_dispatch` tag bump.

`develop` is the integration branch (Git Flow); `master` receives merges from `develop` for releases. Commit messages follow Conventional Commits (per `docs/git-repos.md`).

## Goals / Non-Goals

**Goals:**
- Produce an installable package per OS (macOS `.dmg`, Linux `.AppImage`/`.deb`, Windows `.msi`) from a single packaging job definition shared between nightly and tagged releases.
- On push to `develop`: build all three packages and publish them as a rolling `nightly` GitHub pre-release, replacing the previous nightly's assets.
- On push to `master` (i.e., `develop` merged in): compute the next semver from Conventional Commits since the last release tag, create and push that tag, build all three packages, and publish a GitHub Release with the packaged artifacts.
- Keep PR validation (`pr.yaml`) and day-to-day CI build/test (`build.yaml`) fast and separate from packaging.

**Non-Goals:**
- Code signing / notarization for macOS or Authenticode signing for Windows (tracked as a follow-up; unsigned artifacts ship with an OS-level warning on first run).
- Auto-updater integration inside the app.
- Publishing to app stores (Mac App Store, Microsoft Store) or Linux package repositories (apt/flatpak/snap).
- Changing `bump-version.yaml`'s manual dispatch mechanism beyond noting it's now redundant for `master`.

## Decisions

### Packaging tool: `cargo-packager`
Chose `cargo-packager` over `cargo-bundle` and hand-rolled scripts because it is the only actively maintained option that declaratively produces macOS `.dmg`, Windows `.msi`/NSIS, and Linux `.deb`/`.AppImage` from one `[package.metadata.packager]` block in `Cargo.toml`, without per-OS shell scripting. `cargo-bundle` only targets macOS/Linux and hasn't shipped Windows support. Hand-rolled scripts (manual `hdiutil`, WiX, `appimagetool` invocations) were rejected as higher-maintenance and harder to keep in sync across three OSes.

### Versioning: `mathieudutour/github-tag-action`
Chose this over the existing `anothrNick/github-tag-action` because it parses actual Conventional Commit prefixes (`fix:` → patch, `feat:` → minor, `BREAKING CHANGE:`/`!` → major) from the commit log since the last tag, rather than requiring `#major`/`#minor` hashtags in commit messages. It runs only on push to `master` and only creates one tag per merge. `bump-version.yaml`'s manual dispatch stays in place, unchanged, as an escape hatch for out-of-band bumps (e.g. hotfix major bumps); it is not part of this change's automated path.

Rejected: `release-please`. It's more capable (changelog PRs, release notes) but introduces a release-PR review step, which changes the existing "merge develop → master → done" workflow more than needed here.

### Workflow structure: one reusable `workflow_call` job, two trigger workflows
Factor the package-build-and-upload logic into a reusable workflow (`.github/workflows/package.yaml`) that takes `tag` (string) and `prerelease` (boolean) inputs, runs the OS matrix, and publishes/updates a GitHub Release for that tag. Two thin trigger workflows call it:
- `nightly.yaml`: `on: push` to `develop` (paths-filtered to source/Cargo files, same as `build.yaml`) → calls the reusable workflow with `tag: nightly`, `prerelease: true`.
- `release.yaml`: `on: push` to `master` → runs the version-bump job first, then calls the reusable workflow with the new tag and `prerelease: false`.

This avoids duplicating the OS build matrix and release-publish steps in two files.

### Release publishing: `ncipollo/release-action`
Chosen over `softprops/action-gh-release` for its explicit `allowUpdates`/`removeArtifacts` options, which make the "replace nightly's previous assets" behavior a one-line config rather than a manual `gh release delete-asset` loop.

### Windows keyring dependency
`docs/rust.md`/app `CLAUDE.md` require the Linux runner to install `libsecret-1-dev` (already true for `build.yaml`'s functional tests; must also apply to the Linux packaging job since `keyring` is a runtime dependency of the packaged binary). Windows uses the native Credential Manager backend and needs no extra system package. macOS uses the native Keychain backend and needs no extra system package.

### `build.yaml` scope change (BREAKING)
Remove the auto-bump-and-commit-Cargo.toml step from `build.yaml`. It exists today to keep `Cargo.toml`'s version field current, but conflicts with the new tag-driven versioning (which reads Conventional Commits, not the crate version) and causes redundant commits on every push. `build.yaml` becomes build+test only; version stamping happens exclusively in the `release.yaml` tag step, which then updates `Cargo.toml` and pushes it as part of the same job that creates the tag.

## Risks / Trade-offs

- **[Risk] gpui's Windows backend may be incomplete or unbuildable** (the pinned `gpui` git revision has historically had partial Windows support) → **Mitigation**: add the Windows matrix leg but mark it `continue-on-error: true` initially in `pr.yaml`/CI smoke checks; the packaging workflow itself should fail loudly if the Windows build breaks so it's visible, but this is called out as an explicit open question below rather than assumed to work.
- **[Risk] Unsigned installers trigger OS security warnings** (Gatekeeper on macOS, SmartScreen on Windows) → **Mitigation**: document this limitation in the release notes template; signing is out of scope for this change and tracked separately.
- **[Risk] Rolling `nightly` tag/release makes `git describe`/tag history noisy or conflicts with real tags** → **Mitigation**: use `ncipollo/release-action` with `tag: nightly` and `allowUpdates: true` against a lightweight (non-annotated, force-updated) tag; exclude `nightly` from the Conventional Commits version-scan range in the release job by always diffing against the latest *semver* tag, not the latest tag overall.
- **[Risk] Reusable workflow adds indirection that's harder to debug in the Actions UI** → **Mitigation**: keep the reusable workflow's job/step names descriptive and identical in structure to the removed inline steps, and document the split in the workflow file's header comment.
- **[Trade-off] No code signing means macOS/Windows users must manually bypass OS warnings** — acceptable for now given no existing signing certificates/infrastructure; revisit once the app has a stable enough audience to justify the cost.

## Migration Plan

1. Add `[package.metadata.packager]` configuration to `Cargo.toml` (or the relevant crate) for all three OS targets; verify locally with `cargo packager` where possible (macOS/Linux; Windows via CI only).
2. Add the Windows target to `rust-toolchain.toml`-driven CI matrices where needed (`x86_64-pc-windows-msvc`).
3. Introduce `.github/workflows/package.yaml` (reusable) and `.github/workflows/nightly.yaml`; validate nightly builds succeed on a feature branch via `workflow_dispatch` before merging to `develop`.
4. Introduce the version-bump job in `release.yaml` and validate it against a scratch tag on a test branch (never push to real `master` during validation).
5. Remove the auto-bump-and-commit step from `build.yaml` in the same change so there is never a window where both the old and new versioning mechanisms run.
6. Roll out by merging to `develop` first (exercises the nightly path for real), then merge `develop` → `master` once to exercise the tagged-release path end-to-end.

Rollback: revert the workflow-file changes; `build.yaml`'s build/test behavior is unaffected and can continue running standalone if `release.yaml`/`nightly.yaml`/`package.yaml` are reverted.

## Open Questions

- Does the pinned `gpui` revision build on `windows-latest` today? Needs a spike/`workflow_dispatch` trial before this is merged as a required (non-`continue-on-error`) leg.
- Should Linux ship both `.deb` and `.AppImage`, or just one? Defaulting to both since `cargo-packager` supports both from one config with negligible extra CI time.
- Do we want signing in a near-term follow-up, or is unsigned acceptable indefinitely for this app's distribution model (direct download vs. store)?
