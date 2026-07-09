## Why

`crates/dtrpg-core/src/services/` has two files that have outgrown a single-file shape: `sdk.rs` is 1203 lines and `collections_sdk.rs` is 765 lines, both well past this repo's documented 700-line file cap. Each file interleaves four distinct concerns — an SDK gateway trait + HTTP/stub implementations, the `LibraryService`/`CollectionsService` adapter logic, response-to-domain mapping helpers, and a large `#[cfg(test)] mod tests` block — with no internal separation. `sdk.rs`'s `map_order_product` alone is ~148 lines against the documented 50-line function limit, mixing publisher resolution, cover-URL resolution, kind/format derivation, file dedup/mapping, and date parsing in one function. This makes both files hard to navigate and risks silent behavior changes when editing one concern accidentally touches another.

## What Changes

- Split `services/sdk.rs` into a `services/sdk/` module directory: `mod.rs` (the `RustSdkLibraryService` struct and its `LibraryService` impl), `gateway.rs` (the `SdkLibraryGateway` trait, `HttpSdkLibraryGateway`, `UnavailableSdkGateway`), `mapping.rs` (`map_order_product` and its helpers, `publisher_lookup`, `product_lookup`, `file_extension_label`, `last_page_from_links`), and `errors.rs` (`map_client_error`, `map_sdk_error`).
- Split `services/collections_sdk.rs` the same way: `mod.rs`, `gateway.rs`, `errors.rs` (no separate `mapping.rs` — this file has no equivalent of `map_order_product`).
- Decompose `map_order_product` (~148 lines) into named helper functions for each independent transform it currently performs inline (publisher resolution, cover-URL resolution, kind/format derivation, file list mapping, year/date parsing), each under the 50-line function guideline.
- Move each file's `#[cfg(test)] mod tests` block to live alongside the code it actually tests, per this repo's Rust convention (unit tests colocated in the module under test) rather than one large trailing block.
- **No behavior change**: this is a structural reorganization only. Public API surface (`RustSdkLibraryService`, `RustSdkCollectionsService`, and what they implement) is unchanged; only internal file/module layout and function decomposition change.

## Capabilities

### New Capabilities
(none — this is an internal code organization change with no user-facing or spec-level behavior)

### Modified Capabilities
(none — no requirement-level behavior changes; this proposal intentionally has no `specs/` deltas)

## Impact

- Affected files: `crates/dtrpg-core/src/services/sdk.rs` (deleted, replaced by `sdk/mod.rs`, `sdk/gateway.rs`, `sdk/mapping.rs`, `sdk/errors.rs`), `crates/dtrpg-core/src/services/collections_sdk.rs` (deleted, replaced by `collections_sdk/mod.rs`, `collections_sdk/gateway.rs`, `collections_sdk/errors.rs`), `crates/dtrpg-core/src/services/mod.rs` (module declarations, if paths change).
- `login.rs` (105 lines) is unaffected — already well within size limits.
- No changes to `dtrpg-ui`, `dtrpg-sdk`, or any other crate; nothing outside `dtrpg-core/src/services/` imports these files' private items, only the public `RustSdkLibraryService`/`RustSdkCollectionsService` types and their trait impls, which keep their existing paths (`crate::services::sdk::RustSdkLibraryService`, `crate::services::collections_sdk::RustSdkCollectionsService`).
- Risk is limited to compile-time breakage (missed `use` after moving items) and accidental behavior drift while decomposing `map_order_product`; both are caught by `cargo check`/`clippy`/existing test suite, which this change must keep green with zero test behavior changes.
