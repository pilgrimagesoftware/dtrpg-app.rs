## Why

Issue #474. Reporting always bounced the user to a browser page, with no way to
attach the log or a screenshot. `gh` can file the issue directly when present,
and Knot already solved attachments within GitHub's limits.

## What Changes

- Submit through `gh issue create` when `gh` is installed and signed in; fall
  back to the browser handoff otherwise.
- An "Attach application log" option: the tail of the newest log goes into the
  issue body (collapsed) on the `gh` path; the browser path names the log file.
- An "Attach screenshots" option: GitHub has no upload API, so the body names
  the files and the file manager reveals them to drag onto the issue page.

## Impact

- `crates/dtrpg-core/src/report.rs` (new), `report_feature_bug_window_view.rs`,
  `i18n/{en,de,fr}.yaml`.
- Amends `shared-report-feature-bug`, which required browser handoff only.
