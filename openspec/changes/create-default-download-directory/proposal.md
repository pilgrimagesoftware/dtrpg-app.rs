## Why

On a fresh install, `StorageConfig::root_path()` resolves to `~/Downloads/dtrpg` (or the
platform equivalent), but nothing ever creates that directory. `SettingsController::new`
only checks `is_accessible()` and logs a warning plus surfaces a "storage folder does not
exist" banner in Settings — the app never creates the folder it owns. The user has to
manually click "Reveal" in Settings (which lazily creates the directory as a side effect of
opening Finder/Explorer) before the warning goes away.

## What Changes

- `StorageConfig` gains `is_default() -> bool` (true when no user override path is
  configured) and `ensure_root_exists() -> std::io::Result<()>` (creates the resolved root
  directory and any missing parents).
- `SettingsController::new` now calls `ensure_root_exists()` when the resolved path is the
  platform default and not yet accessible, before computing `storage_unavailable`. A
  user-chosen override path that is missing is left untouched — that more likely indicates
  an unmounted external/network volume than a fresh install, and silently recreating an
  arbitrary user-chosen path could mask the real problem.
- No change to `apply_storage_path` (still requires the target directory to already exist
  when the user explicitly sets a custom location — see `storage-location-field-ux`).

## Capabilities

### New Capabilities

- `storage-auto-create`: The platform default download directory is created automatically
  on first launch (or whenever no storage override is configured) if it does not already
  exist, rather than surfacing a "does not exist" warning for a location the app itself owns.

### Modified Capabilities

_(none)_

## Impact

- `dtrpg-ui/src/data/storage.rs` — add `StorageConfig::is_default`, `ensure_root_exists`
- `dtrpg-ui/src/controllers/settings.rs` — call `ensure_root_exists` in `new()` for the
  default-path case
