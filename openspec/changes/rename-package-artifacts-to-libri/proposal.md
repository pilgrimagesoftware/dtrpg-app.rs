## Why

The packaged app displays as "Libri" on macOS (`Libri.app`, `Libri_0.0.6_aarch64.dmg`, driven by `product-name = "Libri"` in `crates/dtrpg-core/Cargo.toml`), but the Linux artifacts still carry the internal Rust crate name: `dtrpg-core_0.0.6_x86_64.AppImage` and `dtrpg-core_0.0.6_amd64.deb` (confirmed from CI logs of run `28836717587`). cargo-packager falls back to the Cargo package name (`dtrpg-core`) for the artifact filename whenever no explicit `name` is set in `[package.metadata.packager]` — `product-name` only controls the macOS `.app`/`.dmg` display name, not the filename base used for Linux packaging formats (and, by the same fallback, Windows NSIS/WiX once that leg is fixed). Users downloading a release see an inconsistent, internal-sounding filename on Linux next to a branded one on macOS.

## What Changes

- Add an explicit `name = "libri"` field to `[package.metadata.packager]` in `crates/dtrpg-core/Cargo.toml`, giving cargo-packager a lowercase, Debian-package-name-safe identifier so Linux (and future Windows) artifact filenames use `libri` instead of `dtrpg-core`.
- No change to the Cargo crate name itself (`dtrpg-core` stays as-is) or to `product-name` (`Libri`, already correct for macOS).

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `release-packaging`: the "Cross-platform package build" requirement (defined in `openspec/changes/add-release-packaging-workflow/specs/release-packaging/spec.md`, not yet archived) is amended to require that all platform artifact filenames use the `libri` name rather than the internal crate name.

## Impact

- Affected file: `dtrpg-app/rust/crates/dtrpg-core/Cargo.toml` only.
- No changes to `.github/workflows/package.yaml`, `nightly.yaml`, or any other workflow — the glob patterns there (`*.dmg`, `*.AppImage`, `*.deb`, `*.msi`, `*.exe`) already match any filename and require no update.
- Resulting artifact filenames change (e.g. `dtrpg-core_0.0.6_amd64.deb` -> `libri_0.0.6_amd64.deb`); no impact to already-published releases, only future builds.
