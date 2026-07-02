## Context

`AuthStateController::new()` unconditionally calls `initial_state()`, which returns `AuthState::Unauthenticated` in release builds (and optionally an env-var override in debug builds). `LibraryRootView::new` creates a fresh controller with `cx.new(|_| AuthStateController::new())`. Because `Unauthenticated` generates a `NotSignedIn` notice immediately, the banner fires on every library window open regardless of whether the user is actually signed in.

The library window is only ever opened from two call sites — `open_library_window` in `app/mod.rs` — both of which occur after successful authentication. The default of `Unauthenticated` is therefore always wrong for this context.

## Goals / Non-Goals

**Goals:**
- Eliminate the spurious "not signed in" banner when the library window first opens.
- Preserve the ability to transition to `Unauthenticated` or `SessionExpired` later (e.g. token expiry detection).
- Keep the debug env-var override working.

**Non-Goals:**
- Changing how or when the banner is dismissed by the user.
- Detecting session expiry at runtime (out of scope).

## Decisions

### Pass initial state as a parameter to `AuthStateController::new`

Change the signature to `AuthStateController::new(initial_state: AuthState) -> Self` and remove the `initial_state()` helper. The caller is now responsible for providing the correct starting state.

**Why**: The controller has no access to the keyring and no way to know whether the window was opened post-auth or from some other code path. Making the initial state explicit at the call site is clearer and more testable than having the controller guess from the environment.

**Alternative considered**: `AuthStateController::new()` checks the keyring itself (reads the access token). Rejected because it adds a platform I/O call inside a controller constructor and duplicates the auth check that `app/mod.rs` already performed.

**Alternative considered**: A separate `AuthStateController::new_authenticated()` constructor. Redundant — a single parameterized constructor is simpler.

### `LibraryRootView::new` passes `AuthState::Authenticated`

The only call site for `AuthStateController::new` inside the library path is `LibraryRootView::new`. It passes `AuthState::Authenticated`.

**Debug override preserved**: The `initial_state()` helper logic (env-var check) moves inline into the call site in `LibraryRootView::new` so development overrides still work.

## Risks / Trade-offs

- [Risk] Other future callers of `AuthStateController::new` must remember to pass the correct state → Mitigation: the compiler enforces the argument; there is no zero-arg constructor to accidentally use.
- [Risk] `Default` impl for `AuthStateController` (currently delegates to `new()`) breaks if we remove the zero-arg form → Mitigation: remove `impl Default for AuthStateController` since no callers use it via `Default`.
