## 1. Pre-flight

- [x] 1.1 Grep the workspace for any external references to private/internal items in `sdk.rs`
  and `collections_sdk.rs` (not `RustSdkLibraryService`/`RustSdkCollectionsService` themselves)
  to confirm nothing outside these two files depends on them.
  - Confirmed: only `RustSdkLibraryService::{from_keyring_with_tokens,unauthenticated}` and
    `RustSdkCollectionsService::{from_keyring_with_tokens,unauthenticated}` are referenced
    externally, both from `dtrpg-core/src/app/mod.rs`.
- [x] 1.2 Record the current `cargo test --workspace` pass count (unit + doc tests) as a
  baseline to diff against after the split.
  - Baseline: dtrpg-core 36 unit tests, dtrpg-ui 189 unit tests + 10 doc-tests.

## 2. First pass: two parallel top-level modules (superseded)

- [x] 2.1 Split `sdk.rs` into `services/sdk/{mod,gateway,mapping,errors}.rs` and
  `collections_sdk.rs` into `services/collections_sdk/{mod,gateway,errors}.rs`, each with
  `map_order_product` decomposed into helpers and tests colocated per file.
  - Superseded by section 3: after implementing and verifying this layout, feedback was to
    consolidate everything SDK-related under a single `sdk` module organized by domain, with
    genuinely shared code (the connection-building logic, which turned out to be duplicated
    verbatim between the two gateways) hoisted to a peer module. Reworked before merging;
    the intermediate two-top-level-module state was never committed as final.

## 3. Consolidate into a single `sdk` module organized by domain

- [x] 3.1 Restructure into `services/sdk/{mod.rs,connection.rs,library/,collections/}`:
  `library/{mod,gateway,mapping,errors}.rs` (moved from the section-2 `sdk/` layout, gateway
  slimmed to use `connection.rs`) and `collections/{mod,gateway,errors}.rs` (moved from the
  section-2 `collections_sdk/` layout, same gateway slimming). `sdk/mod.rs` re-exports
  `RustSdkLibraryService` and `RustSdkCollectionsService`.
- [x] 3.2 Extract `sdk/connection.rs`: `SdkConnection { client, runtime }`, a domain-agnostic
  `ConnectionError` enum, `connect_from_keyring_with_tokens`/`connect_from_environment`
  builders, and the shared `build_connection` helper — replacing the near-identical
  `HttpSdkLibraryGateway`/`HttpSdkCollectionsGateway` `from_keyring_with_tokens`/
  `from_environment`/`build` bodies (~90 lines each) with thin per-domain wrappers.
- [x] 3.3 Add `map_connection_error` to `library/errors.rs` and `collections/errors.rs`,
  translating `ConnectionError` into each domain's own error type and exact wording
  (verified against the original inline messages, including the "library"/"collections"
  word difference and identical kind classification).
- [x] 3.4 Remove `crates/dtrpg-core/src/services/collections_sdk.rs`/`collections_sdk/` and
  the old flat `services/sdk/{gateway,errors}.rs`; drop `pub mod collections_sdk;` from
  `services/mod.rs`.
- [x] 3.5 Update the one external call site: `app/mod.rs`'s
  `crate::services::collections_sdk::RustSdkCollectionsService` -> `crate::services::sdk::
  RustSdkCollectionsService`. `RustSdkLibraryService`'s path is unchanged.

## 4. Verify

- [x] 4.1 Run `cargo check --workspace --all-targets`. — Passes clean.
- [x] 4.2 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`. — Passes
  clean, zero warnings.
- [x] 4.3 Run `cargo +nightly fmt --all -- --check`. — This repo's `rustfmt.toml` sets
  `unstable_features = true` (per `docs/rust.md`, formatting requires `+nightly`, not stable
  `cargo fmt`). Ran `cargo +nightly fmt --all`, then `-- --check` passes clean, zero diffs.
- [x] 4.4 Run `cargo test --workspace` and confirm the pass count matches the task 1.2
  baseline exactly.
  - dtrpg-core: 36 passed, 0 failed. dtrpg-ui: 189 passed + 10 doc-tests, 0 failed. Matches
    baseline exactly.
- [x] 4.5 Confirm every file is at or under 700 lines, and `map_order_product`'s extracted
  helpers are each at or under 50 lines.
  - `sdk/connection.rs` 105, `sdk/mod.rs` 14, `library/errors.rs` 99, `library/gateway.rs` 90
    (down from 157 pre-consolidation), `library/mapping.rs` 640 (mostly tests),
    `library/mod.rs` 473, `collections/errors.rs` 96, `collections/gateway.rs` 228 (down from
    295), `collections/mod.rs` 405 — all under 700. Extracted `map_order_product` helpers all
    under 50 lines; `map_order_product` itself is 54 (see design.md's noted exception).
