## Why

`crates/dtrpg-core/src/services/` has two files that have outgrown a single-file shape: `sdk.rs` is 1203 lines and `collections_sdk.rs` is 765 lines, both well past this repo's documented 700-line file cap. Each file interleaves four distinct concerns — an SDK gateway trait + HTTP/stub implementations, the `LibraryService`/`CollectionsService` adapter logic, response-to-domain mapping helpers, and a large `#[cfg(test)] mod tests` block — with no internal separation. `sdk.rs`'s `map_order_product` alone is ~148 lines against the documented 50-line function limit, mixing publisher resolution, cover-URL resolution, kind/format derivation, file dedup/mapping, and date parsing in one function. This makes both files hard to navigate and risks silent behavior changes when editing one concern accidentally touches another.

## What Changes

- Consolidate `services/sdk.rs` and `services/collections_sdk.rs` into a single `services/sdk/` module, organized by domain rather than as two parallel top-level modules:
  - `sdk/library/`: `mod.rs` (`RustSdkLibraryService` + `LibraryService` impl), `gateway.rs` (`SdkLibraryGateway` trait, `HttpSdkLibraryGateway`, `UnavailableSdkGateway`), `mapping.rs` (`map_order_product` and its helpers), `errors.rs` (`map_client_error`, `map_sdk_error`, `map_connection_error`).
  - `sdk/collections/`: `mod.rs` (`RustSdkCollectionsService` + `CollectionsService` impl, plus `extract_member_id`/`extract_product_list_item_id`), `gateway.rs` (`SdkCollectionsGateway` trait, `HttpSdkCollectionsGateway`, `UnavailableCollectionsGateway`), `errors.rs` (same three-function shape as library's).
  - `sdk/connection.rs`: a peer of `library`/`collections` holding what's genuinely shared between them — both domains are backed by the same `dtrpg_sdk::LibraryClient` and go through an identical credential-resolution (keyring/env) and connection-setup sequence. Returns a domain-agnostic `ConnectionError`; each domain's `errors.rs` translates it into its own service error type and wording via `map_connection_error`.
  - `sdk/mod.rs`: re-exports `RustSdkLibraryService`/`RustSdkCollectionsService` so both keep a single public path (`crate::services::sdk::RustSdkLibraryService`, `crate::services::sdk::RustSdkCollectionsService` — the latter's path changes from the old `crate::services::collections_sdk::...`).
- Decompose `map_order_product` (~148 lines) into named helper functions for each independent transform it currently performs inline (publisher resolution, cover-URL resolution, kind/format derivation, file list mapping, year/date parsing), each under the 50-line function guideline.
- Move each file's `#[cfg(test)] mod tests` block to live alongside the code it actually tests, per this repo's Rust convention (unit tests colocated in the module under test) rather than one large trailing block.
- **No behavior change**: this is a structural reorganization only. `RustSdkLibraryService` and `RustSdkCollectionsService`'s behavior (and `RustSdkLibraryService`'s import path) are unchanged; `RustSdkCollectionsService`'s import path moves from `crate::services::collections_sdk::` to `crate::services::sdk::` (updated at its one call site in `app/mod.rs`).

## Capabilities

### New Capabilities
(none — this is an internal code organization change with no user-facing or spec-level behavior)

### Modified Capabilities
(none — no requirement-level behavior changes; this proposal intentionally has no `specs/` deltas)

## Impact

- Affected files: `crates/dtrpg-core/src/services/sdk.rs` and `crates/dtrpg-core/src/services/collections_sdk.rs` (both deleted), replaced by `sdk/{mod.rs,connection.rs}`, `sdk/library/{mod,gateway,mapping,errors}.rs`, `sdk/collections/{mod,gateway,errors}.rs`; `crates/dtrpg-core/src/services/mod.rs` (drops the `collections_sdk` module declaration); `crates/dtrpg-core/src/app/mod.rs` (its one `crate::services::collections_sdk::RustSdkCollectionsService` reference updated to `crate::services::sdk::RustSdkCollectionsService`).
- `login.rs` (105 lines) is unaffected — already well within size limits.
- No changes to `dtrpg-ui`, `dtrpg-sdk`, or any other crate.
- Risk is limited to compile-time breakage (missed `use` after moving items) and accidental behavior drift while decomposing `map_order_product` or extracting the shared `connection.rs`; both are caught by `cargo check`/`clippy`/existing test suite, which this change must keep green with zero test behavior changes.
