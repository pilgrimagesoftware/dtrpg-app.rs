## 1. Replace hand-rolled rows with DescriptionList

- [x] 1.1 In `settings_account_view.rs`, replace the Email row's `div().flex().items_baseline()` construction with a `DescriptionList::horizontal().bordered(false)` containing a single `DescriptionItem`.
- [x] 1.2 Do the same for the conditional API Key row, added to the same `DescriptionList` (or a second one immediately below) so both rows share one label column width.
- [x] 1.3 Set `label_width` on the list wide enough to fit "API Key" (the longer label) plus a small margin, right-aligning both labels within that column.

## 2. Preserve existing styling

- [x] 2.1 Wrap each label in a `div().text_right().text_xs().text_color(colors.text_secondary).child(t!(...))` (or equivalent) passed as the `DescriptionItem` label, so label color/size match the current row styling rather than the component's default label style.
- [x] 2.2 Wrap each value in a `div().text_xs().font_family(MONOSPACE_FONT).text_color(colors.text_tertiary).child(...)` passed as the `DescriptionItem` value, preserving the monospace treatment from `settings-api-key-monospace`.
- [x] 2.3 Compare row spacing/padding against the current layout; adjust `size`/gaps if `DescriptionList`'s default row padding visibly changes the section's spacing.

## 3. Verification

- [ ] 3.1 Manually verify: open Settings while signed in with an email and an API key hint present; confirm both labels right-align to the same column edge and label/value text is vertically aligned within each row.
- [ ] 3.2 Manually verify: open Settings while signed in without an API key hint (Email row only); confirm the single row still renders correctly.
- [x] 3.3 Run `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, and `cargo test --all-features --workspace`.
