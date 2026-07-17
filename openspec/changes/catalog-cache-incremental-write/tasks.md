## 1. Checkpoint Trigger

- [x] 1.1 Pick a checkpoint cadence (e.g. every 5 pages or every 10 seconds, whichever
      fires first) and thread it through the page-receive loop
- [x] 1.2 Spawn a background `save_catalog_cache` call against the current accumulated
      buffer at each checkpoint, without blocking the page-receive loop

## 2. Safety

- [x] 2.1 Confirm checkpoint writes reuse the existing atomic `.tmp`-then-rename write
      path so a crash mid-write cannot leave a corrupt cache file
- [x] 2.2 Confirm checkpoint writes do not race with the final post-fetch save (e.g. via a
      simple in-flight-write guard)

## 3. Build and Quality

- [x] 3.1 `cargo check --workspace`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Start a load against a large library, kill the app after a few checkpoints but
      before completion, and confirm the next launch shows a partial (not empty, not
      fully-stale) cache
- [ ] 4.2 Let a load complete normally and confirm the final cache matches the full
      dataset (checkpointing didn't leave a stale partial file)
