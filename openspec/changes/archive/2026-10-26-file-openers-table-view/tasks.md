## 1. Table structure

- [ ] 1.1 Replace the outer `div().flex().flex_col()` row list in `render_file_openers_section` with
      `Table`/`TableHeader`/`TableBody` from `gpui_component::table`.
- [ ] 1.2 Add a `TableHead` row with Extension and Application column labels (new i18n keys as needed).

## 2. Row rendering

- [ ] 2.1 Convert `render_entry_row` to build a `TableRow` with `TableCell`s for extension badge,
      application name (+ inline stale-app warning), and the remove button.
- [ ] 2.2 Convert `render_pending_row` to a `TableRow` with the extension `Input` and app name in cells
      aligned to the same columns, preserving the Escape-to-cancel `on_key_down` handler and cancel
      button.
- [ ] 2.3 Verify cell padding/spacing via `TableCell`'s `Styled` impl matches current visual spacing
      (`px(8.0)`/`px(12.0)`).

## 3. Empty state and wiring

- [ ] 3.1 Keep `render_empty_state` rendered outside the table when there are no entries and no
      pending add.
- [ ] 3.2 Confirm the header row is only rendered when there's at least one entry or a pending add (no
      bare header over an empty table).

## 4. Verification

- [ ] 4.1 `cargo check --all-targets` and `cargo clippy --all-targets --all-features -- -D warnings` in
      `dtrpg-app/rust`.
- [ ] 4.2 Manually run the app's Settings > File Openers section: verify header alignment, add flow,
      stale-app warning, and remove confirmation all still work.
