## Context

`render_avatar_button` in `toolbar_view.rs` has two branches:

1. **Unauthenticated** — already correct: a `div()` with `.rounded_full()`, fixed 30×30 px, surface-alt background, border, and the `👤` icon centered inside.
2. **Authenticated** — currently a plain `Button::new("avatar-btn")` with a text label. No circular shape, no consistent sizing, no image rendering even when `avatar_bytes` is present.

The `toolbar-avatar-display` spec requires circular rendering for all three authenticated states (Gravatar image, initials fallback, unauthenticated placeholder). Only the unauthenticated placeholder meets that requirement today.

## Goals / Non-Goals

**Goals:**
- Replace the authenticated `Button` path with a circular `div` styled consistently with the unauthenticated placeholder.
- Render the Gravatar image (clipped to a circle) when `auth.avatar_bytes` is `Some`.
- Render an accent-color circle with the user's initial when no avatar bytes are available.
- Preserve the logout dropdown behavior.

**Non-Goals:**
- Changing how avatar bytes are fetched, cached, or stored (owned by `avatar-load-and-cache`).
- Altering the unauthenticated branch (it's already correct).
- Changing button sizing — keep 30×30 px to match the existing placeholder.

## Decisions

### Replace `Button` with a styled `div` + manual click/dropdown handling

`Button` in gpui applies its own padding, hover states, and label styling that fight against a circular shape. A plain `div` with `.rounded_full()` gives full control over visual output and exactly mirrors the working unauthenticated branch.

The logout dropdown must be preserved. GPUI `div` elements support `.on_click` and context menus; the dropdown can be attached the same way existing toolbar buttons use it.

### Image rendering: `img()` element clipped with `rounded_full()`

When `avatar_bytes` is `Some`, use gpui's `img()` element inside the circular `div`. Applying `.rounded_full()` to the `img()` itself (or to its container) clips the raster image to a circle. This is the same mechanism used for any other rounded image in the UI.

### Initials branch: `div` with accent-color background

When authenticated but no avatar bytes, use the same `div` structure as the unauthenticated branch, substituting:
- Background: accent color (not `surface_alt`)
- Content: initial letter in white, `text_xs`
- No border (accent bg provides sufficient contrast)

## Risks / Trade-offs

- **Image aspect ratio**: Gravatar images are square by spec, so `rounded_full()` on a square image produces a perfect circle. Non-square images would appear stretched — acceptable given Gravatar's guaranteed square output.
- **Dropdown attachment**: Moving from `Button` to `div` requires wiring the dropdown manually. If the GPUI API changes for context menus, this will need updating — but it mirrors existing patterns in the codebase.
