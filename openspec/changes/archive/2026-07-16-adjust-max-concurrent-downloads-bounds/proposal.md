## Why

`max_concurrent_downloads` is currently bounded 1-10
(`MIN_CONCURRENT_DOWNLOADS`/`MAX_CONCURRENT_DOWNLOADS` in
`settings_storage_view.rs`), rendered via a hand-rolled minus/value/plus
button row. 10 concurrent connections is more than this app needs in
practice and invites accidentally saturating a user's connection; narrowing
the upper bound to 5 reduces that risk while still allowing meaningfully
faster thumbnail/download throughput than serial fetching. The control
itself should also stop being hand-rolled — `gpui-component` already ships
`NumberInput`, an editable numeric field with built-in +/- buttons and
`min`/`max` clamping, which the "Recently Updated window" setting
(`recently_updated_window_days`) already adopted in a prior change; this
control should follow the same pattern instead of maintaining a second,
inconsistent hand-rolled stepper implementation.

## What Changes

- `MAX_CONCURRENT_DOWNLOADS` lowers from 10 to 5; `MIN_CONCURRENT_DOWNLOADS`
  stays at 1.
- The Storage settings page's "Max concurrent downloads" row switches from
  a hand-rolled minus/value/plus button trio to
  `gpui_component::input::NumberInput`, bound to an `InputState` with
  `min`/`max` set to the (unchanged) 1 and (new) 5 bounds — mirroring the
  wiring already used for the "Recently Updated window" field
  (`render_recently_updated_window_row` in `settings_advanced_view.rs`).
- The decrement/increment tooltip i18n keys used only by the old hand-rolled
  buttons are removed, since `NumberInput` supplies its own step controls.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `settings-editable-fields`: adds a requirement for the "Max concurrent
  downloads" field becoming a directly editable, bounded (1-5) number input
  rather than a display-only value adjusted only by separate +/- buttons.
  `thumbnail-queue-concurrency` and `download-queue` reference
  `max_concurrent_downloads` only as an opaque configured value and are
  unaffected — the bound change and widget swap are settings-UI concerns,
  not queue-behavior ones.

## Impact

- `crates/dtrpg-ui/src/ui/views/settings_storage_view.rs`:
  `MAX_CONCURRENT_DOWNLOADS` constant lowered to `5`; hand-rolled
  `render_concurrency_stepper` replaced with a `NumberInput`-based row.
- `crates/dtrpg-ui/src/ui/views/root_view.rs`: creates the concurrency
  field's `InputState` (with `min`/`max` bounds) and an `InputEvent::Change`
  subscription committing to `SettingsController`, mirroring the existing
  "Recently Updated window" input wiring.
- `crates/dtrpg-ui/src/controllers/settings.rs`: `SettingsController` gains
  a `max_concurrent_downloads_input: Option<Entity<InputState>>` field,
  snapshot field, and setter, mirroring
  `recently_updated_window_input`/`storage_path_input`.
- `crates/dtrpg-ui/i18n/en.yaml`, `de.yaml`, `fr.yaml`:
  `settings.max_concurrent_downloads_decrement_tooltip` and
  `settings.max_concurrent_downloads_increment_tooltip` keys removed.
