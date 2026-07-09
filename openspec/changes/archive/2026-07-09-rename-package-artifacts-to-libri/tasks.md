## 1. Fix artifact naming

- [x] 1.1 Add `name = "libri"` to `[package.metadata.packager]` in `crates/dtrpg-core/Cargo.toml`.

## 2. Verify

- [ ] 2.1 Trigger the packaging workflow manually (`gh workflow run nightly.yaml` or `workflow_dispatch`) and confirm the Linux job packages `libri_<version>_x86_64.AppImage` and `libri_<version>_amd64.deb` instead of `dtrpg-core_*`.
- [ ] 2.2 Confirm the macOS job still produces `Libri.app` / `Libri_<version>_aarch64.dmg` unchanged.
