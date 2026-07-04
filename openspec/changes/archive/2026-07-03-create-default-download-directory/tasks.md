## 1. Storage config

- [x] 1.1 Add `StorageConfig::is_default() -> bool` (true iff `override_path.is_none()`)
- [x] 1.2 Add `StorageConfig::ensure_root_exists() -> std::io::Result<()>` calling `std::fs::create_dir_all(self.root_path())`
- [x] 1.3 Unit tests: `is_default` true/false, `ensure_root_exists` creates a missing directory and is idempotent on an existing one

## 2. Wiring

- [x] 2.1 In `SettingsController::new`, call `ensure_root_exists()` when `storage.is_default() && !storage.is_accessible()`, logging a warning on failure instead of panicking or silently dropping the error
- [x] 2.2 Leave `apply_storage_path` and user-override handling unchanged — missing override paths still surface the existing warning

## 3. Verify

- [x] 3.1 Run `cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, and `cargo test --workspace` — all pass (93 unit tests, 4 doctests)
- [x] 3.2 Delete `~/Downloads/dtrpg` (or the platform equivalent), launch the app, and confirm the folder is recreated and the Settings storage warning does not appear
