## 1. Packaging configuration

- [ ] 1.1 Add `cargo-packager` as a dev/build dependency and add `[package.metadata.packager]` config to `Cargo.toml` covering macOS (`.dmg`), Linux (`.deb` and `.AppImage`), and Windows (`.msi`) targets, including app name, identifier, icon, and version placeholders.
- [ ] 1.2 Verify `cargo packager` produces a valid package locally on macOS and Linux; document any gaps (e.g. missing icons) and add required assets under `assets/` if missing.
- [ ] 1.3 Add the `x86_64-pc-windows-msvc` Rust target to the toolchain setup used by CI (via `rustup target add` or the `dtolnay/rust-toolchain` action's `targets` input).

## 2. Windows build spike

- [ ] 2.1 Add a `workflow_dispatch`-only trial workflow (or temporary matrix leg) that builds the app on `windows-latest` with the pinned `gpui` revision, to confirm it compiles.
- [ ] 2.2 Resolve or document any Windows-specific build failures from `gpui` before making the Windows leg required in the packaging matrix; if unresolved, mark the leg `continue-on-error: true` and note this in the workflow file and design's Open Questions.

## 3. Reusable packaging workflow

- [ ] 3.1 Create `.github/workflows/package.yaml` as a `workflow_call` reusable workflow accepting `tag` (string) and `prerelease` (boolean) inputs.
- [ ] 3.2 Implement the OS build matrix (macOS, Linux, Windows) in the reusable workflow: checkout, toolchain setup, `Swatinem/rust-cache`, install `libsecret-1-dev` on the Linux leg, run `cargo packager --release`, upload each OS's package as a build artifact.
- [ ] 3.3 Add a final job that downloads all matrix artifacts and publishes/updates a GitHub Release for the given `tag` via `ncipollo/release-action`, using `allowUpdates: true` and `removeArtifacts: true` when `prerelease` is true (nightly), and creating a fresh release when `prerelease` is false (tagged release).

## 4. Nightly workflow

- [ ] 4.1 Create `.github/workflows/nightly.yaml` triggered on `push` to `develop` with the same `paths` filter as `build.yaml` (`crates/**`, `rust-toolchain.toml`, `Cargo.toml`), plus `workflow_dispatch` for manual runs.
- [ ] 4.2 Call the reusable `package.yaml` workflow with `tag: nightly`, `prerelease: true`.
- [ ] 4.3 Verify a full run on a feature branch (via `workflow_dispatch`) produces a `nightly` release with three platform assets attached, and that a second run replaces the first run's assets.

## 5. Versioning and tagged release workflow

- [ ] 5.1 Create `.github/workflows/release.yaml` triggered on `push` to `master`.
- [ ] 5.2 Add a `version` job using `mathieudutour/github-tag-action` scoped to the latest semantic version tag (excluding `nightly`), computing the next version from Conventional Commits since that tag.
- [ ] 5.3 Add a step that updates `Cargo.toml`'s `[workspace.package] version` to the computed version and commits/pushes that change to `master` (`[skip ci]`) before tagging, or as part of the same tag commit.
- [ ] 5.4 Call the reusable `package.yaml` workflow with the computed tag and `prerelease: false`, depending on the `version` job.
- [ ] 5.5 Validate the full flow end-to-end on a scratch branch/tag (never against real `master`) before merging this change.

## 6. Retire old versioning path

- [ ] 6.1 Remove the auto-bump-and-commit-`Cargo.toml` step and the `anothrNick/github-tag-action` step from `build.yaml`, leaving it as build/test only.
- [ ] 6.2 Decide whether to keep `bump-version.yaml` as a manual escape hatch or remove it; update its description/comments to clarify it is independent of the new automated tagging on `master`.
- [ ] 6.3 Update `.github/workflows/debug.yaml` if it depends on any removed steps or outputs.

## 7. Documentation and validation

- [ ] 7.1 Document the release process (nightly vs. tagged, where artifacts land, how versioning works) in the app's `README.md`.
- [ ] 7.2 Run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo build` locally to confirm no regressions from `Cargo.toml` changes.
- [ ] 7.3 Merge to `develop` first to exercise the nightly path in production, then merge `develop` to `master` once to exercise the tagged-release path, confirming both produce correct GitHub Releases with all three platform packages attached.
