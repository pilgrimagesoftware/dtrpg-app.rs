## Context

`render_file_openers_section` in `settings_file_openers_view.rs` builds each row as a bare
`div().flex().items_center().gap(...)`, matching column widths and gaps by hand across the header,
pending row, and entry rows. `gpui-component` ships two table APIs already in use elsewhere in the app:

- `Table` / `TableHeader` / `TableBody` / `TableRow` / `TableHead` / `TableCell` — stateless, no
  delegate, caller supplies rows as children directly.
- `DataTable` + `TableDelegate` + `TableState` — virtualized, delegate-driven, used by
  `catalog_view.rs` and `detail_panel_view.rs` for large/dynamic lists.

## Goals / Non-Goals

**Goals:**
- Column-aligned header and rows for extension, application name, and status/actions.
- Preserve all existing row behavior: stale-app warning, remove button + confirmation dialog, pending
  add row with focused inline extension input and Escape-to-cancel, empty state.

**Non-Goals:**
- No virtualization — the list is a handful of user-configured entries, never large enough to need it.
- No change to `FileOpenerEntry`, controller methods, or i18n keys.
- No change to the add/remove interaction flows themselves (covered by
  `settings-file-opener-add-dialog` and `settings-file-openers-remove-confirm`).

## Decisions

- **Use the stateless `Table` API, not `DataTable`.** `DataTable` requires a `TableDelegate` +
  `TableState` entity and is built for virtualized, homogeneous rows. This list needs one row shape
  that's a live input (pending), others with a remove button + tooltip + conditional warning badge —
  heterogeneous, interactive content that's simpler to express as `TableRow` children than to route
  through a delegate's `render_cell`. The list is also always small (bounded by how many overrides a
  user configures), so virtualization buys nothing.
- **Columns**: Extension | Application | (empty header) actions. The stale-app warning renders inline
  in the Application cell (as today), not as a separate column, to avoid an extra header label for a
  rare state.
- **Pending row stays a `TableRow`** with the extension `Input` in the Extension cell and app name in
  the Application cell, keeping visual alignment with committed rows instead of the current
  visually-distinct pending row.

## Risks / Trade-offs

- [Table's default padding/border styling might not match the current hand-tuned row spacing] →
  Override cell padding via `TableCell`'s `Styled` impl to match existing `px(8.0)`/`px(12.0)` values;
  verify visually before merging.
- [Focus handling for the pending row's `on_key_down` Escape handler might behave differently nested
  inside `TableRow`] → `TableRow` doesn't intercept key events, so the existing handler attaches the
  same way; confirmed by reading `table.rs`'s `RenderOnce` impl (no key event handling in the row/cell
  types).
