## 1. Broaden the theme-sync function

- [x] 1.1 In `data/theme.rs`, rename `apply_table_colors` to `apply_theme_colors` (or add the new field groups to it, keeping the name — either is fine as long as there's one function) and update its doc comment to describe the full scope, not just tables
- [x] 1.2 Add the base semantic field group: `background`, `foreground`, `border`, `muted`, `muted_foreground`, `ring`, `selection`, `caret`, `drag_border`, `drop_target`, `description_list_label`, `description_list_label_foreground` (both `.colors` and `.tokens` where the field exists on both)
- [x] 1.3 Add the button field group: `button`, `button_hover`, `button_active`, `button_foreground`, and the `button_{danger,info,primary,secondary,success,warning}{,_active,_foreground,_hover}` variants, per design.md's mapping table
- [x] 1.4 Add the input field group: `input`
- [x] 1.5 Add the popover field group: `popover`, `popover_foreground`
- [x] 1.6 Add the scrollbar field group: `scrollbar`, `scrollbar_thumb`, `scrollbar_thumb_hover`
- [x] 1.7 Add the sidebar field group: `sidebar`, `sidebar_accent`, `sidebar_accent_foreground`, `sidebar_border`, `sidebar_foreground`, `sidebar_primary`, `sidebar_primary_foreground`
- [x] 1.8 Add the base semantic-role fields backing the button/sidebar variants: `primary`, `primary_active`, `primary_foreground`, `primary_hover`, `secondary`, `secondary_active`, `secondary_foreground`, `secondary_hover`, `danger`, `danger_active`, `danger_foreground`, `danger_hover`, `warning`, `warning_active` (if present), `warning_foreground`, `warning_hover` (if present), `info`, `info_active`, `info_foreground`, `info_hover`, `success`, `success_active`, `success_foreground`, `success_hover`, `list_active` (per design.md's mapping table)
- [x] 1.9 Update the two call sites (`app::setup`, `LibraryController::set_theme`) to call the renamed/broadened function

## 2. Fix hardcoded warning literals

- [x] 2.1 In `settings_file_openers_view.rs:206`, replace `gpui::hsla(0.08, 0.9, 0.55, 1.0)` with `colors.warning_text`
- [x] 2.2 In `settings_storage_view.rs:168`, replace `gpui::hsla(0.08, 0.9, 0.55, 1.0)` with the `warning_text` local binding already in scope

## 3. Remove dead code

- [x] 3.1 Delete `ui/windows/app.rs` and its `pub mod app;` declaration in `ui/windows/mod.rs`
- [x] 3.2 Confirm nothing else references `AppWindow` (should already be true per the proposal's audit) and remove the now-empty `ui/windows/` module if `app.rs` was its only content

## 4. Verification

- [x] 4.1 Run `cargo check --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo +nightly fmt -- --check`, `cargo test --all-features --workspace`
- [ ] 4.2 Manually verify: switch through all themes (status-bar quick-switcher); confirm the sidebar background/border/active-item colors change with each switch
- [ ] 4.3 Manually verify: switch themes with the Settings window open; confirm default-styled buttons, text inputs, and dropdown menus/popovers update
- [ ] 4.4 Manually verify: trigger the missing-download-folder warning (or missing-file-opener warning); confirm its color follows the active theme instead of staying a fixed amber
- [ ] 4.5 Manually verify: switch to the Ink (dark) theme specifically; confirm no widget is left showing a light-mode default that clashes with the dark surface around it
