## 1. Implementation

- [ ] 1.1 In `toolbar_view.rs`, change `section_title_for` return type from `&str` to `String`
- [ ] 1.2 Update the `Publisher(name)` arm to return `format!("Publisher: {name}")`
- [ ] 1.3 Update all other arms to return `"<label>".to_string()` and remove the `.to_string()` call at the use site (line 51)

## 2. Tests

- [ ] 2.1 Add unit tests for `section_title_for` covering: `Publisher("Kobold Press")` → `"Publisher: Kobold Press"`, and at least one non-publisher filter returning its existing static label

## 3. Verification

- [ ] 3.1 Run `cargo test --all-features --workspace` and confirm all tests pass
