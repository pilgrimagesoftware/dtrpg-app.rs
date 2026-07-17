## 1. Pending-Focus Flag

- [x] 1.1 Add `focus_email_pending: bool` to `SettingsController`, defaulting to `false`.
- [x] 1.2 Add `pub fn request_email_focus_on_account_tab(&mut self, cx: &mut Context<Self>)` that
      sets `active_page_ix` to `0` (Account) via the existing persistence path and sets
      `focus_email_pending = true`.

## 2. Banner Wiring

- [x] 2.1 In `notification_banner_view.rs`'s "Login to DriveThruRPG" button `on_click`, call
      `settings_entity.update(cx, |ctrl, cx| ctrl.request_email_focus_on_account_tab(cx))`
      alongside the existing `root_entity.update(cx, |view, cx| view.show_settings(cx))` call.
      (Implemented via a new `LibraryRootView::show_settings_focused_on_email` helper that wraps
      both calls, so the banner keeps calling a single method through `root_entity`.)

## 3. Consuming the Flag

- [x] 3.1 In the unauthenticated Account tab's render path (`settings_account_view.rs`'s
      `render_unauthenticated`, or wherever `window: &mut Window` is available for the settings
      window), check `SettingsController`'s `focus_email_pending`; if set, call
      `email_input.update(cx, |input, cx| input.focus(window, cx))` and immediately clear the
      flag so it fires exactly once per banner click, not on every render.
      (Implemented in `SettingsWindowView::render` instead, which already has `window: &mut
      Window` for the settings window on every render pass and is where `email_input` is read
      from the snapshot — avoids threading `window`/`cx` through the render-helper call chain
      just for this one-shot check.)

## 4. Verification

- [x] 4.1 Add a unit test (or controller-level test, matching this codebase's existing
      `SettingsController` test patterns) confirming `request_email_focus_on_account_tab` sets
      both `active_page_ix = 0` and `focus_email_pending = true`, and that consuming the flag
      clears it.
- [x] 4.2 Manually verify (left to the user, per project convention): clicking "Login to
      DriveThruRPG" from a fresh, unauthenticated startup opens Settings on the Account tab
      with the email field focused and ready to type; manually clicking the Account tab
      afterward does not force-focus the email field again.
