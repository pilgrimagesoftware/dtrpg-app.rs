## 1. Drop SettingsTab

- [x] 1.1 Remove `SettingsTab` enum and all its impls from `controllers/settings.rs`
- [x] 1.2 Remove `active_tab` field, `set_tab()`, `active_tab()`, and `open_to()` from `SettingsController`
- [x] 1.3 Remove `active_tab` from `SettingsSnapshot`
- [x] 1.4 Update `add_file_opener` and `remove_file_opener` to call `self.file_openers.save()` with no args
- [x] 1.5 Update `new()` to not restore the active tab from disk

## 2. Simplify Downstream Tab References

- [x] 2.1 In `data/notification.rs`: change `NoticeAction::OpenSettings(SettingsTab)` to `NoticeAction::OpenSettings` (unit variant, no parameter)
- [x] 2.2 In `controllers/auth_state.rs`: update both notice constructions to use `NoticeAction::OpenSettings`
- [x] 2.3 In `notification_banner_view.rs`: remove `tab` extraction; change the action handler to just `ctrl.open(cx)`
- [x] 2.4 In `toolbar_view.rs`: change `ctrl.open_to(SettingsTab::Account, cx)` to `ctrl.open(cx)`

## 3. Add Clone to AuthStateSnapshot

- [x] 3.1 Add `#[derive(Clone)]` to `AuthStateSnapshot` in `controllers/settings.rs` so it can be captured by `SettingItem::render` closures

## 4. Update FileOpenerConfig::save

- [x] 4.1 In `data/file_openers.rs`: simplify `save` to no arguments, remove `load_with_tab`, drop `active_settings_tab` from `AppConfigFile`

## 5. Rewrite settings_view.rs

- [x] 5.1 Add `gpui_component::setting::{Settings, SettingGroup, SettingItem, SettingPage}` import
- [x] 5.2 Remove `render_tab_strip` and `render_active_section` functions
- [x] 5.3 Change `render_settings_panel` signature to remove the `active_tab` parameter
- [x] 5.4 Build a `Settings::new("settings-panel")` with three `SettingPage` entries: "Account", "Storage", "File Openers"
- [x] 5.5 Wrap each existing section render function in a `SettingItem::render` closure, capturing owned clones of the required snapshot data
- [x] 5.6 Replace the tab strip + section content `div` inside the modal card with the `Settings` component
- [x] 5.7 Change the modal card width from `px(560.0)` to `px(720.0)`

## 6. Update root_view.rs

- [x] 6.1 Remove `settings_snap.active_tab` from the `render_settings_panel` call

## 7. Build and Verify

- [x] 7.1 Run `cargo check -p dtrpg-ui` — zero errors
- [x] 7.2 Run `cargo clippy -p dtrpg-ui -- -D warnings` — zero warnings
- [ ] 7.3 Launch the app; open Settings from toolbar; verify three sidebar pages are present and navigate correctly
- [ ] 7.4 Verify the search input in the settings sidebar filters items
- [ ] 7.5 Verify Account, Storage, and File Openers content renders correctly in each page
- [ ] 7.6 Verify Escape closes the settings panel
- [ ] 7.7 Verify the notification banner action button opens settings
