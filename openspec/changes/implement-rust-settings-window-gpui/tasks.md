## 1. Window opening and tracking

- [ ] 1.1 Add a settings-window-open function in `dtrpg-ui/src/ui/app/mod.rs` (parallel to `open_library_window`) that opens a `WindowOptions` window whose root is `render_settings_panel` (or a new thin root view wrapping it) inside `gpui_component::Root`.
- [ ] 1.2 Add app-level tracking of the currently-open settings window handle (e.g. a `Global` or field the `ShowSettings` handler closes over), set when the window opens, cleared when it closes.
- [ ] 1.3 Wire the settings window's close event to clear the tracked handle and call `SettingsController::close(cx)`.

## 2. Rewire the Settings action

- [ ] 2.1 Update the `on_action::<ShowSettings>` handler (currently `settings_for_action.update(cx, |ctrl, cx| ctrl.open(cx))` in `root_view.rs`) to: if the tracked settings window handle exists and is open, activate/focus it; otherwise call `SettingsController::open(cx)` and open the new window.
- [ ] 2.2 Confirm the `Cmd-,` keybinding (`ui/app/mod.rs`) still resolves to the updated handler with no changes needed to the binding itself.

## 3. Remove the in-window overlay

- [ ] 3.1 Remove the `settings_snap.is_open` conditional branch and `render_settings_panel(...)` call from `LibraryRootView::render` in `root_view.rs`.
- [ ] 3.2 Remove the `settings_focus` focus-trap field/handling from `LibraryRootView` (the `window.focus(&self.settings_focus, cx)` call and the associated focus handle) since there's no overlay left to trap focus in.
- [ ] 3.3 Verify no other code path reads `settings_focus` or depends on the overlay branch (search for `settings_focus` and `render_settings_panel` usages) before deleting.

## 4. Settings window shell

- [ ] 4.1 Give the settings window its own root focus handle and `Esc`-to-close binding, scoped to that window, replacing the removed overlay's `Esc` handling.
- [ ] 4.2 Set an initial window size/title for the settings window (e.g. matching the overlay's current width) via `WindowOptions`.
- [ ] 4.3 Confirm `SettingsController` entity is shared (cloned handle) between the main window and the settings window rather than reconstructed, so state persists across close/reopen.

## 5. Manual verification

- [ ] 5.1 Launch the app, open settings via `Cmd-,`, confirm it opens as a separate OS window (own titlebar, independently movable) and the main window stays interactive (click catalog items, scroll, switch tabs) while it's open.
- [ ] 5.2 Trigger `Cmd-,` again while settings is open; confirm the existing window is brought to front rather than a duplicate opening.
- [ ] 5.3 Edit a draft field (e.g. storage path), close the settings window, reopen it, confirm the draft value and active tab persisted.
- [ ] 5.4 Close the settings window and confirm the main app does not quit and the main window is unaffected.
- [ ] 5.5 Run `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, and `cargo test --all-features --workspace`.
