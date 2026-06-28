## 1. Implementation

- [x] 1.1 In `toolbar_view.rs`, change `section_title_for` return type from `&str` to `String`
- [x] 1.2 Update the `Publisher(name)` arm to return `format!("Publisher: {name}")`
- [x] 1.3 Update all other arms to return `"<label>".to_string()` and remove the `.to_string()` call at the use site (line 51)

## 2. Tests

- [x] 2.1 Add unit tests for `section_title_for` covering: `Publisher("Kobold Press")` → `"Publisher: Kobold Press"`, and at least one non-publisher filter returning its existing static label

## 3. Verification

- [x] 3.1 Run `cargo test --all-features --workspace` and confirm all tests pass
