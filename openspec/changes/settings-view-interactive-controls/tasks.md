## 1. Dependencies and Data Model

- [ ] 1.1 Add `rfd = { version = "0.15", features = ["async-std"] }` to `[workspace.dependencies]` in `dtrpg-app/rust/Cargo.toml` and `rfd = { workspace = true }` to `dtrpg-ui/Cargo.toml`
- [ ] 1.2 Add `PendingAdd { app_path: PathBuf, extension_draft: String }` struct to `controllers/settings.rs`
- [ ] 1.3 Add `pending_add: Option<PendingAdd>` and `storage_path_draft: String` fields to `SettingsController`; initialize `storage_path_draft` to the placeholder string
- [ ] 1.4 Add `begin_add(app_path: PathBuf, cx: &mut Context<Self>)`, `set_extension_draft(ext: String, cx: &mut Context<Self>)`, `confirm_add(cx: &mut Context<Self>)`, and `cancel_add(cx: &mut Context<Self>)` methods to `SettingsController`
- [ ] 1.5 Add `set_storage_path_draft(path: String, cx: &mut Context<Self>)` method to `SettingsController`
- [ ] 1.6 Extend `SettingsSnapshot` with `pending_add: Option<(PathBuf, String)>` and `storage_path_draft: String`; update `snapshot()` to populate them
- [ ] 1.7 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 2. File Openers — Add Flow

- [ ] 2.1 In `settings_file_openers_view.rs`, update `render_add_button` to spawn `rfd::AsyncFileDialog::new().add_filter("Applications", &["app"]).set_directory("/Applications").pick_file()` via `cx.spawn()`; on successful pick call `ctrl.begin_add(path, cx)`
- [ ] 2.2 In `render_file_openers_section`, check `snapshot.pending_add`; if `Some`, render the "enter extension" inline form instead of the entry list
- [ ] 2.3 Implement `render_pending_add_form(app_path, draft, entity, colors)` with: app name label, single-line extension text input, Confirm button (calls `ctrl.confirm_add()`), and Cancel button (calls `ctrl.cancel_add()`)
- [ ] 2.4 Wire the extension text input to update `ctrl.extension_draft` on each keystroke via `ctrl.set_extension_draft()`
- [ ] 2.5 In `confirm_add()`, normalize the draft (strip leading `.`, trim, lower-case), validate non-empty, call `self.file_openers.add(entry)`, call `self.file_openers.save(tab_name)`, clear `pending_add`, emit `SettingsChanged`
- [ ] 2.6 In `cancel_add()`, clear `pending_add` and emit `SettingsChanged` so the list view is restored
- [ ] 2.7 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 3. Storage Section — Editable Path and Warning Style

- [ ] 3.1 In `settings_storage_view.rs`, replace the static path `div` with a single-line text input (gpui-component `TextInput` or `div`-based key-capture) bound to `storage_path_draft` from `SettingsSnapshot`; each keystroke calls `ctrl.set_storage_path_draft()`
- [ ] 3.2 Change the warning line `.text_color(text_tertiary)` to `.text_color(gpui::hsla(0.08, 0.9, 0.55, 1.0))` and update the string literal to `"⚠ Changing the storage location will not move existing downloaded files."`
- [ ] 3.3 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 4. Verification

- [ ] 4.1 Build and run the app; open Settings → File Openers; click Add; confirm a native macOS open-file dialog appears
- [ ] 4.2 Select an `.app` bundle; confirm the extension entry form appears with the correct app name displayed
- [ ] 4.3 Type an extension and click Confirm; confirm the new entry appears in the list with correct extension and app name
- [ ] 4.4 Click Add again and dismiss the dialog without selecting a file; confirm no new entry appears
- [ ] 4.5 Add an entry with an extension that already exists; confirm the existing entry is replaced (not duplicated)
- [ ] 4.6 Click Remove on an entry; confirm the entry disappears from the list immediately
- [ ] 4.7 Open Settings → Storage; click the path field; type a new value; confirm the field accepts input
- [ ] 4.8 Close and reopen Settings; confirm the path field shows the placeholder (not the typed value, since persistence is deferred)
- [ ] 4.9 Confirm the warning line displays "⚠ Changing the storage location will not move existing downloaded files." in amber
