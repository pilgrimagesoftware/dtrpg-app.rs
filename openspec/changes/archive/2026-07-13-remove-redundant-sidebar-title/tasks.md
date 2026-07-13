## 1. Remove Duplicate Wordmark

- [x] 1.1 Remove the `t!("sidebar.app_name")` child from `build_header` in
      `sidebar_view.rs`
- [x] 1.2 Adjust top spacing/padding on the sidebar's nav menu so removing the header
      doesn't leave a visual gap or, conversely, crowd the first nav item
- [x] 1.3 Remove `build_header`/`SidebarHeader` entirely if nothing else uses it, or leave
      an empty header slot if `Sidebar` requires one structurally

## 2. Build and Quality

- [x] 2.1 `cargo check --workspace`
- [x] 2.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 2.3 `cargo test --workspace`

## 3. Manual Verification

- [x] 3.1 Confirm "Libri" appears exactly once, in the title bar under the traffic lights
- [x] 3.2 Confirm the sidebar's smart-filter nav starts with sensible top spacing
