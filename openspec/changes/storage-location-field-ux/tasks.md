## 1. SettingsController — background path check

- [x] 1.1 Add `storage_path_exists: bool` field to `SettingsController` (initialized to `true`)
- [x] 1.2 Add `storage_path_exists: bool` field to `SettingsSnapshot`; populate it from the controller field in `snapshot()`
- [x] 1.3 Add private method `check_storage_path_exists(&mut self, path: PathBuf, cx: &mut Context<Self>)` that spawns a background task via `cx.spawn`, checks `path.exists()`, then calls `entity.update` to write `storage_path_exists` and `cx.notify()`
- [x] 1.4 Call `check_storage_path_exists` in `SettingsController::new()` with the initial storage root path
- [x] 1.5 Call `check_storage_path_exists` in `apply_storage_path()` after the path is applied successfully, passing the new path

## 2. Settings storage view — inline layout

- [x] 2.1 Restructure the path-display child in `render_storage_section` into a horizontal `flex` row: path display takes `flex_1().min_w_0()`, followed by two icon buttons as `flex_none()` squares (32×32 px)
- [x] 2.2 Replace the "Change…" text button with an icon-only `div` showing `"📂"` and a tooltip `"Change\u{2026}"`; preserve the existing `on_click` handler
- [x] 2.3 Replace the "Show in Finder/Explorer/Files" text button with an icon-only `div` showing `"↗"` and a tooltip using `platform_reveal_label()`; preserve the existing `on_click` handler
- [x] 2.4 Remove the old separate actions row (the `div().flex().gap(px(12.0))` block containing the two buttons)

## 3. Settings storage view — warning row

- [x] 3.1 Add `storage_path_exists: bool` parameter to `render_storage_section`
- [x] 3.2 After the path row, conditionally render a warning row when `!storage_path_exists`: a `div` with `warning_bg` background, `rounded(px(6.0))`, `px(px(10.0))`, `py(px(6.0))`, flex row, items-center, gap, containing a `"⚠"` glyph and warning text, both colored `warning_text`
- [x] 3.3 Update the call site in `settings_view.rs` to pass `storage_snap.storage_path_exists` to `render_storage_section`

## 4. Verification

- [x] 4.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [x] 4.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 4.4 Launch the app; confirm path field and icon buttons appear in one row; confirm tooltips appear on hover
- [ ] 4.5 Temporarily set the storage path to a non-existent directory; confirm the warning row appears with correct colors
- [ ] 4.6 Restore a valid storage path; confirm the warning row disappears
