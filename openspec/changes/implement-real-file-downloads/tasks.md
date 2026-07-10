## 1. Verify the API contract (blocking — do this before writing implementation code)

- [x] 1.1 Call `prepare_download` against a real authenticated account (via `dtrpg-sdk`'s existing test harness or a scratch script) using both a top-level order-product id and a per-file `orderProductDownloadId`, and record which one the endpoint actually expects — confirmed: `order_product_id` + a required `index` (the file's position within the entry's file list, matching `OrderProductFile::index`), not `orderProductDownloadId`. Fixed upstream in `dtrpg-sdk` v1.0.0 (`prepare-download-file-index`).
- [x] 1.2 Inspect the raw JSON response and identify the exact field path holding the download URL; document it in a comment at the call site — `data.attributes.url`, a watermark-portal URL that 302-redirects to a pre-signed object-storage URL with a ~30s expiry
- [x] 1.3 If the assumptions in design.md's Open Questions turn out wrong, update design.md before proceeding to implementation — both open questions resolved and documented in `download.rs`'s module doc comment

## 2. SDK-backed service

- [x] 2.1 Add `download_item(&self, order_product_id: u64, index: u32, dest: &Path, cancel: &AtomicBool) -> Result<(), LibraryServiceError>` to the `LibraryService` trait in `crates/dtrpg-ui/src/services/mod.rs` (signature adjusted from the original `download_id: u64` draft once task 1 confirmed the real parameter shape)
- [x] 2.2 Implement it in the SDK-backed service (new sibling module `crates/dtrpg-core/src/services/sdk/library/download.rs`): call `gateway.prepare_download(order_product_id, index)`, extract `data.attributes.url`, fetch via `reqwest::blocking` streamed to `{dest}.part`, rename to `dest` on success
- [x] 2.3 On any fetch/write error, delete `{dest}.part` if it exists and return a `LibraryServiceError`

## 3. Cancellation-aware streaming

- [x] 3.1 Thread a cancellation check into the chunked read loop (not just a single before/after checkpoint) so a cancelled transfer stops promptly rather than finishing the whole download first
- [x] 3.2 On cancellation, delete `{dest}.part` before returning

## 4. Wire into the controller

- [x] 4.1 In `LibraryController::dispatch_download`, replace the hardcoded `Ok(())` outcome with a call to `service_arc.download_item(...)` run on the background executor, resolving `dest` via `StorageConfig::path_for_item(&item_id).join(file.name.as_ref())` (same convention `ItemOpener` already uses). Downloads the entry's first file — multi-file entries have no per-file download UI yet, noted as a scope limitation in the doc comment, not a silent bug.
- [x] 4.2 Confirm the existing cancel-flag plumbing in `dispatch_download` still applies correctly now that the fetch can take real, non-trivial time — the same `cancel_flag: Arc<AtomicBool>` is now passed by reference into `download_item`, so a mid-transfer cancel is observed inside the streaming loop itself (task 3.1), not just at the single before/after checkpoint the controller already had

## 5. Verification

- [x] 5.1 `cargo build --workspace --all-features`
- [x] 5.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] 5.3 `cargo test --workspace --all-features` (207 dtrpg-ui tests + 11 doc-tests + 39 dtrpg-core tests, all pass)
- [ ] 5.4 Launch app: download a real item, confirm the file lands on disk and Open/Reveal in Finder both work afterward
- [ ] 5.5 Launch app: start a download and cancel it mid-transfer, confirm no file (partial or final) remains at the destination
