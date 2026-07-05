## 1. Window opening and tracking

- [x] 1.1 Add a settings-window-open function in `dtrpg-ui/src/ui/app/mod.rs` (parallel to `open_library_window`) that opens a `WindowOptions` window whose root is `render_settings_panel` (or a new thin root view wrapping it) inside `gpui_component::Root`.
- [x] 1.2 Add app-level tracking of the currently-open settings window handle (e.g. a `Global` or field the `ShowSettings` handler closes over), set when the window opens, cleared when it closes.
- [x] 1.3 Wire the settings window's close event to clear the tracked handle and call `SettingsController::close(cx)`.

## 2. Rewire the Settings action

- [x] 2.1 Update the `on_action::<ShowSettings>` handler (currently `settings_for_action.update(cx, |ctrl, cx| ctrl.open(cx))` in `root_view.rs`) to: if the tracked settings window handle exists and is open, activate/focus it; otherwise call `SettingsController::open(cx)` and open the new window.
- [x] 2.2 Confirm the `Cmd-,` keybinding (`ui/app/mod.rs`) still resolves to the updated handler with no changes needed to the binding itself.

## 3. Remove the in-window overlay

- [x] 3.1 Remove the `settings_snap.is_open` conditional branch and `render_settings_panel(...)` call from `LibraryRootView::render` in `root_view.rs`.
- [x] 3.2 Remove the `settings_focus` focus-trap field/handling from `LibraryRootView` (the `window.focus(&self.settings_focus, cx)` call and the associated focus handle) since there's no overlay left to trap focus in.
- [x] 3.3 Verify no other code path reads `settings_focus` or depends on the overlay branch (search for `settings_focus` and `render_settings_panel` usages) before deleting. Also removed the `SettingsChanged` subscription that refocused `root_focus` to work around the overlay's stale-focus-handle bug — no longer applicable since settings has its own window/focus tree.

## 4. Settings window shell

- [x] 4.1 Give the settings window its own root focus handle and `Esc`-to-close binding, scoped to that window, replacing the removed overlay's `Esc` handling.
- [x] 4.2 Set an initial window size/title for the settings window (e.g. matching the overlay's current width) via `WindowOptions`.
- [x] 4.3 Confirm `SettingsController` entity is shared (cloned handle) between the main window and the settings window rather than reconstructed, so state persists across close/reopen.
- [x] 4.4 (Added during implementation) Replace gpui-component's `Settings` widget-driven page navigation with a custom `Sidebar`/`SidebarMenu` driven by `SettingsController::active_page_ix`, persisted via `UiPrefs`. Discovered during manual testing: the vendored `Settings` widget tracks the active page in its own per-window state with no exposed way to read it back, so a new settings window always reset to the first page — active tab did not survive close/reopen despite draft values doing so. Storing the page index in `SettingsController` (persisted to `ui_prefs.toml`) restores it correctly.

## 5. Manual verification

- [x] 5.1 Launch the app, open settings via `Cmd-,`, confirm it opens as a separate OS window (own titlebar, independently movable) and the main window stays interactive (click catalog items, scroll, switch tabs) while it's open. Verified live via screenshot: clicking a point on the main "Libri" window (outside the Settings window's bounds) registered on the Libri window per Accessibility API while Settings remained open.
- [x] 5.2 Trigger `Cmd-,` again while settings is open; confirm the existing window is brought to front rather than a duplicate opening. Verified live: only two windows ("Settings", "Libri") existed after a second `Cmd-,` invocation.
- [x] 5.3 Edit a draft field (e.g. storage path), close the settings window, reopen it, confirm the draft value and active tab persisted. Draft-value persistence verified via code review (`SettingsController` entity reused, unaffected by this change). Tab persistence verified live before the 4.4 fix (reproduced the bug: reopened on "Account" instead of "About"); the fix itself (4.4) is verified by code review and passing build/clippy/tests — a second live click-through after the fix was blocked by a macOS keychain re-authorization prompt on rebuild that requires the user's login password, and the user opted to skip re-verifying live.
- [x] 5.4 Close the settings window and confirm the main app does not quit and the main window is unaffected. Verified live: `dtrpg-core` process remained running and the "Libri" window was the only window left after closing Settings via its native close button.
- [x] 5.5 Run `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, and `cargo test --all-features --workspace`. `cargo +nightly fmt` and `cargo test` (excluding `util::datetime`, a pre-existing unrelated i18n pluralization failure on `develop`) pass clean for all files touched by this change. `cargo clippy -D warnings` is blocked workspace-wide by one pre-existing `unused_variables` error in `settings_account_view.rs` (confirmed present on `develop` before this change, unrelated file); verified no new clippy issues from this change's files via `cargo clippy ... -A unused-variables`.
