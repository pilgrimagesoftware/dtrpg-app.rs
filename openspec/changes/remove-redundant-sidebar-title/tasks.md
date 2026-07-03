## 1. Remove Duplicate Wordmark

- [ ] 1.1 Remove the `t!("sidebar.app_name")` child from `build_header` in
      `sidebar_view.rs`
- [ ] 1.2 Adjust top spacing/padding on the sidebar's nav menu so removing the header
      doesn't leave a visual gap or, conversely, crowd the first nav item
- [ ] 1.3 Remove `build_header`/`SidebarHeader` entirely if nothing else uses it, or leave
      an empty header slot if `Sidebar` requires one structurally

## 2. Build and Quality

- [ ] 2.1 `cargo check --workspace`
- [ ] 2.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 2.3 `cargo test --workspace`

## 3. Manual Verification

- [ ] 3.1 Confirm "Libri" appears exactly once, in the title bar under the traffic lights
- [ ] 3.2 Confirm the sidebar's smart-filter nav starts with sensible top spacing
