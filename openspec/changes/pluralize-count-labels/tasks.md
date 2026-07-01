## 1. Utility

- [x] 1.1 Create `crates/dtrpg-ui/src/util/pluralize.rs` with `pub fn pluralize(count: usize, singular: &str, plural: &str) -> String`; add unit tests for 0, 1, and n > 1
- [x] 1.2 Re-export `pluralize` from `crates/dtrpg-ui/src/util/mod.rs`

## 2. Toolbar count label

- [x] 2.1 Import `pluralize` in `toolbar_view.rs` and replace the `count_label` format strings so "item/items", "publisher item/publisher items", and "filtered/filtered" all use it

## 3. Sidebar section suffix counts

- [x] 3.1 In `sidebar_view.rs`, replace `collections_count.to_string()` in the suffix with `pluralize(collections_count, "collection", "collections")`
- [x] 3.2 In `sidebar_view.rs`, replace `publishers_count.to_string()` in the suffix with `pluralize(publishers_count, "publisher", "publishers")`

## 4. Sidebar footer count

- [x] 4.1 In `sidebar_view.rs` `build_footer`, replace `format!("{total_count} titles")` with `pluralize(total_count, "title", "titles")`
