## 1. Bound Constant

- [x] 1.1 In `crates/dtrpg-ui/src/ui/views/settings_storage_view.rs`, lower
      `MAX_CONCURRENT_DOWNLOADS` from `10` to `5`; leave
      `MIN_CONCURRENT_DOWNLOADS` at `1`

## 2. Settings Controller

- [x] 2.1 In `crates/dtrpg-ui/src/controllers/settings.rs`, add a private
      `max_concurrent_downloads_input: Option<Entity<InputState>>` field to
      `SettingsController` (initialized to `None` in `new`), a public
      `max_concurrent_downloads_input: Option<Entity<InputState>>` field to
      `SettingsSnapshot`, a `set_max_concurrent_downloads_input(&mut self,
      input: Entity<InputState>)` setter, and populate the snapshot field in
      `snapshot()` — mirror `recently_updated_window_input`'s exact
      field/setter/snapshot shape field-for-field

## 3. Root View Wiring

- [x] 3.1 In `crates/dtrpg-ui/src/ui/views/root_view.rs`, create a
      `max_concurrent_downloads_input` `InputState` entity (default value
      from `StorageConfig::load().max_concurrent_downloads()`, `.min(f64::from(MIN_CONCURRENT_DOWNLOADS))`,
      `.max(f64::from(MAX_CONCURRENT_DOWNLOADS))`), subscribe to its
      `InputEvent::Change`, parse the text as `usize`, and on success call
      `SettingsController::set_max_concurrent_downloads` — mirror the
      `recently_updated_window_input` block immediately above it
      field-for-field
- [x] 3.2 Attach the created input to the controller via
      `settings.update(cx, |ctrl, _cx| ctrl.set_max_concurrent_downloads_input(...))`

## 4. Settings UI

- [x] 4.1 In `crates/dtrpg-ui/src/ui/views/settings_storage_view.rs`,
      replace `render_concurrency_stepper`'s hand-rolled minus/value/plus
      button trio with a `gpui_component::input::NumberInput` bound to the
      passed-in `Option<Entity<InputState>>`, falling back to a
      plain-text value when `None` (mirror
      `render_recently_updated_window_row`'s `Option` fallback pattern in
      `settings_advanced_view.rs`)
- [x] 4.2 Update `render_storage_section`'s signature to accept
      `max_concurrent_downloads_input: Option<Entity<InputState>>` and pass
      it through to the rebuilt stepper row
- [x] 4.3 Thread `max_concurrent_downloads_input` through
      `render_settings_panel` (`settings_view.rs`) and its call site in
      `settings_window_view.rs` (`snap.max_concurrent_downloads_input`),
      following the same threading already in place for
      `recently_updated_window_input`

## 5. Localization Cleanup

- [x] 5.1 Remove the now-unused
      `settings.max_concurrent_downloads_decrement_tooltip` and
      `settings.max_concurrent_downloads_increment_tooltip` keys from
      `crates/dtrpg-ui/i18n/en.yaml`, `de.yaml`, `fr.yaml` (title/note keys
      are unchanged and stay)

## 6. Build and Quality

- [x] 6.1 `cargo check --workspace`
- [x] 6.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 6.3 `cargo test --workspace`
- [x] 6.4 `cargo +nightly fmt --all -- --check`

## 7. Manual Verification

- [ ] 7.1 Confirm the "Max concurrent downloads" field on the Storage
      settings page is directly editable (typing a value) and its +/-
      buttons work, that it won't go below 1 or above 5, and that the
      change persists across an app restart
