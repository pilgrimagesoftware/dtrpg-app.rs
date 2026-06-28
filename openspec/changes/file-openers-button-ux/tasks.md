## 1. Add button — icon and tooltip

- [ ] 1.1 In `render_add_button`, replace the child `div` containing `"Add"` with one containing `"+"` (keep font weight and color unchanged)
- [ ] 1.2 Add a `use gpui_component::tooltip::Tooltip;` import if not already present in `settings_file_openers_view.rs`
- [ ] 1.3 Add `.tooltip(|window, cx| Tooltip::new("Add file opener").build(window, cx))` to the "add-file-opener" div

## 2. Remove button — icon, tooltip, and confirmation dialog

- [ ] 2.1 Add `use gpui::App;` and `use gpui_component::dialog::AlertDialog;` to the imports in `settings_file_openers_view.rs`
- [ ] 2.2 Add `cx: &App` parameter to `render_file_openers_section` and `render_entry_row`; update the internal call from `render_file_openers_section` to `render_entry_row` to pass `cx`
- [ ] 2.3 In `render_entry_row`, replace the remove `div`'s child text `"Remove"` with `"×"` and add a tooltip `"Remove"`
- [ ] 2.4 Wrap the remove `div` with `AlertDialog::new(cx).confirm()` using `.trigger(remove_div)`, `.title("Remove file opener?")`, `.description(format!("Remove the .{extension} opener for {app_name}?"))`, and `.on_ok(move |_, _, cx| { entity_remove.update(cx, ...); true })`; move the `entity_remove.update` remove call into the `on_ok` handler and remove the standalone `on_click` from the remove div

## 3. Call site update

- [ ] 3.1 In `settings_view.rs`, pass `cx` to `render_file_openers_section` at the call site

## 4. Verification

- [ ] 4.1 Run `cargo check --all-targets` and confirm no compile errors
- [ ] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [ ] 4.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 4.4 Launch the app; confirm Add button shows "+" with tooltip "Add file opener"
- [ ] 4.5 Confirm remove button on each entry shows "×" with tooltip "Remove"
- [ ] 4.6 Click remove; confirm a dialog appears identifying the entry; confirm cancelling leaves the entry intact; confirm confirming removes the entry
