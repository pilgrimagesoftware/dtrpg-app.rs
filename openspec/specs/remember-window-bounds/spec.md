### Requirement: Library window position and size persist across restarts
The library window's position and size SHALL be saved whenever the window is about to close, and SHALL be restored the next time the app launches instead of always using the default placement.

#### Scenario: Window reopens at its last position and size
- **WHEN** the user resizes and/or moves the library window, then quits and relaunches the app
- **THEN** the library window opens at the position and size it had just before quitting

#### Scenario: First launch uses the default placement
- **WHEN** the app launches with no persisted window bounds (e.g. first launch, or a preferences file predating this preference)
- **THEN** the library window opens at the default placement, the same as before this change

### Requirement: Saved bounds are validated against connected displays before restoring
If the saved window bounds no longer intersect any currently connected display, the app SHALL fall back to the default placement rather than opening the window off-screen or on a monitor that is no longer connected.

#### Scenario: Saved display is no longer connected
- **WHEN** the app launches with saved window bounds that don't intersect any currently connected display (e.g. an external monitor the window was previously on has been disconnected)
- **THEN** the library window opens at the default placement instead of the saved position

#### Scenario: Saved display is still connected
- **WHEN** the app launches with saved window bounds that intersect a currently connected display
- **THEN** the library window opens at the saved position and size

### Requirement: Only the library window's bounds are remembered
The settings window's position and size SHALL NOT be persisted — it always opens centered at a fixed size, independent of this preference.

#### Scenario: Settings window is unaffected
- **WHEN** the user opens Settings after moving or would-be-resizing the library window
- **THEN** the settings window opens centered at its fixed size, regardless of any saved library window bounds
