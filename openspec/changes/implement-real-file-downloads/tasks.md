## 1. Verify the API contract (blocking — do this before writing implementation code)

- [ ] 1.1 Call `prepare_download` against a real authenticated account (via `dtrpg-sdk`'s existing test harness or a scratch script) using both a top-level order-product id and a per-file `orderProductDownloadId`, and record which one the endpoint actually expects
- [ ] 1.2 Inspect the raw JSON response and identify the exact field path holding the download URL; document it in a comment at the call site
- [ ] 1.3 If the assumptions in design.md's Open Questions turn out wrong, update design.md before proceeding to implementation

## 2. SDK-backed service

- [ ] 2.1 Add `download_item(&self, download_id: u64, dest: &Path) -> Result<(), LibraryServiceError>` to the `LibraryService` trait in `crates/dtrpg-ui/src/services/mod.rs`
- [ ] 2.2 Implement it in the SDK-backed service (`crates/dtrpg-core/src/services/sdk/library/mod.rs` or a sibling module): call `gateway.prepare_download(download_id)`, extract the URL per task 1.2's finding, fetch via `reqwest::blocking` streamed to `{dest}.part`, rename to `dest` on success
- [ ] 2.3 On any fetch/write error, delete `{dest}.part` if it exists and return a `LibraryServiceError`

## 3. Cancellation-aware streaming

- [ ] 3.1 Thread a cancellation check into the chunked read loop (not just a single before/after checkpoint) so a cancelled transfer stops promptly rather than finishing the whole download first
- [ ] 3.2 On cancellation, delete `{dest}.part` before returning

## 4. Wire into the controller

- [ ] 4.1 In `LibraryController::dispatch_download`, replace the hardcoded `Ok(())` outcome with a call to `self.vm.service().download_item(...)` (or the appropriate accessor) run on the background executor, using the same `entry_dir.join(file.name.as_ref())` path resolution already established by `ItemOpener`/`detail-file-list-disk-size`
- [ ] 4.2 Confirm the existing cancel-flag plumbing in `dispatch_download` still applies correctly now that the fetch can take real, non-trivial time (previously it resolved near-instantly)

## 5. Verification

- [ ] 5.1 `cargo build --workspace --all-features`
- [ ] 5.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 5.3 `cargo test --workspace --all-features`
- [ ] 5.4 Launch app: download a real item, confirm the file lands on disk and Open/Reveal in Finder both work afterward
- [ ] 5.5 Launch app: start a download and cancel it mid-transfer, confirm no file (partial or final) remains at the destination
