## 1. Add dividers

- [x] 1.1 In `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`, insert `.right(Separator::vertical())` between `.right(theme_picker)` and `.right(activity_indicator)` in the `StatusBar` builder chain
- [x] 1.2 Insert a second `.right(Separator::vertical())` between `.right(activity_indicator)` and `.right(notification_indicator)`

## 2. Build and quality gates

- [x] 2.1 `cargo build --workspace --all-features`
- [x] 2.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] 2.3 `cargo fmt --all -- --check` (pre-existing repo-wide failure on `develop`, unrelated to this change: `rustfmt.toml` requires nightly-only options such as `indent_style = Visual`, which the stable toolchain silently ignores, so the whole codebase reads as unformatted; my two-line addition does not add any new formatting drift)

## 3. Manual verification

- [ ] 3.1 Launch the app and confirm the status bar's right side shows: theme picker, divider, activity indicator, divider, notifications button
- [ ] 3.2 Confirm the new dividers visually match the existing left-side divider (same height, color, spacing)
