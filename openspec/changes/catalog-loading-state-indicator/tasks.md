## 1. Catalog List Loading

- [ ] 1.1 Add a `loading: bool` (or shared read of `LibraryController`'s existing loading
      flag) to `CatalogListDelegate`
- [ ] 1.2 Override `CatalogListDelegate::loading()` to return that flag
- [ ] 1.3 Confirm the built-in skeleton view replaces the empty table during initial load

## 2. Sidebar Sections

- [ ] 2.1 Publishers section body shows a compact loading indicator while publishers are
      not yet known
- [ ] 2.2 Collections section body shows a compact loading indicator while collections
      have not yet loaded

## 3. Build and Quality

- [ ] 3.1 `cargo check --workspace`
- [ ] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Cold start with no cache: list presentation shows skeleton rows, not an empty
      table
- [ ] 4.2 Sidebar shows a loading indicator, not "no publishers", before the first
      catalog page or collections response arrives
