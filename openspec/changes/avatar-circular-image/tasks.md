## 1. Refactor Authenticated Avatar Button

- [ ] 1.1 Replace `Button::new("avatar-btn")` authenticated branch in `render_avatar_button` with a circular `div` (30×30 px, `rounded_full()`, accent-color background)
- [ ] 1.2 Center the user's initial letter inside the circular `div` as white `text_xs`
- [ ] 1.3 Attach the logout dropdown to the new `div` element (matching the existing `Button` dropdown behavior)
- [ ] 1.4 Add a tooltip ("Account") to the authenticated circular `div`

## 2. Gravatar Image Rendering

- [ ] 2.1 Add a branch inside the authenticated path: when `auth.avatar_bytes` is `Some`, render an `img()` element inside the circular `div`
- [ ] 2.2 Apply `rounded_full()` to the `img()` element (or its container) to clip the image to a circle
- [ ] 2.3 Verify the image fills the 30×30 px container without distortion

## 3. Verification

- [ ] 3.1 Build and run the app; confirm the authenticated avatar displays as a circle with the correct initial and accent color
- [ ] 3.2 Verify the logout dropdown appears on click of the authenticated avatar
- [ ] 3.3 Confirm the unauthenticated placeholder is visually unchanged
- [ ] 3.4 Run `cargo clippy --all-targets --all-features -- -D warnings` with no new warnings
