## Why

The settings panel contains several controls that are visually present but non-functional: the extension badge in File Openers is static text, the Add button inserts a hardcoded stub, and the Storage path is a read-only div. Users cannot actually configure their settings — the panel looks interactive but is not.

## What Changes

- **File Openers — extension field**: The `.ext` badge becomes an editable text input so users can type a file extension when adding or correcting an entry.
- **File Openers — Add button**: Opens a native macOS open-file dialog (via `rfd::AsyncFileDialog`) scoped to `.app` bundles so the user can select an application, then prompts for the extension; the resulting `FileOpenerEntry` is added to the list.
- **File Openers — Remove button**: Already wired to `ctrl.remove_file_opener()` — verify the `SettingsChanged` emission propagates correctly so the UI re-renders.
- **Storage — path field**: The static path div becomes an editable text input backed by a transient local string; value is not persisted until `catalog-storage-location` is connected, but the field becomes typeable.
- **Storage — warning text**: Applies a warning color (`hsla(0.08, 0.9, 0.55, 1.0)`, the same amber used for stale app warnings) and prepends a `⚠` symbol to the "files will not be moved" notice.

## Capabilities

### New Capabilities

- `settings-file-opener-add-dialog`: Opening the Add button in File Openers launches a native application-picker dialog; the selected app path and a user-supplied extension are combined into a `FileOpenerEntry` and persisted.
- `settings-editable-fields`: Text input elements in the settings panel (extension field, storage path) accept keyboard input and reflect changes immediately in the UI.
- `settings-warning-style`: The storage location warning text renders in an amber warning color with a warning symbol prefix.

### Modified Capabilities

## Impact

- **`dtrpg-ui/src/ui/views/settings_file_openers_view.rs`**: Replace static extension badge with a text input; replace stub Add handler with `rfd::AsyncFileDialog` flow.
- **`dtrpg-ui/src/ui/views/settings_storage_view.rs`**: Replace static path div with a text input; apply warning color and `⚠` prefix to the warning line.
- **`dtrpg-ui/Cargo.toml`**: Add `rfd` dependency (async feature) for the native file dialog.
- **`dtrpg-ui/src/controllers/settings.rs`**: May need a `set_extension()` / `update_extension()` method to support in-place extension edits on existing entries.
- No new data-model changes: `FileOpenerConfig` already has `add()` and `remove()`.
