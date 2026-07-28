## Context

`render_concurrency_stepper` in `settings_storage_view.rs` renders "Max
concurrent downloads" as a fixed-width value flanked by two icon-button divs
wired to `entity.update(cx, |ctrl, cx| ctrl.set_max_concurrent_downloads(n
± 1, cx))`, each guarded by `MIN_CONCURRENT_DOWNLOADS`/
`MAX_CONCURRENT_DOWNLOADS` checks. The value itself is never directly
editable — only +/- by one at a time, up to 10 clicks to go from 1 to 10.

A prior change (`fix-recently-updated-filter`) needed a very similar
bounded-numeric settings control (`recently_updated_window_days`, 7-90) and,
instead of copying this hand-rolled pattern, adopted
`gpui_component::input::NumberInput` — an editable text field with built-in
+/- buttons, `min`/`max` bounds set on the underlying `InputState`, and
clamp-on-blur. That wiring (`InputState` created once in `RootView::new`,
attached to `SettingsController` via a setter, exposed through
`SettingsSnapshot`, committed on `InputEvent::Change`) is the template for
this change.

## Goals / Non-Goals

**Goals:**
- Lower `MAX_CONCURRENT_DOWNLOADS` from 10 to 5.
- Replace `render_concurrency_stepper`'s hand-rolled button pair with
  `NumberInput`, so the value is directly typeable in addition to
  step-by-one via the built-in buttons.
- Reuse the exact `InputState`-creation/subscription/setter pattern already
  established for `recently_updated_window_days`, rather than inventing a
  second approach.

**Non-Goals:**
- Changing `MIN_CONCURRENT_DOWNLOADS` (stays at 1 — 0 would stop thumbnails
  and downloads entirely, per the existing constant's doc comment).
- Changing where the control lives (stays on the Storage settings page,
  next to "Create collections" — unlike `recently_updated_window_days`,
  this setting is about storage/download behavior, so the existing tab is
  still the right fit; nothing here justifies moving it to Advanced).
- Any change to `remaining_slots`, the download/thumbnail queue dispatch
  logic, or how `max_concurrent_downloads` is consumed once set — only the
  bound and the settings-page widget change.

## Decisions

- **Reuse `NumberInput` exactly as `recently_updated_window_days` does.**
  Create the `InputState` once in `RootView::new` with
  `.min(f64::from(MIN_CONCURRENT_DOWNLOADS)).max(f64::from(MAX_CONCURRENT_DOWNLOADS))`,
  subscribe to `InputEvent::Change`, parse as `usize`, and call
  `SettingsController::set_max_concurrent_downloads` on success — the same
  four-step shape (create input → subscribe → attach via setter → render via
  snapshot) already used for the window-days field, so a future reader who
  knows one control understands the other.
- **`MAX_CONCURRENT_DOWNLOADS` lowers to 5, not made configurable.** The
  proposal only asks for a bound change and a widget change; introducing a
  second layer of "bounds for the bounds" is out of scope and unrequested.
- **Data-layer value is unclamped by this change.** `StorageConfig` stores
  `max_concurrent_downloads` as a plain `usize` with no clamp in its setter
  today (unlike `recently_updated_window_days`, which added one in the
  prior change). Since the proposal doesn't ask for that hardening and the
  UI-layer bound is enforced by `NumberInput`'s own `min`/`max`, this change
  leaves `StorageConfig::set_max_concurrent_downloads` as-is rather than
  scope-creeping in a second, unrelated clamp.
- **`SettingsController` gains `max_concurrent_downloads_input:
  Option<Entity<InputState>>`**, mirroring
  `recently_updated_window_input`/`storage_path_input` field-for-field
  (private struct field, public snapshot field, `set_*_input` setter).

## Risks / Trade-offs

- [A user who previously set a value between 6 and 10 has it silently
  exceed the new max on next load] → `StorageConfig::load()` performs no
  migration or reclamping of a persisted out-of-range value; the field
  simply displays whatever was saved. The first time the user interacts
  with the (now bounded 1-5) `NumberInput`, blur-clamping brings it back in
  range. Accepted as low-impact: `max_concurrent_downloads` only throttles
  parallelism, so a stale value above 5 is not a correctness or safety
  issue, only a temporarily-unenforced upper bound until the user next
  touches the field.
- [Duplicate wiring code between the two `NumberInput` fields] → Both
  `RootView::new`'s input-creation blocks and both `SettingsController`
  input fields/setters are near-identical boilerplate. Accepted rather than
  extracted into a shared helper: two instances of a five-line pattern
  don't yet justify an abstraction (see the project's "introduce
  abstractions only when a pattern repeats multiple times" guidance); worth
  revisiting if a third bounded-numeric settings field appears.
