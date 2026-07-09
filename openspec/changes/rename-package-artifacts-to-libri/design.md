## Context

cargo-packager reads `[package.metadata.packager]` from `Cargo.toml` into the same `Config` struct used by a standalone `Packager.toml`. That struct has two distinct name fields: `name` (used to derive package/artifact filenames, must be filesystem/package-manager safe) and `product-name` (used for display purposes such as the macOS `.app` bundle name and window title). Only `product-name` is currently set (`"Libri"`); `name` is unset and falls back to the Cargo package name, `dtrpg-core`, which is what produces `dtrpg-core_0.0.6_amd64.deb` etc.

## Goals / Non-Goals

**Goals:**
- Make all platform artifact filenames consistently branded as `libri`.

**Non-Goals:**
- Renaming the `dtrpg-core` Cargo crate (unrelated; the crate name and the packaged artifact name are independent by design in cargo-packager).
- Changing `product-name` or any user-visible in-app branding — already correct.
- Fixing the currently-broken Windows packaging leg (tracked separately; the same `name` fallback applies there but isn't verifiable until that leg builds).

## Decisions

- Use `name = "libri"` (lowercase) rather than `"Libri"`: Debian package names must be lowercase, and using the same value across all platforms keeps filenames consistent rather than special-casing per format.
- Set this in `crates/dtrpg-core/Cargo.toml` under the existing `[package.metadata.packager]` table rather than introducing a separate `Packager.toml` — keeps all packaging config in one place, matching the existing convention in this repo.

## Risks / Trade-offs

- [Risk] Any external tooling or documentation that references the old `dtrpg-core_*` filenames (e.g. download links, install scripts) will break. → Mitigation: none currently exist per repo search; this is a pre-1.0 nightly/dev artifact naming fix, not a change to a stable, linked-to filename.
