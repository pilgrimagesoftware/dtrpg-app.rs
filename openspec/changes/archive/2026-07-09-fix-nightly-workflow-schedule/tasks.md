## 1. Fix nightly.yaml

- [x] 1.1 Change `permissions: contents: read` to `permissions: contents: write` in `.github/workflows/nightly.yaml`.
- [x] 1.2 Change the cron schedule from `0 0 * * *` to `0 7 * * *`, adding a comment that this targets midnight Pacific Daylight Time and drifts ~1 hour during Pacific Standard Time.

## 2. Verify

- [ ] 2.1 Trigger the workflow manually via `workflow_dispatch` (or `gh workflow run nightly.yaml`) on `develop` and confirm the run completes successfully (no `startup_failure`, `publish` job updates the `nightly` release).
- [ ] 2.2 Confirm the next scheduled run (or a subsequent manual check) fires at the corrected time and still succeeds.
