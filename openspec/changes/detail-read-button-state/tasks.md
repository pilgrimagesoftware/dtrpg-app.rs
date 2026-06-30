## 1. Detail panel Read button state

- [x] 1.1 In `detail_panel_view.rs`, give the Read button `.id("detail-read")` in both enabled and disabled branches
- [x] 1.2 When `is_downloaded`, render the Read button with accent background, `.cursor_pointer()`, and the existing label — unchanged from current behavior
- [x] 1.3 When `!is_downloaded`, render the Read button with accent background at `.opacity(0.4)`, no `.cursor_pointer()`, no `.on_click()`, and a `.tooltip(|w, cx| Tooltip::new("Download this item first").build(w, cx))`

## 2. Verification

- [x] 2.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 2.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [x] 2.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 2.4 Launch the app; open the detail panel for a non-downloaded item; confirm the Read button appears dimmed and non-clickable with a tooltip on hover
- [ ] 2.5 Trigger a download to make the item downloaded; confirm the Read button becomes fully enabled with accent appearance and no tooltip
