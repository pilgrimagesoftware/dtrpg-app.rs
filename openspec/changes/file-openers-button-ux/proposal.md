## Why

The File Openers settings section uses full text labels ("Add", "Remove") for its action buttons. These are inconsistent with the rest of the settings UI and take up unnecessary space. The remove action is also destructive with no confirmation, which risks accidental data loss.

## What Changes

- Replace the "Add" button label with a "+" icon; add a tooltip "Add file opener"
- Replace each "Remove" button label with a "×" icon; add a tooltip "Remove"
- Wrap each remove button with a confirmation `AlertDialog` that asks the user to confirm before the entry is deleted

## Capabilities

### New Capabilities

- `settings-file-openers-remove-confirm`: Confirmation dialog before removing a file opener entry

### Modified Capabilities

- `rust-main-window-library-layout`: File openers section action buttons change from text labels to icon-only with tooltips

## Impact

- `settings_file_openers_view.rs`: `render_add_button` gains a tooltip and switches to a "+" icon; `render_entry_row` wraps the remove button with `AlertDialog`; render functions gain `cx: &App` parameter to construct the dialog
- `settings_view.rs`: `render_file_openers_section` call site gains `cx` argument
- No controller or data model changes
