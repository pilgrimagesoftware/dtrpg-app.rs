## Context

`AuthStateSnapshot` is the view-layer snapshot of auth state. It currently carries `is_logged_in: bool`, `display_initial: Option<char>`, and `avatar_bytes: Option<Arc<Vec<u8>>>`. The `email` field exists in `AuthState::LoggedIn` in `SettingsController` but is not propagated to the snapshot, so it is inaccessible to render functions.

The avatar dropdown is built in the `dropdown_menu` closure on `Button`. Closures passed to `dropdown_menu` must be `'static`, so any data used inside must be owned (moved in). Currently only `settings: Entity<SettingsController>` is moved in.

`PopupMenuItem::label(text)` renders a non-interactive, visually disabled label row in the popup menu — exactly the right primitive for a read-only email header. `PopupMenuItem::separator()` renders a horizontal rule.

## Goals / Non-Goals

**Goals:**
- Add `email: Option<String>` to `AuthStateSnapshot`
- Propagate the email from `AuthState::LoggedIn` in `SettingsController::snapshot()`
- Move a cloned `email: String` (or `Option<String>`) into the `dropdown_menu` closure
- Prepend `PopupMenuItem::label(email)` + `PopupMenuItem::separator()` before "Log Out"

**Non-Goals:**
- Showing a display name (not stored; only email is available)
- Adding a "Settings" shortcut item (separate change)
- Truncating long email addresses (the `PopupMenuItem::label` row will truncate via the menu's own overflow handling)

## Decisions

### Add `email: Option<String>` to `AuthStateSnapshot`

`None` when logged out, `Some(email.clone())` when logged in. This follows the existing `Option<char>` and `Option<Arc<Vec<u8>>>` pattern in the same struct. The snapshot is already cloned per render pass, so a `String` clone is acceptable.

### Use `PopupMenuItem::label` not a custom element

`PopupMenuItem::label(text)` is the purpose-built API for non-interactive menu header rows. It renders as disabled text, which is the correct affordance — the email is information, not an action.

### Move `Option<String>` into the closure; guard on `is_logged_in`

The unauthenticated avatar button already returns early (no dropdown is attached to it), so the closure is only called when logged in. `auth.email` will always be `Some` at that point. Cloning the `String` before the closure is the idiomatic approach, matching the `settings.clone()` pattern used for the entity.

## Risks / Trade-offs

- **Email privacy**: The email is shown in a dropdown visible only to the user operating the app. No network call, no logging. No new disclosure risk.
- **`AuthStateSnapshot` field addition**: All construction sites for `AuthStateSnapshot` are in `settings.rs` (two branches in `snapshot()`). Rust's exhaustive struct initialization will produce a compile error if either branch is missed, making the change safe to apply.
