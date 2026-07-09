## Context

Both `services/sdk.rs` (1203 lines) and `services/collections_sdk.rs` (765 lines) follow the same internal shape: a gateway trait defining the SDK operation boundary, a real HTTP-backed gateway implementation, a stub/unavailable gateway for the unauthenticated state, a `*Service` struct implementing the app-facing `LibraryService`/`CollectionsService` trait by delegating to the gateway, a handful of free functions translating SDK/HTTP errors into the service's own error type, and (in `sdk.rs` only) response-shape-to-domain-model mapping logic. Tests for all of the above currently live in one trailing `mod tests` per file.

`crates/dtrpg-core/CLAUDE.md`-inherited project style (docs/rust.md) caps files at 700 lines and functions at 50 lines, and specifies unit tests live colocated with the code they test, not in a single trailing block.

## Goals / Non-Goals

**Goals:**
- Bring both files under the 700-line cap via module directories with concern-based splits.
- Bring `map_order_product` under (or close to) the 50-line function guideline via extraction, without changing its output for any existing test case.
- Colocate tests with the code they exercise.
- Zero behavior change: same public types at the same import paths, same runtime behavior, same test outcomes.

**Non-Goals:**
- Changing the `SdkLibraryGateway`/`SdkCollectionsGateway` trait shapes, the `LibraryService`/`CollectionsService` trait impls' method signatures, or any error classification logic.
- Renaming `RustSdkLibraryService`/`RustSdkCollectionsService` or moving them to a different parent module path.
- Touching `login.rs` (105 lines, already appropriately sized) or anything outside `crates/dtrpg-core/src/services/`.
- Adding new abstractions (e.g. a shared generic gateway trait between the two services) — the two services' gateways differ enough (library pagination vs. collections membership operations) that forcing a shared abstraction now would be speculative, not requested.

## Decisions

- **Module directory over flat files with longer names.** `services/sdk/{mod,gateway,mapping,errors}.rs` groups related code under one directory rather than flat files like `services/sdk_gateway.rs`, `services/sdk_mapping.rs` — makes the four-way split visually obvious in a file tree and matches how `dtrpg-ui`'s `ui/<feature>/{state,data}.rs` convention already groups feature-scoped concerns (per this repo's `CLAUDE.md`).
- **`mod.rs` keeps the service struct + trait impl, not just re-exports.** The struct and its `LibraryService`/`CollectionsService` impl are the module's primary public surface and the thing most readers of `services::sdk` are looking for; putting it in `mod.rs` (with `gateway`/`mapping`/`errors` as clearly secondary, supporting modules) reads better than a `mod.rs` that's just `pub use` statements plus the real logic hidden in a same-named submodule.
- **`mapping.rs` only exists for `sdk`, not `collections_sdk`.** `collections_sdk.rs` has no equivalent of `map_order_product` — its gateway calls already return data close to what `CollectionsService` needs, only error mapping is non-trivial. Forcing an empty or trivial `mapping.rs` there would be a distinction without a difference; `errors.rs` is sufficient.
- **`map_order_product` decomposition boundaries follow its existing comment-delimited sections, not an arbitrary line-count split**: extract `resolve_publisher(attributes, publishers) -> String`, `resolve_cover_url(product_info) -> Option<String>` (or fold into a `resolve_product_info`/`resolve_cover_url` pair matching the existing product-lookup-then-cover-url two-step), `resolve_kind(attributes) -> String`, `resolve_format(files) -> String`, `map_files(files) -> Vec<LibraryItemFile>` (the dedup + per-file mapping), and `resolve_year(attributes) -> u32`. `map_order_product` itself becomes an orchestrator calling each in sequence and assembling the final `LibraryItem`. This mirrors the function's existing comment structure, so the split documents itself rather than requiring new prose.
- **Tests move with their subject, not into a shared `tests.rs`.** Mapping tests (`map_order_product_*`, `last_page_from_links_*`) move into `mapping.rs`'s own `#[cfg(test)] mod tests`; gateway/pagination tests (`sdk_service_*`, `count_items_*`) move into `mod.rs`'s (or `gateway.rs`'s, whichever owns the code under test) `#[cfg(test)] mod tests`. Test helper structs (`seeded()`, stub gateways) move alongside the tests that use them; if a helper is needed by tests in more than one new file, it's duplicated rather than introducing a `test_support` module for two call sites (per this repo's reuse-before-duplicating threshold of "multiple" meaning more than two).

## Risks / Trade-offs

- [Moving code across files risks a mechanical error — a missed `use`, a dropped `pub(crate)`, or a test accidentally not carried over] → Mitigation: `cargo check --workspace --all-targets` and `cargo test --workspace` must both pass with identical test counts (before/after) before this change is considered done; tasks.md requires diffing the test count explicitly.
- [Decomposing `map_order_product` into six-plus helper functions could obscure the original function's implicit ordering dependencies, if any exist] → Mitigation: read through the existing function fully before extracting (already done for this design) — the transforms are independent (publisher/cover/kind/format/files/year each read only from `attributes`/`item`/lookup maps, none depend on another's output), so extraction is safe; the existing tests (`map_order_product_*`) pin the combined behavior regardless.
- [A module directory split could break relative-path assumptions elsewhere, e.g. `crate::services::sdk::SomeType`] → Mitigation: grep for all external references to items in both files before moving (task 1 in tasks.md); `RustSdkLibraryService`/`RustSdkCollectionsService` keep the same fully-qualified path (`crate::services::sdk::RustSdkLibraryService` still resolves once `sdk.rs` becomes `sdk/mod.rs`), so only genuinely private/internal items are at risk, and those have no external callers by definition.
