## 1. Packaging configuration

- [x] 1.1 Add `cargo-packager` as a dev/build dependency and add `[package.metadata.packager]` config to `Cargo.toml` covering macOS (`.dmg`), Linux (`.deb` and `.AppImage`), and Windows (`.msi`) targets, including app name, identifier, icon, and version placeholders.
- [x] 1.2 Verify `cargo packager` produces a valid package locally on macOS and Linux; document any gaps (e.g. missing icons) and add required assets under `assets/` if missing.
  - Verified locally on macOS: `cargo packager --release --package dtrpg-core -f dmg -f app` produced a working `DriveThruRPG.app` and `.dmg`. `icons` is left unset (no dedicated app icon exists yet, only the generic Lucide set under `assets/icons/`); cargo-packager falls back to a default icon with no build failure. Documented as a follow-up in the `Cargo.toml` config comment.
  - Not verified locally on Linux (no Linux environment available in this session); relies on CI (task 4.3/5.5) for first real validation.
- [x] 1.3 Add the `x86_64-pc-windows-msvc` Rust target to the toolchain setup used by CI (via `rustup target add` or the `dtolnay/rust-toolchain` action's `targets` input).

## 2. Windows build spike

- [x] 2.1 Add a `workflow_dispatch`-only trial workflow (or temporary matrix leg) that builds the app on `windows-latest` with the pinned `gpui` revision, to confirm it compiles.
- [x] 2.2 Resolve or document any Windows-specific build failures from `gpui` before making the Windows leg required in the packaging matrix; if unresolved, mark the leg `continue-on-error: true` and note this in the workflow file and design's Open Questions.
  - Not run yet (requires pushing to GitHub Actions, no reachable remote/auth in this session). Windows leg is `continue-on-error: true` in `package.yaml` pending a real run of `windows-spike.yaml`.

## 3. Reusable packaging workflow

- [x] 3.1 Create `.github/workflows/package.yaml` as a `workflow_call` reusable workflow accepting `tag` (string) and `prerelease` (boolean) inputs.
- [x] 3.2 Implement the OS build matrix (macOS, Linux, Windows) in the reusable workflow: checkout, toolchain setup, `Swatinem/rust-cache`, install `libsecret-1-dev` on the Linux leg, run `cargo packager --release`, upload each OS's package as a build artifact.
- [x] 3.3 Add a final job that downloads all matrix artifacts and publishes/updates a GitHub Release for the given `tag` via `ncipollo/release-action`, using `allowUpdates: true` and `removeArtifacts: true` when `prerelease` is true (nightly), and creating a fresh release when `prerelease` is false (tagged release).

## 4. Nightly workflow

- [x] 4.1 Create `.github/workflows/nightly.yaml` triggered on `push` to `develop` with the same `paths` filter as `build.yaml` (`crates/**`, `rust-toolchain.toml`, `Cargo.toml`), plus `workflow_dispatch` for manual runs.
- [x] 4.2 Call the reusable `package.yaml` workflow with `tag: nightly`, `prerelease: true`.
- [ ] 4.3 Verify a full run on a feature branch (via `workflow_dispatch`) produces a `nightly` release with three platform assets attached, and that a second run replaces the first run's assets.
  - Requires pushing this branch to GitHub and dispatching the workflow; not runnable from this local session (no authenticated `gh`/git remote access). Do this before merging to `develop`.

## 5. Versioning and tagged release workflow

- [x] 5.1 Create `.github/workflows/release.yaml` triggered on `push` to `master`.
- [x] 5.2 Add a `version` job using `mathieudutour/github-tag-action` scoped to the latest semantic version tag (excluding `nightly`), computing the next version from Conventional Commits since that tag.
- [x] 5.3 Add a step that updates `Cargo.toml`'s `[workspace.package] version` to the computed version and commits/pushes that change to `master` (`[skip ci]`) before tagging, or as part of the same tag commit.
  - Implemented as: dry-run version calc -> commit `Cargo.toml` update -> create and push the annotated tag on that same commit, so the tag always matches the committed version (avoids the tag pointing at a pre-bump commit).
- [x] 5.4 Call the reusable `package.yaml` workflow with the computed tag and `prerelease: false`, depending on the `version` job.
- [ ] 5.5 Validate the full flow end-to-end on a scratch branch/tag (never against real `master`) before merging this change.
  - Requires pushing to GitHub; not runnable from this local session. Do this before merging `develop` into `master`.

## 6. Retire old versioning path

- [x] 6.1 Remove the auto-bump-and-commit-`Cargo.toml` step and the `anothrNick/github-tag-action` step from `build.yaml`, leaving it as build/test only.
- [x] 6.2 Decide whether to keep `bump-version.yaml` as a manual escape hatch or remove it; update its description/comments to clarify it is independent of the new automated tagging on `master`.
  - Kept as a manual escape hatch; comment added clarifying it's independent of `release.yaml`'s automated tagging.
- [x] 6.3 Update `.github/workflows/debug.yaml` if it depends on any removed steps or outputs.
  - No dependency found; added a comment documenting that it's unaffected.

## 7. Documentation and validation

- [x] 7.1 Document the release process (nightly vs. tagged, where artifacts land, how versioning works) in the app's `README.md`.
- [x] 7.2 Run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo build` locally to confirm no regressions from `Cargo.toml` changes.
  - `cargo build --workspace` and `cargo clippy --workspace -- -D warnings` both pass. `cargo fmt --check` reports pre-existing formatting diffs in `crates/dtrpg-core/src/app/assets.rs` unrelated to this change (this change touches only `Cargo.toml` and `.github/workflows/*.yaml`, neither of which `cargo fmt` checks); not introduced by this change.
- [ ] 7.3 Merge to `develop` first to exercise the nightly path in production, then merge `develop` to `master` once to exercise the tagged-release path, confirming both produce correct GitHub Releases with all three platform packages attached.
  - Deliberately left for the user to perform: requires pushing to the real GitHub remote and merging to `master`, which is a production action outside this session's scope.
