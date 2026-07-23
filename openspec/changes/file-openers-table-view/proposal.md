## Why

The File Openers settings section (`settings_file_openers_view.rs`) renders its rows as hand-rolled
`div().flex()` rows instead of using `gpui-component`'s `Table`/`DataTable` primitives. This duplicates
column-alignment logic that the component library already solves and diverges from the pattern used
elsewhere in the app (e.g. `detail_panel_view.rs`'s item list `DataTable`).

## What Changes

- Replace the manual flex-row layout in `render_file_openers_section` with a `gpui-component` table:
  extension, application name, and status/actions as columns with a proper header row.
- Keep existing row-level behavior intact: stale-app warning indicator, remove button with confirmation
  dialog, and the pending "add" row with its inline extension input.
- Keep the empty-state message when there are no configured openers.
- No change to the underlying `FileOpenerEntry` data model, add/remove controller logic, or i18n keys.

## Capabilities

### New Capabilities
- `settings-file-openers-table-layout`: the File Openers settings section renders its entries as a table
  with aligned header and row columns, using `gpui-component` table primitives instead of a custom flex
  layout.

### Modified Capabilities
(none — `settings-file-opener-add-dialog` and `settings-file-openers-remove-confirm` describe interaction
behavior that is preserved unchanged, not layout)

## Impact

- `dtrpg-ui/src/ui/views/settings_file_openers_view.rs`: rewritten to build rows via `gpui-component`
  table components instead of `div().flex()`.
- No changes to `dtrpg-ui/src/data/file_openers.rs`, `dtrpg-ui/src/controllers/settings.rs`, or i18n files.
