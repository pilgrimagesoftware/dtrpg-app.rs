## Why

`AuthStateController` always initializes to `Unauthenticated`, which immediately generates a "Not Signed In" banner notice. The library window is only ever opened after successful authentication, so the controller's default is incorrect — the banner fires every time even when the user is fully signed in.

## What Changes

- `AuthStateController::new()` gains an `initial_state: AuthState` parameter so callers can provide the correct starting state.
- `LibraryRootView::new` passes `AuthState::Authenticated` when constructing the controller, eliminating the spurious banner.
- The debug environment-variable override (`DTRPG_AUTH_STATE_OVERRIDE`) is preserved so testing unauthenticated/expired states in development remains possible.

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

<!-- No spec-level requirement changes; this is a bug fix in initialization logic only. -->

## Impact

- `dtrpg-ui/src/controllers/auth_state.rs` — `AuthStateController::new()` accepts `initial_state: AuthState`
- `dtrpg-ui/src/ui/views/root_view.rs` — passes `AuthState::Authenticated` to `AuthStateController::new()`
- `dtrpg-ui/src/controllers/auth_state.rs` tests — updated to pass an explicit initial state
