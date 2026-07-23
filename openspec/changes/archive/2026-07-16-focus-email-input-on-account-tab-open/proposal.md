## Why

Clicking the "Login to DriveThruRPG" notification banner action opens the Settings window to
the Account tab, but the user still has to click into the email field before they can type —
an unnecessary extra step given the whole point of that action is to get the user signing in
as fast as possible.

## What Changes

- When the banner's "Login to DriveThruRPG" action opens Settings to the Account tab, the
  email input field is focused immediately, so the user can start typing without an extra
  click.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `unauthenticated-main-window`: extends the existing "Sign-in MUST be available from the
  Settings Account tab" requirement with a focus behavior specific to the banner-triggered
  entry path — opening Settings via the banner's action focuses the email input immediately,
  rather than requiring a click to start typing.

## Impact

- `crates/dtrpg-ui/src/ui/views/notification_banner_view.rs`: the banner action's `on_click`
  handler, which currently only calls `LibraryRootView::show_settings`.
- `crates/dtrpg-ui/src/ui/views/root_view.rs`: `show_settings`, and however it threads through
  to the Account tab's email `InputState` so a focus request can reach it.
- `crates/dtrpg-ui/src/ui/views/settings_account_view.rs`: the unauthenticated Account tab
  render function, which owns the email `Entity<InputState>`.
