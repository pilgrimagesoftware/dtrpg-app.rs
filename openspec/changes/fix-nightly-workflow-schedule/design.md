## Context

`nightly.yaml` calls the reusable `package.yaml` workflow via `uses: ./.github/workflows/package.yaml` with `secrets: inherit`. `package.yaml` declares `permissions: contents: write` at its own workflow level, but a reusable workflow's effective token permissions are capped by whatever the calling job grants — they can never exceed it. `nightly.yaml` grants only `contents: read` (added by a Copilot code-scanning autofix), so every run since 2026-07-08 fails at startup before any job executes.

## Goals / Non-Goals

**Goals:**
- Restore nightly builds to a passing state.
- Align the schedule to midnight Pacific Time as intended.

**Non-Goals:**
- Changing the trigger type (schedule vs. push) — already settled by a prior commit, out of scope here.
- Handling DST-exact scheduling (GitHub Actions cron has no timezone support; a fixed UTC offset is accepted with a documented ~1 hour seasonal drift).
- Any change to `package.yaml`, `release.yaml`, or other workflows.

## Decisions

- Set `permissions: contents: write` in `nightly.yaml` (matching what `release.yaml` already grants its `package` job) rather than removing the workflow-level `permissions` block entirely — keeps the explicit least-privilege declaration the code-scanning autofix intended, just at the correct level.
- Use `0 7 * * *` for the cron schedule. Midnight PDT (UTC-7, in effect for most of the year) is 07:00 UTC; midnight PST (UTC-8) would be 08:00 UTC. A single fixed cron can't track the seasonal shift, so 07:00 UTC is chosen as the year-round default with a comment noting the PST drift, rather than maintaining two seasonal cron lines.

## Risks / Trade-offs

- [Risk] During Pacific Standard Time (roughly November-March), the nightly build fires at 11pm PT instead of midnight. → Mitigation: documented in a code comment; acceptable since exact timing isn't functionally significant for a nightly pre-release channel.
- [Risk] Broadening permissions could reintroduce the code-scanning alert the original autofix addressed. → Mitigation: `contents: write` is the minimum the `publish` job actually needs (to create/update the GitHub Release); this is a justified, scoped grant, not a revert to unrestricted default permissions.
