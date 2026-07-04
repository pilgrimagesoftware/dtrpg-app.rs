## 1. Implement popover date-added row

- [ ] 1.1 In `item_popover_view.rs`, import `format_relative` and `format_absolute` from `crate::util::datetime`, plus `SharedString` and `Tooltip` as needed.
- [ ] 1.2 Replace the `TODO: add updated date HUMAN READABLE` comment with a `.when(item.date_added.is_some(), |list| ...)` block that adds a `DescriptionItem` using `t!("detail.field_added")`, the relative-date string as the visible value, and a tooltip showing the absolute date.
- [ ] 1.3 Match the detail panel's tooltip pattern (unique element id derived from `item.id`) so multiple popovers/tooltips don't collide.

## 2. Verify

- [ ] 2.1 Run `cargo check --all-targets` and `cargo clippy --all-targets --all-features -- -D warnings` for the `dtrpg-ui` crate.
- [ ] 2.2 Run `cargo fmt --all -- --check`.
- [ ] 2.3 Manually confirm (or add a unit test if practical) that an item with `date_added: None` renders the popover without a date-added row.
