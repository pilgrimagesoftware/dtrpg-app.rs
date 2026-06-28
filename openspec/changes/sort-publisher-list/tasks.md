## 1. Implementation

- [x] 1.1 In `dtrpg-ui/src/util/publisher.rs`, change the `publisher_entries` sort comparator from count-descending/name-ascending to case-insensitive name-ascending (`a.name.to_lowercase().cmp(&b.name.to_lowercase())`)
- [x] 1.2 Update the doc comment on `publisher_entries` to reflect the new sort order

## 2. Tests

- [x] 2.1 Add or update a unit test in `publisher.rs` asserting that `publisher_entries` returns publishers in alphabetical order regardless of item count
- [x] 2.2 Add a test asserting that sort is case-insensitive (e.g., "a publisher" before "B Publisher")
- [x] 2.3 Run `cargo test --all-features --workspace` and confirm all tests pass
