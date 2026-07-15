## Context

`locale-manager` already provides locale detection, fallback, and the `t!()` mechanism; nearly
all UI text uses it. This change closes the last gap: 7 hardcoded Activity Panel label call
sites in `controllers/library.rs` and `controllers/settings.rs`, found by a full-repo scan
cross-checked against `en.yaml`/`de.yaml`/`fr.yaml`.

## Goals / Non-Goals

**Goals:**
- Every remaining hardcoded label found by the scan renders through `t!()`.
- New keys land in all three locale files in the same commit (no English-only key, per
  existing project convention already followed throughout `i18n/`).

**Non-Goals:**
- No changes to the `locale-manager` mechanism itself (detection, fallback, `rust_i18n`
  wiring).
- No new locales added.
- `ui/windows/app.rs`'s hardcoded `"Click Me"`/`"ok"` strings are out of scope — that struct
  (`AppWindow`) has no callers anywhere in the codebase; it is dead code, not reachable UI.

## Decisions

- **Key naming**: follow the existing `activity.<snake_case_label>` convention already used
  for `activity.loading_library`, `activity.loading_collections`, etc. — e.g.
  `activity.creating_collection`, `activity.deleting_collection`,
  `activity.downloading_file`. Keeps all Activity Panel strings grouped under one YAML
  section instead of introducing a new top-level namespace.
- **Interpolation**: use `rust_i18n`'s `%{name}` placeholder syntax already used elsewhere
  (e.g. `activity.loading_library_page: "Loading library: page %{page}…"`), matching each
  `format!()` call's existing interpolated values (`name`, `remaining`, `title`,
  `file_name`, error text) 1:1 so no information is dropped in translation.
- **One key per distinct message shape**, not parameterized variants collapsed into one —
  matches the existing pattern where e.g. `loading_library_page` and `loading_library_count`
  are separate keys rather than one key with a "mode" parameter. Keeps each translator-facing
  string a complete, grammatical sentence in every locale rather than assembled from
  fragments.

## Risks / Trade-offs

- [Risk] A locale file gets a key added without its sibling files, silently falling back to
  the key name at runtime (per `locale-manager`'s existing fallback behavior) instead of
  failing the build → Mitigation: add all three locale entries in the same edit pass per key,
  and grep all three files for the new key names before considering the task done.
- [Risk] `settings.rs`'s `"Session setup failed after sign-in: {}"` embeds a raw SDK/service
  error string (`e.0`) that itself may be untranslated (comes from the network layer, not the
  UI layer) → Mitigation: localize only the fixed prefix ("Session setup failed after
  sign-in:"); the interpolated error detail is expected to stay in its original form, same as
  every other error-detail interpolation already in the codebase (e.g.
  `notification.session_expired`-style messages don't translate upstream error text either).

## Migration Plan

Single PR, no runtime migration — string replacement plus additive YAML keys is
backward-compatible and takes effect on next app launch. No rollback concerns beyond a normal
revert.
