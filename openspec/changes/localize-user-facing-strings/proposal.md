## Why

`locale-manager` (see `openspec/specs/locale-manager/spec.md`) makes the app locale-aware at
startup, and nearly every user-facing string in `src/ui/` already routes through `t!()` with
entries in `i18n/en.yaml`, `de.yaml`, and `fr.yaml`. A handful of Activity Panel labels were
missed and are still hardcoded English `format!()`/string literals, so a German or French user
sees a mix of translated UI chrome and untranslated English progress messages in the same
panel. A fresh code scan (ripgrep across `crates/dtrpg-ui/src/`, cross-checked against every
locale file) found the exact remaining set below — this is a closeout of the earlier
`i18n-localization` effort, not a new i18n subsystem.

## What Changes

- Replace the following hardcoded strings with `t!()` lookups and matching keys in
  `i18n/en.yaml`, `de.yaml`, `fr.yaml`:
  - `controllers/library.rs`: `"Creating collection '{name}'..."` (2 call sites),
    `"Deleting collection…"`, `"Loading thumbnails…"`,
    `"Loading thumbnails… ({remaining} remaining)"`,
    `"Downloading {title} — {file_name}..."`, `"Downloading {title}..."`
  - `controllers/settings.rs`: `"Session setup failed after sign-in: {}"`
- No other production `src/ui/` or `src/controllers/` code paths were found with hardcoded
  user-facing text — placeholder, tooltip, title, message, and static `.child()` text are
  already fully localized. `ui/windows/app.rs` (`"Click Me"`, `"ok"`) is dead scaffold code
  with no callers; excluded from scope, flagged for removal separately.

## Capabilities

### New Capabilities

- `activity-label-localization`: All Activity Panel labels (collection create/delete,
  thumbnail load/refresh progress, file download progress, session-setup failure) are
  emitted through `t!()` rather than hardcoded English, so they render in the active locale
  like every other panel in the app.

### Modified Capabilities

_(none — `locale-manager`'s existing requirements about startup detection and fallback
behavior are unaffected; this change only extends *coverage*, not the mechanism)_

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: 6 call sites across
  `create_collection`/`delete_collection`/thumbnail-queue/download-queue activity-label
  construction.
- `crates/dtrpg-ui/src/controllers/settings.rs`: 1 call site in the post-sign-in session-setup
  error path.
- `crates/dtrpg-ui/i18n/en.yaml`, `de.yaml`, `fr.yaml`: new `activity.*` keys, added in lockstep
  across all three locales (per this project's i18n convention of never landing an
  English-only key).
