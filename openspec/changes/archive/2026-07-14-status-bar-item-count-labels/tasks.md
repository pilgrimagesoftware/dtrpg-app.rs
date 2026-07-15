## 1. Status Bar Labels

- [x] 1.1 `library_summary` uses `pluralize(snap.total_count, "item", "items")` instead of
      the bare count
- [x] 1.2 `active_tab_summary` uses `pluralize(snap.active_tab_count, "item", "items")`
      instead of the bare count
- [x] 1.3 Verify i18n: if `pluralize` takes literal English nouns rather than `t!()` keys,
      confirm this matches the existing toolbar usage pattern (or route through `t!()` if
      the toolbar does)

## 2. Build and Quality

- [x] 2.1 `cargo check --workspace`
- [x] 2.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 2.3 `cargo test --workspace`

## 3. Manual Verification

- [x] 3.1 Confirm the status bar reads e.g. "128 items • 2.1 GB" instead of "128 • 2.1 GB"
- [x] 3.2 Confirm a library of exactly 1 item reads "1 item", not "1 items"
