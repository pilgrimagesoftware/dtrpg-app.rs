## 1. Update AuthStateController

- [ ] 1.1 Change `AuthStateController::new()` to `AuthStateController::new(initial_state: AuthState) -> Self`, removing the `initial_state()` helper function
- [ ] 1.2 Remove `impl Default for AuthStateController` (no callers use it via `Default`)
- [ ] 1.3 Update all unit tests in `auth_state.rs` to pass an explicit `AuthState` to `AuthStateController::new`

## 2. Update LibraryRootView

- [ ] 2.1 In `LibraryRootView::new`, pass `AuthState::Authenticated` to `AuthStateController::new`; wrap with the debug env-var override so `DTRPG_AUTH_STATE_OVERRIDE` still works in debug builds

## 3. Verify

- [ ] 3.1 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 3.2 Manually launch the app with a valid API key and confirm no "not signed in" banner appears in the library window
