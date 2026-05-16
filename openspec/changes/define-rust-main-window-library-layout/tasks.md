## 1. Rust Layout Spec

- [x] 1.1 Add Rust child change for GPUI main-window library layout behavior
- [x] 1.2 Add `rust-main-window-library-layout` capability delta spec

## 2. GPUI Implementation Planning

- [ ] 2.1 Define GPUI view/controller boundaries for the disclosable search/filter area
- [ ] 2.2 Define Rust account menu state and actions for account identity, token set/reset, and settings navigation
- [ ] 2.3 Define shared Rust browsing state for list/tree and grid presentations
- [ ] 2.4 Define Rust sync status and thumbnail loading state that update without blocking the main window
- [ ] 2.5 Identify existing search/sort/grouping code that can satisfy the shared layout contract

## 3. Verification

- [ ] 3.1 Verify expanded and collapsed filter states preserve active browsing state
- [ ] 3.2 Verify list/tree and grid views share matched items and summary counts
- [ ] 3.3 Verify account menu avoids passive raw token disclosure
- [ ] 3.4 Verify sync status and thumbnail loading do not block main-window interaction
