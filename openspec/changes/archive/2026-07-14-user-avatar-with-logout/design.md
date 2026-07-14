## Context

The toolbar currently has a gear button for settings and nothing indicating account identity. The settings panel's Account tab is already stubbed. This change adds a toolbar avatar that:

1. Fetches a Gravatar image asynchronously using the authenticated user's email.
2. Falls back to a generated initials avatar if no Gravatar exists.
3. Uses a generic icon when unauthenticated.
4. Shows a logout popover on click (authenticated only).

Auth state is still stubbed until `secure-credential-storage` is connected. The controller gains the shape of auth state, but the email value will be hardcoded or empty until that change lands.

## Goals / Non-Goals

**Goals:**
- Circular 32×32 avatar button to the right of the gear button.
- Gravatar fetch via HTTP on a background task; cached for the session in `SettingsController`.
- Generated initials fallback (first character of email, white on accent circle).
- Generic icon fallback when logged out.
- gpui-component `DropdownMenu` or `Popover` for the logout option.
- Auth state modeled in `SettingsController` with a clean `AuthState` enum.

**Non-Goals:**
- Persisting auth state (blocked on `secure-credential-storage`).
- Editing profile information from the avatar menu.
- Animated loading spinner while the Gravatar fetch is in flight (fallback is shown immediately).
- Cross-session Gravatar caching (memory-only for the session).

## Decisions

### Decision 1: MD5 hash via the `md5` crate

Gravatar requires the MD5 hash of the lower-cased, trimmed email. The `md5` crate (v0.7, no_std-compatible, ~2 KB) is the smallest correct option. We compute `format!("{:x}", md5::compute(email.trim().to_lowercase()))` and append it to `https://www.gravatar.com/avatar/`.

**Alternative considered**: Implement MD5 inline — unnecessary complexity. Using `sha2` or `blake3` — wrong hash algorithm for Gravatar. Rejected.

### Decision 2: Fetch with `reqwest` on a `cx.background_executor()` task

`reqwest` is used (or will be used) by the SDK adapter layer and should already be in the dependency tree. We spawn a background task via `cx.background_executor().spawn(async move { ... })` that:
1. Builds the Gravatar URL with `?d=404&s=64` (returns 404 for missing avatars rather than a default image, so we can detect absence cleanly).
2. Issues a GET request with a 5-second timeout.
3. On success (200), returns the raw bytes.
4. On any failure, returns `None`.

The spawned task calls `entity.update(cx, |ctrl, cx| ctrl.set_avatar_bytes(Some(bytes), cx))` on the UI thread when done.

**Alternative considered**: Use `d=mp` (mystery person default) — then we can't distinguish "real Gravatar" from "Gravatar fallback", preventing the initials avatar from rendering. Rejected.

### Decision 3: Auth state lives in `SettingsController`

```rust
pub enum AuthState {
    LoggedOut,
    LoggedIn { email: String, avatar_bytes: Option<Arc<Vec<u8>>> },
}
```

`SettingsController` gains `auth_state: AuthState`. Methods: `set_logged_in(email, cx)` (triggers Gravatar fetch), `set_avatar_bytes(bytes, cx)`, `logout(cx)`. `SettingsSnapshot` gains `auth_state: AuthStateSnapshot { is_logged_in: bool, display_initial: Option<char>, avatar_bytes: Option<Arc<Vec<u8>>> }`.

`Arc<Vec<u8>>` is used so the bytes can be cloned cheaply into the snapshot without a full copy on every render pass.

**Alternative considered**: A separate `AvatarController` entity — premature separation; auth state and settings are closely related (the Account tab already shows auth state). Rejected for now.

### Decision 4: Logout popover via `gpui-component` `DropdownMenu`

`gpui-component` already provides `DropdownMenu` / `PopupMenuItem` and it is used in the sort selector. We reuse the same pattern for the avatar popover, anchored below the avatar button. When `AuthState::LoggedOut`, the `on_click` handler on the avatar div is a no-op.

### Decision 5: Render avatar image bytes as a gpui `img()` element

gpui has an `img()` builder function that renders image data. We pass the raw PNG bytes (Gravatar always returns JPEG or PNG) via `ImageSource::Data` (or the equivalent gpui API). If the API uses a URI source, we serve the bytes through a local `gpui::ImageData` handle. The circular clip is achieved via `.rounded_full()` on the containing div.

**Note**: The exact gpui image API should be confirmed during implementation — if `img()` does not accept raw bytes directly, the approach is to decode to an `ImageData` and register it with the `cx.image_cache()` or equivalent, then reference by ID.

## Risks / Trade-offs

**[Risk] `reqwest` not yet in the workspace** → Add `reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json"] }` to workspace deps. If a different HTTP client is already present (e.g., `ureq`), use that instead to avoid two HTTP stacks.

**[Risk] gpui image-from-bytes API shape** → gpui's `img()` API may require a `SharedUri` or a registered `ImageData`. During implementation, check the gpui source; adapt to whichever pattern exists. This does not affect the spec or design — only the rendering call.

**[Risk] MD5 on user email is not secret** → Gravatar's own protocol uses MD5 in public URLs. This is an accepted limitation of the Gravatar service, not a security issue introduced here.

**[Risk] Avatar state in `SettingsController` mixes auth and settings concerns** → Accepted trade-off; both live in the same panel. If a dedicated `AuthController` is introduced later (likely when `secure-credential-storage` lands), the `auth_state` field can be moved.

## Migration Plan

1. Add `md5 = "0.7"` to workspace; confirm `reqwest` is present or add it.
2. Add `data/avatar.rs` with hash helper and fetch function.
3. Add `AuthState` enum and fields to `SettingsController`; update `SettingsSnapshot`.
4. Implement Gravatar fetch trigger in `set_logged_in()`.
5. Add `render_avatar_button()` to `toolbar_view.rs`; insert it in the controls row.
6. Wire logout action: `ctrl.logout(cx)` resets to `AuthState::LoggedOut`.
7. `cargo check -p dtrpg-ui` with zero errors.
8. Stub with a test email for visual verification until `secure-credential-storage` connects.
