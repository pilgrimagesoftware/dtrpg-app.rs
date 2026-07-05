## ADDED Requirements

### Requirement: Settings window is opened via a dedicated gpui window
The Rust frontend SHALL open settings using `cx.open_window` with its own `WindowOptions`, mirroring `open_library_window`, rather than compositing `render_settings_panel` as an overlay child inside `LibraryRootView`.

#### Scenario: Opening settings from the main window
- **WHEN** the user triggers the `ShowSettings` action while no settings window is tracked as open
- **THEN** the app calls `cx.open_window` to create a new window whose root renders `render_settings_panel` inside `gpui_component::Root`, and the main library window remains visible and interactive

### Requirement: A single settings window handle is tracked to prevent duplicates
The Rust frontend SHALL track the currently-open settings window handle so that `ShowSettings` reuses it instead of opening a second window.

#### Scenario: Triggering ShowSettings while a settings window is already open
- **WHEN** the `ShowSettings` action fires and a settings window handle is already tracked and open
- **THEN** the app activates/focuses that window instead of calling `cx.open_window` again

#### Scenario: Tracked handle is cleared on close
- **WHEN** the settings window is closed
- **THEN** the tracked window handle is cleared and `SettingsController::close` is invoked

### Requirement: SettingsController entity is shared across windows, not recreated
The Rust frontend SHALL reuse the existing `Entity<SettingsController>` handle for the settings window rather than constructing a new controller per window open.

#### Scenario: Draft state persists across close/reopen
- **WHEN** the user edits a draft field (e.g. storage path) in the settings window, closes the window, then reopens it via `ShowSettings`
- **THEN** the reopened settings window reflects the same `SettingsController` entity state, including the unsaved draft value

### Requirement: Overlay-specific focus handling is removed from the main window
The Rust frontend SHALL remove `LibraryRootView`'s `settings_focus` focus-trap handle and the `settings_snap.is_open` conditional overlay branch, since settings no longer renders inside the main window.

#### Scenario: Main window render path has no settings overlay branch
- **WHEN** `LibraryRootView::render` executes
- **THEN** it no longer conditionally composites `render_settings_panel` or focuses `settings_focus`, regardless of whether the settings window is open
