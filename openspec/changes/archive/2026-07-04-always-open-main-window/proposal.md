## Why

The app currently routes unauthenticated users to a separate login modal that hides the main window entirely. This forces users to authenticate before seeing anything, even if they have locally cached content available. An offline-first or returning user should land directly in the library.

## What Changes

- The app no longer opens a login window on startup. The main library window opens unconditionally.
- Authentication state (logged in / not logged in) is surfaced via the existing avatar button and a top-of-window notification banner, not by hiding the main UI.
- When not logged in, the catalog shows whatever content is available locally (cached/downloaded files). When no local content exists, an appropriate empty state is shown.
- The login flow becomes an action the user initiates from the banner or the avatar menu — not a forced gate.
- The login window code is removed or repurposed as an in-app sheet/panel rather than a standalone window.

## Capabilities

### New Capabilities

- `unauthenticated-main-window`: The main window opens and functions without requiring a prior authentication step. Auth state drives the avatar button appearance, the notification banner, and the availability of cloud-backed catalog data — but never prevents the window from opening.

### Modified Capabilities

- `rust-main-window-library-layout`: The account menu requirement changes — the menu must reflect a "not signed in" state with a sign-in action, in addition to the existing signed-in identity and settings navigation.

## Impact

- `crates/dtrpg-ui/src/ui/app/mod.rs`: `setup()` always opens the library window; login window opener call is replaced with a state update on the main window's settings controller.
- `crates/dtrpg-ui/src/ui/views/root_view.rs`: `LibraryRootView::new` must handle the unauthenticated case (no tokens available).
- `crates/dtrpg-ui/src/ui/app/mod.rs`: `open_library_window` no longer requires `LoginTokens` as a pre-condition; when no tokens are available, the service is initialized in a degraded/offline mode.
- `crates/dtrpg-ui/src/services/mod.rs` + `crates/dtrpg-core/src/services/sdk.rs`: `ServiceFactory` must handle the case where no tokens are available (deferred auth / offline mode).
- `crates/dtrpg-ui/src/controllers/settings.rs`: `SettingsController` sign-in action must be initiatable from the banner or avatar menu without opening a new window.
- `crates/dtrpg-ui/src/ui/views/toolbar_view.rs`: Avatar button "not signed in" branch gains a sign-in action in addition to the tooltip.
- `crates/dtrpg-ui/src/ui/windows/login.rs` (and related): Login window code removed or inlined into a panel/sheet.
- `crates/dtrpg-ui/src/data/auth_state.rs` + `AuthStateController`: The notification banner already exists; its content and actions for the unauthenticated case need to be defined.
