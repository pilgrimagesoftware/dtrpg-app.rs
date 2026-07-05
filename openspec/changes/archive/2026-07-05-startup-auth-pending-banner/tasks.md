## 1. Add Authenticating notice kind and pending flag

- [x] 1.1 Add `NoticeKind::Authenticating` variant to `crates/dtrpg-ui/src/data/notification.rs`
- [x] 1.2 Add `is_auth_pending: bool` field to `AuthStateController` in `crates/dtrpg-ui/src/controllers/auth_state.rs`
- [x] 1.3 Add `set_auth_pending(&mut self, pending: bool, cx)` method to `AuthStateController` that updates `is_auth_pending`, regenerates notices, and emits `AuthStateChanged`; calling `set_state` should also clear `is_auth_pending`
- [x] 1.4 Update `notices_for` (or `active_notices`) so that when `is_auth_pending` is `true`, the returned notice is `NoticeKind::Authenticating` instead of `NoticeKind::NotSignedIn`

## 2. Add startup auth events

- [x] 2.1 Add `StartupAuthBegun` event struct to `crates/dtrpg-ui/src/data/events.rs` and `impl EventEmitter<StartupAuthBegun> for SettingsController {}`
- [x] 2.2 Add `StartupAuthFailed` event struct to `crates/dtrpg-ui/src/data/events.rs` and `impl EventEmitter<StartupAuthFailed> for SettingsController {}`
- [x] 2.3 In `SettingsController::startup_auth`, emit `StartupAuthBegun` immediately (before spawning the background task) and emit `StartupAuthFailed` on the error branch instead of only logging

## 3. Wire pending state in LibraryRootView

- [x] 3.1 In `LibraryRootView::new`, move `auth_state` entity creation and all `cx.subscribe` calls to before the `startup_api_key` check, so subscriptions are registered before `startup_auth` fires
- [x] 3.2 Subscribe to `StartupAuthBegun` from `settings`: call `auth_state.update(cx, |ctrl, cx| ctrl.set_auth_pending(true, cx))`
- [x] 3.3 Subscribe to `StartupAuthFailed` from `settings`: call `auth_state.update(cx, |ctrl, cx| ctrl.set_auth_pending(false, cx))`
- [x] 3.4 Update the existing `SignInSucceeded` subscription to also call `ctrl.set_auth_pending(false, cx)` on `auth_state` before (or as part of) transitioning to `Authenticated` (clearing the flag is idempotent but explicit)

## 4. Render the Authenticating notice row

- [x] 4.1 In `crates/dtrpg-ui/src/ui/views/notification_banner_view.rs`, update `notice_strings` to return a label for `NoticeKind::Authenticating` (e.g. `"Signing in to DriveThruRPG..."`)
- [x] 4.2 In the notice row rendering loop, skip the action button and dismiss button when `notice.kind == NoticeKind::Authenticating`

## 5. Update tests and verify

- [x] 5.1 Add unit tests to `auth_state.rs`: pending flag produces `Authenticating` notice; `set_state` clears pending flag; clearing flag with `Unauthenticated` restores `NotSignedIn` notice
- [x] 5.2 Run `cargo test --all-features --workspace` and confirm all tests pass
- [x] 5.3 Manually launch the app with a valid stored API key and confirm "Signing in..." appears briefly before the banner disappears
- [x] 5.4 Manually launch the app with an invalid stored API key and confirm "Signing in..." appears then transitions to "Not signed in"
