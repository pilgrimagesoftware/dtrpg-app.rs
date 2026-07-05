## Context

After the `startup-auth-pending-banner` change, `LibraryRootView::new` calls `settings.startup_auth(key, cx)` when a stored API key exists. The window opens with `AuthState::Unauthenticated`, which immediately generates a `NotSignedIn` notice. A fraction of a second later the background auth resolves and the banner disappears (on success) or stays (on failure). The flash of "Not signed in" while valid credentials are being checked is confusing.

`AuthStateController` owns the current `AuthState` and derives the active notice list from it via `notices_for()`. `NoticeKind` has two variants: `NotSignedIn` and `SessionExpired`. The notification banner renders each active notice as a row with a primary action button and a dismiss button.

## Goals / Non-Goals

**Goals:**
- Show a neutral "Signing in..." indicator while startup re-authentication is in flight.
- Transition to no banner on success and the "Not signed in" banner on failure.
- Keep `AuthState` itself unchanged (no new `Authenticating` variant).

**Non-Goals:**
- Showing auth progress for interactive sign-in via the settings panel (the settings overlay already has `sign_in_in_progress` state for that).
- A loading spinner or animation; a static text row is sufficient.

## Decisions

### Decision: `is_auth_pending` flag on `AuthStateController` rather than a new `AuthState` variant

Adding `AuthState::Authenticating` would require updating every `match` on `AuthState` across the codebase and would conflate a transient UI concern (notice display) with domain state. The pending state is purely cosmetic — it does not affect what services are active or what the library controller does.

Instead, `AuthStateController` gains `is_auth_pending: bool`. When `true`, `active_notices()` returns a single `Authenticating` notice (replacing the `NotSignedIn` notice that would otherwise appear) regardless of the underlying `AuthState`. On auth completion the flag is cleared; normal notice derivation resumes.

**Alternatives considered:**
- New `AuthState::Authenticating` variant → blast radius across all `match` arms; conflates UI and domain state.
- Start the window as `AuthState::Authenticated` with no tokens (suppress the banner entirely) → misleading; the library service would be in stub mode while appearing authenticated.
- Do nothing; accept the flash → degrades perceived quality without any technical reason.

### Decision: Two new events — `StartupAuthBegun` and `StartupAuthFailed`

`SettingsController::startup_auth` currently emits `SignInSucceeded` on success (which `LibraryRootView` subscribes to). Adding `StartupAuthBegun` (emitted when the background task starts) and `StartupAuthFailed` (emitted on error) lets `LibraryRootView` drive `AuthStateController` without giving `SettingsController` a reference to `AuthStateController`.

`StartupAuthBegun` → `LibraryRootView` sets `auth_pending = true` on `auth_state`.
`SignInSucceeded` → existing handler sets `auth_state` to `Authenticated` (also clears `is_auth_pending` implicitly via `set_state`).
`StartupAuthFailed` → `LibraryRootView` clears `auth_pending` (triggers `Unauthenticated` notice as before).

### Decision: Reorder construction in `LibraryRootView::new`

Currently `startup_auth` is called before `auth_state` is created, so subscriptions are not yet wired when the events fire. Move `auth_state` creation and all subscriptions before the `startup_auth` call. Events fire asynchronously (the spawn resolves later) so the order is safe.

### Decision: `Authenticating` notice row has no action or dismiss buttons

The row is informational only — there is nothing for the user to do while auth is in flight. Rendering a "Cancel" button would require plumbing cancellation into `startup_auth`, which is out of scope. The `NoticeAction` enum does not need a new variant; `NoticeKind::Authenticating` notices are filtered out of the action/dismiss rendering path.

## Risks / Trade-offs

- [Very fast auth on localhost/loopback may make the banner flash for <50ms] → Acceptable; still better than showing "Not signed in".
- [Reordering `LibraryRootView::new` construction] → Low risk; the GPUI entity model is not order-sensitive for async events. Subscriptions must be set before events fire, but background tasks don't fire until the next async tick.
- [`StartupAuthBegun` / `StartupAuthFailed` are consumed only by `LibraryRootView`] → No concern about orphaned subscribers; `detach()` on the subscription handle is correct.
