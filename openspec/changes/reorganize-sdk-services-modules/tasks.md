## 1. Pre-flight

- [ ] 1.1 Grep the workspace for any external references to private/internal items in `sdk.rs`
  and `collections_sdk.rs` (not `RustSdkLibraryService`/`RustSdkCollectionsService` themselves)
  to confirm nothing outside these two files depends on them.
- [ ] 1.2 Record the current `cargo test --workspace` pass count (unit + doc tests) as a
  baseline to diff against after the split.

## 2. Split services/sdk.rs

- [ ] 2.1 Create `crates/dtrpg-core/src/services/sdk/` directory.
- [ ] 2.2 Create `sdk/gateway.rs`: move the `SdkLibraryGateway` trait, `HttpSdkLibraryGateway`
  (struct + impls), and `UnavailableSdkGateway` (struct + impls), plus their imports.
- [ ] 2.3 Create `sdk/errors.rs`: move `map_client_error` and `map_sdk_error`, plus their
  imports.
- [ ] 2.4 Create `sdk/mapping.rs`: move `last_page_from_links`, `publisher_lookup`,
  `product_lookup`, `file_extension_label`, and `map_order_product`, plus their imports.
- [ ] 2.5 In `sdk/mapping.rs`, decompose `map_order_product` into `resolve_publisher`,
  `resolve_cover_url` (or a `resolve_product_info`/`resolve_cover_url` pair), `resolve_kind`,
  `resolve_format`, `map_files`, and `resolve_year` helper functions, per design.md's
  decomposition boundaries; `map_order_product` becomes an orchestrator under ~50 lines.
- [ ] 2.6 Create `sdk/mod.rs`: keep the `RustSdkLibraryService` struct and its `LibraryService`
  impl (currently lines ~48-268 of the original `sdk.rs`), plus `pub(crate) use` / `mod`
  declarations wiring up `gateway`, `errors`, `mapping` as needed.
- [ ] 2.7 Move each moved item's existing tests into a `#[cfg(test)] mod tests` colocated in
  the new file that owns the code under test (mapping tests -> `sdk/mapping.rs`; gateway/service
  tests -> `sdk/mod.rs` or `sdk/gateway.rs`, whichever owns the tested code; pagination
  (`last_page_from_links_*`) tests -> `sdk/mapping.rs`). Move shared test helpers (stub
  gateways, `seeded()`/`order_product_item()`/`pagination_links()` builders) alongside the
  tests that use them, duplicating into more than one file only if genuinely needed by tests
  in more than one new file.
- [ ] 2.8 Delete the original `crates/dtrpg-core/src/services/sdk.rs`.

## 3. Split services/collections_sdk.rs

- [ ] 3.1 Create `crates/dtrpg-core/src/services/collections_sdk/` directory.
- [ ] 3.2 Create `collections_sdk/gateway.rs`: move the `SdkCollectionsGateway` trait,
  `HttpSdkCollectionsGateway` (struct + impls), and `UnavailableCollectionsGateway`
  (struct + impls), plus their imports.
- [ ] 3.3 Create `collections_sdk/errors.rs`: move `map_client_error` and `map_sdk_error`
  (this file's versions, distinct from `sdk/errors.rs`'s), plus their imports.
- [ ] 3.4 Create `collections_sdk/mod.rs`: keep the `RustSdkCollectionsService` struct and its
  `CollectionsService` impl, plus `mod` declarations for `gateway` and `errors`.
- [ ] 3.5 Move each moved item's existing tests into a `#[cfg(test)] mod tests` colocated in
  the new file that owns the code under test, same approach as task 2.7.
- [ ] 3.6 Delete the original `crates/dtrpg-core/src/services/collections_sdk.rs`.

## 4. Update module wiring

- [ ] 4.1 Update `crates/dtrpg-core/src/services/mod.rs` if its `mod sdk;`/`mod collections_sdk;`
  declarations or any re-exports need adjustment for the new directory layout (typically
  unchanged, since `sdk.rs` -> `sdk/mod.rs` keeps the same module path).
- [ ] 4.2 Grep for any `use crate::services::sdk::...` / `use crate::services::collections_sdk::...`
  imports elsewhere in the workspace and confirm they still resolve unchanged.

## 5. Verify

- [ ] 5.1 Run `cargo check --workspace --all-targets`.
- [ ] 5.2 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- [ ] 5.3 Run `cargo fmt --all -- --check` and note whether any new diffs are introduced by
  this change specifically (vs. pre-existing repo-wide drift).
- [ ] 5.4 Run `cargo test --workspace` and confirm the pass count matches the task 1.2 baseline
  exactly (same tests, same outcomes) — zero behavior change is the acceptance bar for this
  change.
- [ ] 5.5 Confirm every new file under `services/sdk/` and `services/collections_sdk/` is at or
  under 700 lines, and `map_order_product`'s extracted helpers are each at or under 50 lines.
