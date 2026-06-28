## Why

The authenticated avatar button renders as a plain rectangular `Button` with a text label, ignoring the circular shape already required by the `toolbar-avatar-display` spec. The unauthenticated placeholder is already circular; the logged-in paths are not, producing an inconsistent and unfinished appearance.

## What Changes

- The authenticated avatar button (initials fallback) is replaced with a circular `div` styled consistently with the unauthenticated placeholder: same 30×30 px size, `rounded_full()`, accent-color background, white initial letter.
- When `avatar_bytes` are available, the avatar button renders the image clipped to a circle instead of raw bytes or a plain button.
- The existing `Button::new("avatar-btn")` in the authenticated path is replaced with a styled `div` that still surfaces the logout dropdown.

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

- `toolbar-avatar-display`: The circular rendering requirement (already in the spec) is now enforced for both the initials fallback and the Gravatar image paths when the user is authenticated.

## Impact

- `dtrpg-ui/src/ui/views/toolbar_view.rs` — `render_avatar_button`: replace `Button::new("avatar-btn")` authenticated branch with a circular `div`; add image rendering when `auth.avatar_bytes` is `Some`.
- No changes to `SettingsController`, `avatar.rs`, or any data layer.
