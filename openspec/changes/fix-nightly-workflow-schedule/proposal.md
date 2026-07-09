## Why

`nightly.yaml` runs on a `schedule: cron: "0 0 * * *"` trigger, but every scheduled run since 2026-07-08 has failed with `startup_failure`. The cause is commit `2e26a0f` (a Copilot code-scanning autofix), which added a workflow-level `permissions: contents: read` block to `nightly.yaml`. That caps the token available to the called reusable workflow (`package.yaml`) below the `contents: write` its `publish` job needs to push GitHub Release assets, and GitHub fails the run before any job starts. Separately, `0 0 * * *` runs at midnight UTC, not midnight Pacific as intended.

## What Changes

- Grant `contents: write` permission from `nightly.yaml` to the `package` job so the reusable `package.yaml` workflow can publish/update the `nightly` GitHub Release.
- Change the cron schedule from `0 0 * * *` (midnight UTC) to `0 7 * * *` (midnight Pacific Daylight Time / 07:00 UTC), with a comment noting the ~1 hour drift during Pacific Standard Time (GitHub Actions cron has no timezone support).

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `release-packaging`: the "Nightly pre-release from develop" requirement (defined in `openspec/changes/add-release-packaging-workflow/specs/release-packaging/spec.md`, not yet archived) currently describes push-triggered nightly builds; actual behavior is schedule-triggered. This change fixes that schedule (time-of-day and permissions) rather than the trigger type, which was already changed by a prior, separate commit outside this proposal's scope.

## Impact

- Affected file: `dtrpg-app/rust/.github/workflows/nightly.yaml` only.
- No changes to `package.yaml`, application source, or other workflows.
- Once merged to `develop`, the next scheduled run (or a manual `workflow_dispatch`) verifies the fix.
