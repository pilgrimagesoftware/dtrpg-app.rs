## Context

`email_input` (and `password_input`, `storage_path_input`) is created once in `LibraryRootView::new`
(the main window's constructor) and attached to `SettingsController` via `set_email_input` — it
exists for the lifetime of the app, not lazily created when the settings window opens. The
notification banner's "Login to DriveThruRPG" action (`notification_banner_view.rs`) currently
calls `LibraryRootView::show_settings`, which opens/activates the settings window but does not
touch `SettingsController::active_page_ix` (the Account tab is index 0, the fallback case in
`settings_view.rs`'s `page_title` match, so it's already the default unless the user last had a
different tab open — see Risks) or request any focus.

`gpui`'s focus system is per-window: an `InputState` entity must be focused using the `Window`
of the window it's actually rendered in. The banner's `on_click` runs in the *main* window's
event context, but `email_input` renders inside the *settings* window
(`settings_account_view.rs`'s `render_unauthenticated`) — the settings window's own `Window`
isn't available at the banner's click site, especially since `show_settings` may need to create
that window for the first time in the same call.

## Goals / Non-Goals

**Goals:**
- Clicking "Login to DriveThruRPG" opens Settings on the Account tab with the email field
  already focused, so the user can start typing immediately.
- Works whether the settings window is being newly created or already exists (brought to
  front).

**Non-Goals:**
- Focusing the email field on every unauthenticated Account-tab view — only the
  banner-triggered path is in scope; manually clicking the Account tab keeps today's behavior
  (no forced focus).
- Any change to the sign-in form's validation, submission, or error-handling behavior.

## Decisions

**A one-shot "pending focus" flag on `SettingsController`, consumed by the Account tab's own
render pass (which has the correct `Window`), rather than trying to focus from the banner's
click handler directly.**

> **Implementation note:** the flag is actually consumed in `SettingsWindowView::render`
> (`settings_window_view.rs`), not `settings_account_view.rs`'s `render_unauthenticated`.
> `SettingsWindowView::render` already has `window: &mut Window` on every render pass and is
> where `email_input` is read out of the snapshot before being handed down to the render-helper
> chain — consuming the flag there avoids threading `window`/`cx` through
> `render_settings_panel` / `render_account_section` / `render_unauthenticated` just for this
> one-shot check. Same net effect: the check still only applies while the settings window is
> open and only fires once per banner click.

The banner's click handler cannot call `.focus(window, cx)` on `email_input` itself — it only
has the main window's `Window`, not the settings window's, and the settings window may not
exist yet at that point in the call. Instead, the click handler sets `active_page_ix` to the
Account tab and sets a new `focus_email_pending: bool` field on `SettingsController` to `true`.
The Account tab's render function (which already has `window: &mut Window` for the settings
window) checks this flag each render; if set, it calls `email_input.update(cx, |input, cx|
input.focus(window, cx))` and immediately clears the flag so the focus call fires exactly once,
not on every subsequent render while the tab stays open.
Alternative considered: have `show_settings` synchronously focus the input right after creating
the window. Rejected — `show_settings` doesn't have a reference to the settings window's
freshly-created `Window` in a form it can hand to `InputState::focus` at that point in the
call graph, and coupling window-open logic to a specific input field's focus is a worse
separation of concerns than a flag the relevant view consumes itself.

**The flag is scoped to "Account tab, unauthenticated" — it has no effect if the user is
already signed in.**
`render_unauthenticated` (not the signed-in Account tab render path) is the only place the flag
is consumed. If the flag is somehow set while already authenticated (shouldn't happen given the
banner only shows when unauthenticated, but defensively), it's simply never read and has no
effect — no need for an explicit guard clearing it in that case.

## Risks / Trade-offs

- [If the user had a different settings tab open last, and the settings window is *already
  open* when the banner is clicked, forcing `active_page_ix` back to Account is a page switch
  they didn't explicitly ask for] → Accepted: the banner's whole purpose is to get the user to
  the sign-in form as directly as possible; switching to Account tab is the intended behavior,
  not a side effect to avoid.
- [The pending-focus flag could theoretically leak `true` across a settings-window close/reopen
  if the render path that clears it never runs] → Mitigated: the flag is only ever set
  immediately before `show_settings` opens/activates the window, so the Account tab (already
  the active page at that point) renders on the very next frame and clears it; there's no
  window-lifecycle path where it could open without rendering at all.

## Migration Plan

Not applicable — additive behavior change with no persisted state format change (`active_page_ix`
was already persisted via `UiState`, unchanged here; `focus_email_pending` is in-memory only, not
persisted).

## Open Questions

None.
