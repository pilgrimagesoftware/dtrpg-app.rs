## 1. Fix close button colors

- [x] 1.1 In `detail_panel_view.rs`, add `let scrim = colors.scrim;` (accent_on was already bound); removed unused `hover`
- [x] 1.2 On the close button div, change `.bg(hover)` to `.bg(scrim)`
- [x] 1.3 On the close button div, change `.text_color(text_secondary)` to `.text_color(accent_on)`

## 2. Verify

- [x] 2.1 Run `cargo check --all-targets` and confirm no compile errors
- [ ] 2.2 Manually launch the app, select several items with different cover colors (light and dark), and confirm the close button is clearly visible in all cases
- [ ] 2.3 If theme switching is available, verify the button is visible in each of the four themes
