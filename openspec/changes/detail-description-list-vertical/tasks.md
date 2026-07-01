## 1. Implementation

- [x] 1.1 In `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`, in `render_metadata_table`, change `DescriptionList::new()` to `DescriptionList::vertical()`

## 2. Build and Verify

- [x] 2.1 Run `cargo check --workspace` -- no errors
- [x] 2.2 Run `cargo clippy --all-targets --all-features -- -D warnings` -- no new warnings
- [ ] 2.3 Open the detail panel for any item and confirm each metadata label appears above its value
