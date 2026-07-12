## 1. Persistence

- [x] 1.1 In `data/ui_prefs.rs`, define `WindowBoundsPref { x: f32, y: f32, width: f32, height: f32 }` deriving `Serialize`/`Deserialize`/`Clone`/`Copy`
- [x] 1.2 Add `library_window_bounds: Option<WindowBoundsPref>` to `UiPrefsFile`
- [x] 1.3 Add `UiPrefs::library_window_bounds() -> Option<WindowBoundsPref>` and `UiPrefs::save_library_window_bounds(bounds: WindowBoundsPref)`, following the existing getter/setter pattern (e.g. `theme_key`/`save_theme_key`)

## 2. Restore on launch

- [x] 2.1 In `ui/app/mod.rs`'s `open_library_window`, before constructing `WindowOptions`, load `UiPrefs` and resolve the persisted `WindowBoundsPref` (if any) into a `Bounds<Pixels>`
- [x] 2.2 Validate the resolved bounds against `cx.displays()`: keep it only if `bounds.intersects(&display.bounds())` for at least one connected display; otherwise treat it as absent
- [x] 2.3 Set `WindowOptions.window_bounds` to `Some(WindowBounds::Windowed(bounds))` when valid saved bounds exist; leave unset (current default behavior) otherwise

## 3. Save on close

- [x] 3.1 In `open_library_window`, register a `window.on_window_should_close` hook (mirroring `open_settings_window`'s existing pattern) that reads `window.bounds()`, converts it to `WindowBoundsPref`, and calls `UiPrefs::load().save_library_window_bounds(...)`
- [x] 3.2 Confirm the hook returns `true` (allow the close to proceed) after saving, and doesn't interfere with any other should-close behavior for the library window

## 4. Verification

- [x] 4.1 Run `cargo check --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo +nightly fmt -- --check`, `cargo test --all-features --workspace`
- [ ] 4.2 Manually verify: resize and move the library window, quit, relaunch; confirm it reopens at the same position and size
- [ ] 4.3 Manually verify: quit and relaunch without ever having moved the window (fresh prefs); confirm it opens at the same default placement as before this change
- [ ] 4.4 Manually verify (if a multi-monitor setup is available): move the window to a secondary display, disconnect that display, relaunch; confirm it falls back to the default placement instead of opening off-screen
- [ ] 4.5 Manually verify: opening Settings still centers that window at its fixed size, unaffected by the library window's saved bounds
