## 1. Restructure Account Section Layout

- [ ] 1.1 In `settings_account_view.rs`, wrap the identity column div in an outer `div().flex().items_center().justify_between()` row; give the inner identity column `.flex_1().min_w_0()` so it shrinks correctly
- [ ] 1.2 Add a square icon button (32×32, same border/bg style as the gear button) as the second child of the outer flex row (right side); label it with the reload symbol `↺` and attach `.tooltip(|window, cx| Tooltip::new("Reset API Key").build(window, cx))`
- [ ] 1.3 In the actions section below the divider, remove `render_action_button("Reset API Key", accent, accent_on)`; leave only `render_action_button("Log Out", accent, accent_on)`
- [ ] 1.4 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 2. Verification

- [ ] 2.1 Build and run the app; open Settings → Account; confirm the reload symbol button appears to the right of the account info text on the same row
- [ ] 2.3 Hover over the reload symbol button and confirm the "Reset API Key" tooltip appears
- [ ] 2.2 Confirm only "Log Out" appears in the actions area below the divider
