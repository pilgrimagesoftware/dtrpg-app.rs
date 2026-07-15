## Why

`enqueue_download` (added by `download-queue-concurrency-control`) only flips an item's status to `Downloaded` — no bytes are ever written to disk. Every downstream action that assumes a downloaded file exists (Open, Reveal in Finder) fails with "file not found" the moment a user actually tries to use what the app just told them was downloaded. This was a known, deliberately scoped-out gap ("the actual HTTP fetch is a follow-on task"); it's now blocking real usage.

## What Changes

- Add `LibraryService::download_item`, backed by the SDK's `prepare_download` endpoint plus an HTTP fetch of the resulting file, writing to a `.part` temp file and renaming to the final filename on success.
- Wire `LibraryController::dispatch_download` to call it (replacing the current always-`Ok(())` placeholder), so a successful download leaves a real file at `{storage_root}/items/{entry_id}/{file_name}`, and a failure or cancellation leaves no partial file behind.
- No change to the queue, concurrency limiting, activity panel wiring, or cancellation UI — those already exist and this change only replaces the no-op fetch body they call into.

## Capabilities

### New Capabilities

- `real-file-download-transfer`: fetching a downloaded item's actual file bytes from the API and writing them to disk, including partial-download cleanup on failure or cancellation. This is a companion to `download-queue` (from `download-queue-concurrency-control`, not yet archived) — that capability's completion/error/cancellation requirements describe the queue-level contract; this one describes what actually satisfies "the download completed" underneath it.

### Modified Capabilities

<!-- none — depends on download-queue-concurrency-control merging/archiving first; see Impact -->

## Impact

- **Sequencing**: assumes `download-queue-concurrency-control` has already merged (its queue, activity panel wiring, and cancellation UI are the caller of `download_item`). If it hasn't yet, apply that change first.
- `dtrpg-sdk`/`dtrpg-core`: `crates/dtrpg-core/src/services/sdk/library/mod.rs` (or a new sibling module) gains a `download_item` implementation calling `LibraryClient::prepare_download` then fetching the resulting URL.
- `crates/dtrpg-ui/src/services/mod.rs`: `LibraryService` trait gains a `download_item(&self, order_product_id: u64, dest: &Path) -> Result<(), LibraryServiceError>` method (exact signature TBD — see design's open questions on the per-file vs per-entry id).
- `crates/dtrpg-ui/src/controllers/library.rs`: `dispatch_download`'s `outcome` is produced by calling the service instead of `Ok(())`.
- **Depends on an unresolved API contract**: `LibraryClient::prepare_download`'s response is currently an untyped `serde_json::Value` ("the schema for this endpoint has not yet been formally defined by the API contract"). This change either formalizes that contract first (via `dtrpg-api`'s OpenSpec chain, per this repo's preferred pattern) or parses the JSON defensively with a documented assumption — see design.md.
