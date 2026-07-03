## 1. Add dividers

- [ ] 1.1 In `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`, insert `.right(Separator::vertical())` between `.right(theme_picker)` and `.right(activity_indicator)` in the `StatusBar` builder chain
- [ ] 1.2 Insert a second `.right(Separator::vertical())` between `.right(activity_indicator)` and `.right(notification_indicator)`

## 2. Build and quality gates

- [ ] 2.1 `cargo build --workspace --all-features`
- [ ] 2.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 2.3 `cargo fmt --all -- --check`

## 3. Manual verification

- [ ] 3.1 Launch the app and confirm the status bar's right side shows: theme picker, divider, activity indicator, divider, notifications button
- [ ] 3.2 Confirm the new dividers visually match the existing left-side divider (same height, color, spacing)
