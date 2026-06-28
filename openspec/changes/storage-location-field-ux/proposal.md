## Why

The storage location field in Settings shows the path in a wide display row with two text buttons below it in a separate row, wasting vertical space and visually disconnecting the actions from the field they operate on. There is also no feedback when the configured path does not exist on disk, which can silently cause download failures.

## What Changes

- Move the "Change…" and "Show in Finder/Explorer/Files" buttons from a separate row beneath the path field into an inline row to the right of the field
- Replace the button text labels with appropriate icons; move the current labels to tooltips
- Add a conditionally-visible warning row beneath the path field that appears when the configured path does not exist on disk
- The warning row contains a warning icon and warning-colored text
- Perform a background existence check on the storage path whenever the stored value changes; hide the warning when the check passes
- Also run the check on initial render so pre-existing invalid paths are flagged immediately

## Capabilities

### New Capabilities

- `settings-storage-path-validation`: Background existence check for the configured catalog storage path; warning display when the path is missing

### Modified Capabilities

- `rust-main-window-library-layout`: The settings panel storage section layout changes (inline action buttons, warning row added)

## Impact

- `settings_storage_view.rs`: Layout restructured; warning row added; render function gains access to a validity flag from the controller snapshot
- `controllers/settings.rs`: `SettingsSnapshot` gains a `storage_path_exists: bool` field; `SettingsController` performs a background path check after any path change and on initialization
- `data/theme.rs`: No changes — `warning_bg` and `warning_text` tokens already exist
