## Context

`SettingsController` already fetches Gravatar bytes asynchronously on sign-in via `set_logged_in` and stores them in `AuthState::LoggedIn { avatar_bytes }`. `AuthStateSnapshot` already carries `display_initial` (the first character of the email) and `avatar_bytes`. The toolbar's `render_avatar_button` uses `display_initial` but not `avatar_bytes` — it renders the initial as a text label, not an image. The account section of the settings panel currently ignores both fields and receives only `is_authenticated: bool`.

The two gaps are:
1. `AuthStateSnapshot` does not include the full email address, so the settings view cannot display it.
2. The account section has no code to render the avatar circle or email.

## Goals / Non-Goals

**Goals:**
- Show the signed-in user's email address in the account section.
- Show the Gravatar avatar image (if fetched) or a styled initial-letter fallback.
- Avoid duplicating avatar rendering logic between the toolbar button and the settings section.

**Non-Goals:**
- Changing the toolbar avatar button — it stays as a `Button` with an initial label.
- Adding a "change avatar" or "edit account" action.
- Fetching the email from the DTRPG API — the email comes from what the user typed at login and was saved to `profile.toml` (from the `avatar-load-and-cache` change).
- Rendering avatar images anywhere other than the settings account section.

## Decisions

### Add `email` to `AuthStateSnapshot`

`AuthStateSnapshot` gains `email: Option<String>`. Populated from `AuthState::LoggedIn { email, .. }` in `SettingsController::snapshot()`. The field is `None` when logged out.

**Why not read the email from `ProfileConfig` in the view**: Views should not perform disk I/O. The controller already owns the email in `AuthState::LoggedIn`; putting it in the snapshot is the correct data-flow.

### Avatar rendering: GPUI `img()` with byte source, initial fallback

GPUI's `img(source)` accepts `gpui::ImageSource::Data(Arc<ImageData>)`. `ImageData` can be constructed from encoded PNG/JPEG bytes with `ImageData::from_data(format, data)` (available since the Zed-era GPUI API). This produces an `img` element that GPUI decodes and renders.

The avatar circle in the settings section is rendered as:

```
div (56×56 px, rounded_full, overflow_hidden, bg = accent_soft)
  └─ if avatar_bytes.is_some():
       img(ImageSource::Data(Arc<ImageData>))  ← decoded Gravatar JPEG
     else:
       div (centered text, text_xl, text_color = accent)
         └─ display_initial character (or "?" if None)
```

**Why a fixed 56 px circle**: Visually distinct from the 30 px toolbar button; large enough to recognize a face image; small enough to fit above the email text without dominating the settings panel.

**Why not reuse the toolbar button**: The toolbar button is interactive (opens a dropdown menu). The settings section avatar is a display-only identity indicator. Different semantics, different element types.

**Fallback order**:
1. Gravatar image bytes present → render as `img`
2. No bytes but `display_initial` is Some → render initial letter
3. Neither (logged-out path, unreachable in this branch) → render "?"

### Thread `AuthStateSnapshot` instead of `is_authenticated`

`render_settings_panel` and `render_active_section` currently receive `is_authenticated: bool` and pass it to `render_account_section`. Replacing this with `&AuthStateSnapshot` eliminates the redundant bool (already present as `auth.is_logged_in` on the snapshot) and passes the email and avatar bytes without an additional parameter.

`render_settings_panel` receives its data from `SettingsSnapshot`, which already has the `auth` field. The caller (`root_view.rs`) already does `snapshot.auth` for the toolbar — it simply passes it to the settings panel too.

## Risks / Trade-offs

- [Risk] `ImageData::from_data(format, data)` decodes on the main thread. For a ~10 KB Gravatar JPEG this is imperceptible (~0.3 ms). → Acceptable; the avatar is decoded at most once per settings-open per session since the bytes are already in memory.
- [Risk] The GPUI `ImageData` API may differ from what's described above if the pinned Zed commit has a different API surface. → Mitigation: check the GPUI source in `.cargo/git/checkouts/zed-*/crates/gpui/src/image.rs` during implementation and adapt if needed. The fallback to an initial-letter div requires no image API at all.
- [Risk] If the `avatar-load-and-cache` change is not yet applied, `avatar_bytes` will always be `None` in production (since `set_logged_in` is not called from the production sign-in path yet). → Mitigation: the initial-letter fallback handles this gracefully; the image will appear once `avatar-load-and-cache` is implemented.
