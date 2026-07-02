## 1. Visible-items cache

- [x] 1.1-1.5 — superseded: already implemented by `fix-catalog-hover-jank` as `items_cache: Option<Vec<LibraryItem>>` on `LibraryController`, invalidated eagerly at every mutation site (see that change's archived `tasks.md` for the full site list). `visible_items()`/`visible_items_count()`/`visible_items_slice()` all read the cache; no full re-scan on repeated renders of unchanged state.

## 2. Batch flush in append path

- [x] 2.1-2.2 — superseded: `catalog-live-merge` already replaced per-page catalog mutation during live fetch with a single buffered accumulation + one atomic `set_catalog()` call after the fetch completes. This produces fewer renders than the 500 ms batch-flush originally proposed here (one render for the whole fetch, not one per 500 ms window). The disk-cache pre-population path already delivers its full item list in one call, so no batching is needed there either.

## 3. Batched background load loop

- [x] 3.1-3.4 — not implemented: the `futures`-based 500 ms timer buffering described here would regress `catalog-live-merge`'s already-shipped single-flush behavior. Not applicable.

## 4. Verify

- [x] 4.1 `cargo test --all-features --workspace` — passes (verified against current `develop`, no code changes made under this proposal)
- [ ] 4.2-4.4 — not applicable; no runtime behavior changed under this proposal
