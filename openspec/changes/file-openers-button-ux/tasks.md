## 1. Add button — icon and tooltip

- [x] 1.1 In `render_add_button`, replace the child `div` containing `"Add"` with one containing `"+"` (keep font weight and color unchanged)
- [x] 1.2 Add a `use gpui_component::tooltip::Tooltip;` import if not already present in `settings_file_openers_view.rs`
- [x] 1.3 Add `.tooltip(|window, cx| Tooltip::new("Add file opener").build(window, cx))` to the "add-file-opener" div

## 2. Remove button — icon, tooltip, and confirmation dialog

- [x] 2.1 Add `use gpui_component::WindowExt` to the imports in `settings_file_openers_view.rs` (used imperative `window.open_alert_dialog` instead of declarative trigger pattern — no `&mut App` needed in render path)
- [x] 2.2 No render function signature changes required (used imperative API in `on_click`)
- [x] 2.3 In `render_entry_row`, replace the remove `div`'s child text `"Remove"` with `"×"` and add a tooltip `"Remove"`
- [x] 2.4 In the remove button's `on_click`, call `window.open_alert_dialog` with confirm mode; `on_ok` calls `remove_file_opener`

## 3. Call site update

- [x] 3.1 No `settings_view.rs` call site changes required (no signature change)

## 4. Verification

- [x] 4.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [x] 4.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 4.4 Launch the app; confirm Add button shows "+" with tooltip "Add file opener"
- [ ] 4.5 Confirm remove button on each entry shows "×" with tooltip "Remove"
- [ ] 4.6 Click remove; confirm a dialog appears identifying the entry; confirm cancelling leaves the entry intact; confirm confirming removes the entry
