## Context

`LibraryController::dispatch_download` (from `download-queue-concurrency-control`) already handles slot accounting, activity panel entries, cancellation, and completion/error routing — its `outcome` is currently hardcoded to `Ok(())`. This change only needs to produce a real `Result<(), String>` from an actual file transfer; nothing about the queue, concurrency limit, or UI changes.

The SDK (`dtrpg-sdk`) exposes `LibraryClient::prepare_download(order_product_id: u64) -> Result<serde_json::Value, ClientError>`, mapping to `GET /{api_version}/order_products/{order_product_id}/prepare`. Its doc comment states plainly: *"the schema for this endpoint has not yet been formally defined by the API contract."* Everything downstream of this call (which JSON field holds the URL, whether `order_product_id` is the per-file `orderProductDownloadId` or the per-entry id) is an assumption until verified against a real account, or until `dtrpg-api` formalizes the contract.

`LibraryController::dispatch_thumbnail_fetch` already establishes the pattern for fetching bytes from a background executor thread: `reqwest::blocking::get` inside `async_cx.background_executor().spawn(...)`, because GPUI's executors aren't a Tokio runtime and don't drive `.await` on a bare `reqwest::get`.

## Goals / Non-Goals

**Goals:**
- `LibraryService::download_item` resolves a download URL via `prepare_download`, fetches the bytes, and writes them to `{entry_dir}/{file_name}` — never leaving a corrupt/truncated file at the final name.
- Cancellation (already wired via `dispatch_download`'s `cancel_flag`) results in no file, or only a `.part` file, at the final path — never a partial file masquerading as complete.
- The SDK-facing call happens on a background thread, consistent with the thumbnail-fetch pattern, so it never blocks the UI.

**Non-Goals:**
- No byte-level progress reporting (activity panel still shows started/complete/error only — an explicit non-goal already recorded in `download-queue-concurrency-control`'s design).
- No resume of a partially-downloaded file after an app crash — `.part` files from a crashed session are simply left behind (a future cleanup task, not this change).
- No change to `prepare_download`'s SDK signature or the raw JSON response type — this change consumes it as-is and documents the assumed field name; formalizing the contract properly is `dtrpg-api`/`dtrpg-sdk` work outside this repo.

## Decisions

### `LibraryService::download_item(&self, download_id: u64, dest: &Path) -> Result<(), LibraryServiceError>`

Mirrors `get_item(&self, id: u64)`'s shape — synchronous, called from a background executor thread the same way `get_item`/`list_items` already are via the gateway's internal `block_on`. `dest` is the full final file path (`entry_dir.join(file_name)`, resolved by the caller using the same convention `ItemOpener::open_item` and `detail-file-list-disk-size` already establish), not just a directory — keeps the trait method single-purpose (fetch this URL, write to this exact path) and the `.part`-then-rename bookkeeping local to its implementation.

_Alternative considered_: Return the raw bytes (`Result<Vec<u8>, LibraryServiceError>`) and let the caller write the file, mirroring how thumbnail bytes flow back through `CoverCache`. Rejected — download payloads can be large (a full PDF/ZIP, not a small cover image), so buffering the whole file in memory before writing is wasteful; writing to disk incrementally at the service boundary is worth breaking the thumbnail symmetry for.

### Stream the HTTP response body directly to a `.part` file, then rename

Uses `reqwest::blocking`'s `Response::copy_to` (or a manual chunked read loop) writing into `{dest}.part`, then `std::fs::rename` to `dest` only after the copy succeeds fully. A crash, network drop, or cancellation mid-transfer leaves only the `.part` file — `dest` never exists until the transfer is provably complete. This directly implements the mitigation `download-queue-concurrency-control`'s design already flagged as a risk ("write to a `.part` temp file and rename on completion; delete `.part` on cancel or error") but left out of scope for that change.

_Alternative considered_: Download into memory first (`response.bytes()`), then write the whole buffer at once. Rejected for the same large-payload reason as above — streaming avoids holding an entire file in RAM.

### Cancellation deletes the `.part` file

`dispatch_download`'s cancellation path already skips marking the item `Downloaded` when `cancel_flag` is set; this change adds an explicit `.part` file cleanup step so a cancelled download doesn't silently leave orphaned partial data. Implemented as: the download task checks `cancel_flag` between chunks (not just at the single checkpoint `dispatch_download` currently has) and deletes `{dest}.part` before returning if set.

_Alternative considered_: Only check the cancellation flag once, after the full transfer (matching `dispatch_download`'s current single-checkpoint pattern). Rejected — for a large file this could mean many seconds of wasted bandwidth after the user already clicked cancel; a mid-transfer check is cheap to add (checked once per chunk read) and meaningfully improves cancel responsiveness.

## Risks / Trade-offs

- [Risk] `prepare_download`'s response schema is unverified — the field name assumed to hold the download URL (e.g. `data.attributes.download_url`, exact path unknown) may be wrong, or the endpoint may require `order_product_id` when this change assumes the per-file `orderProductDownloadId` (or vice versa). → Mitigation: implementation must include a manual verification step against a real account before merging (see tasks.md); if the assumption is wrong, this surfaces immediately as every download failing, not a silent data-correctness bug.
- [Risk] No retry/backoff on transient network failures — a flaky connection surfaces as a download error requiring the user to manually re-click download. → Accepted for this change; retry logic is a reasonable follow-on once the basic path is proven to work.
- [Trade-off] `.part` files from a crashed session (not a clean cancel) are never cleaned up automatically. → Accepted per Non-Goals; a startup sweep for orphaned `.part` files is a separate, small follow-on if it proves annoying in practice.
- [Risk] `prepare_download`'s response carries `status`/`progress` fields suggesting the watermarking job can be asynchronous (not yet observed in a non-`"Complete"` state). This implementation always fetches `data.attributes.url` immediately without checking `status`, so a file that isn't pre-cached server-side may fail or return an incomplete/error response instead of the real file. → Mitigation: not implemented in this change (never observed in testing); if it surfaces in practice, the fix is a bounded poll-and-retry on `prepare_download` until `status == "Complete"` before fetching `url`.

## Open Questions (resolved)

- ~~Does `prepare_download` expect the top-level entry's order-product id, or the per-file `orderProductDownloadId`?~~ **Resolved**: neither alone — it needs `order_product_id` *and* a required `index` (the file's position within the entry's file list, matching `OrderProductFile::index`). Verified live: 403 "Unable to prepare a file without an index" without it, 200 with it. Fixed upstream in `dtrpg-sdk` v1.0.0 (`prepare-download-file-index`), which added `index` as a required second parameter to `prepare_download`. `LibraryItemFile` gained a matching `index: u32` field, populated from `OrderProductFile::index` in the mapping layer.
- ~~What JSON path in the `prepare_download` response holds the actual download URL?~~ **Resolved**: `data.attributes.url`. It is not a direct file link — it's a watermarking portal URL (`watermark.drivethrurpg.com/download_portal.php?...`) that 302-redirects to a pre-signed object-storage URL with a short expiry (observed: `X-Amz-Expires=30`, i.e. 30 seconds). This means the fetch must happen immediately after `prepare_download` returns, with no caching or delay of the URL — `download_item` does this in one call, never persisting the URL between calls. The response also carries `status`/`progress` fields suggesting the watermarking job can be asynchronous for files not yet cached (`"status":"Complete"`/`"progress":"1.00"` was observed for an already-cached file); polling for a `"Processing"` state is not implemented in this change — see Risks.
