## 1. Relocate the shared copy-button helper

- [ ] 1.1 Create `crates/dtrpg-ui/src/ui/copyable_value.rs` with a `pub(crate) fn
  copyable_value(field_id: SharedString, value: impl Into<SharedString>) -> AnyElement`,
  moved verbatim from `detail_panel_view.rs` (no behavior change, just relocation +
  visibility)
- [ ] 1.2 Register the new module (`pub(crate) mod copyable_value;`) in `ui/mod.rs`
- [ ] 1.3 Remove the private `copyable_value` from `detail_panel_view.rs` and update its call
  site(s) to import from the new module
- [ ] 1.4 Run `cargo check -p dtrpg-ui` to confirm the move didn't break the detail panel's
  existing usage

## 2. Add the copy button to alert history rows

- [ ] 2.1 In `alert_history_view.rs::render_entry_row`, replace the plain error-message `div`
  with a call to `copyable_value`, passing a unique `field_id` derived from `entry.id` (e.g.
  `format!("alert-history-message-{}", entry.id)`) and `entry.message.clone()` as the value
- [ ] 2.2 Confirm the button only appears on hover of that row (via `copyable_value`'s existing
  `group`/`group_hover` behavior) and does not shift the row's layout/height when hidden
- [ ] 2.3 Reuse the existing `detail.copy_tooltip` i18n key for the button's tooltip (no new
  translation key needed)

## 3. Verify

- [ ] 3.1 Run `cargo check --workspace --all-targets`
- [ ] 3.2 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 3.3 Run `cargo fmt --all -- --check`
- [ ] 3.4 Run `cargo test --workspace` and confirm no new failures (pre-existing unrelated
  i18n/datetime test failures, if still present, are not caused by this change)
- [ ] 3.5 Manually trigger an error activity, open the alert history panel, hover a row, click
  the copy button, and paste the clipboard contents somewhere to confirm the full untruncated
  error message was copied (not the visually truncated display text)
- [ ] 3.6 Manually confirm the copy button is invisible when not hovering a row and appears on
  hover, matching the detail panel's existing copy-button behavior
