## 1. Catalog Rows

- [ ] 1.1 Replace `.tooltip()` on the catalog list/thumbs/grid title cell with
      `HoverCard::new(...)` rendering title, publisher, and status
- [ ] 1.2 Configure `HoverCard` open/close delay to match the existing tooltip feel
      (no noticeable lag on hover)

## 2. Detail Panel

- [ ] 2.1 Identify detail panel rows with multi-line-worthy truncated content and
      convert them to `HoverCard`

## 3. Build and Quality

- [ ] 3.1 `cargo check --workspace`
- [ ] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Hover a truncated catalog title and confirm the `HoverCard` shows title,
      publisher, and status
- [ ] 4.2 Confirm the card dismisses when the mouse leaves the row
