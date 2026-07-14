All tasks below are moot — see `proposal.md`'s "Resolution" section. The anchor fix these
tasks describe was already implemented on `develop` prior to this change (commit `47c0661`),
via `gpui-component`'s `Popover` anchored to each panel's trigger button in
`status_bar_view.rs`. Left unchecked to keep an honest record that no work happened here.

## 1. Capture Button Bounds

- [ ] 1.1 Determine the status bar activity button's on-screen bounds at render time
      (GPUI element bounds via layout, or an `.id()` + `window.bounds_for_id`-style lookup
      if available; otherwise track via `on_mouse_down`/layout callback)
- [ ] 1.2 Store the bounds where `root_view.rs` can read them when deciding whether to
      render the activity panel overlay

## 2. Anchor the Panels

- [ ] 2.1 `render_activity_panel` accepts an anchor position/bounds parameter and computes
      its `top`/`left` (or `bottom`/`right`) from it instead of the hardcoded
      `bottom(px(56.0)).left(px(8.0))`
- [ ] 2.2 Apply the same fix to `render_alert_history_panel`'s anchor

## 3. Build and Quality

- [ ] 3.1 `cargo check --workspace`
- [ ] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Click the status bar activity button and confirm the panel opens directly above
      it, not in the bottom-left corner
- [ ] 4.2 Click the notification button and confirm the alert history panel opens near it
- [ ] 4.3 Resize the window and confirm both panels track the button's new position
