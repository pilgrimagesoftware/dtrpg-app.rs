## Context

`remove_download` (`LibraryController`) reverts a `Downloaded` item to `Cloud` status and is wired to three UI entry points, all of which currently call it directly on click with no confirmation:

- `catalog_view.rs`: two `action_remove_download` context-menu handlers (Grid and Thumbs presentations), each already holding `entity_remove`/`remove_id` in scope.
- `item_popover_view.rs`: the single toggle button's "downloaded" branch.
- `detail_panel_view.rs`: the download button's "downloaded" branch.

The codebase already has an established confirmation pattern for a destructive click: `settings_file_openers_view.rs`'s remove button calls `window.open_alert_dialog(cx, |alert, _, _| alert.confirm().title(...).description(...).on_ok(...))`, backed by gpui-component's alert dialog primitive. No custom modal component exists or is needed.

## Goals / Non-Goals

**Goals:**
- Every path that can trigger `remove_download` shows a confirmation dialog naming the item first.
- Reuse the existing `window.open_alert_dialog` + `.confirm()` pattern exactly as `settings_file_openers_view.rs` already uses it — no new dialog component.
- Cancelling the dialog leaves the item's status and the queue/activity state untouched.

**Non-Goals:**
- No change to what `remove_download` itself does (still a status-only revert; no real file deletion exists yet).
- No confirmation for starting a download (`enqueue_download`) or cancelling one (`cancel_download`) — only removing an already-downloaded item is destructive enough to warrant this.
- No "don't ask again" preference — out of scope; can be a follow-on if requested.

## Decisions

### Reuse `window.open_alert_dialog` + `AlertDialog::confirm()`

This is the only confirmation-dialog primitive already in use in the codebase (`settings_file_openers_view.rs`), so introducing anything else would create two competing patterns for the same UX. The `on_ok` callback calls `remove_download` exactly where the old direct call was; declining (closing/cancelling the dialog) does nothing, matching the alert dialog's default behavior.

_Alternative considered_: A custom lightweight inline confirm (e.g., a second click within N seconds, or an inline "Remove?" swap). Rejected — inconsistent with the one destructive-confirmation pattern the app already has, and less discoverable/accessible than a modal.

### Confirmation dialog is per-call-site, not centralized in `LibraryController`

The dialog is a `window`-level concern (`open_alert_dialog` needs `&mut Window`), so it's added at each of the three render call sites rather than inside `remove_download` itself. `remove_download`'s signature and behavior are unchanged; only the three UI handlers gain a dialog wrapper around the existing `ctrl.remove_download(&id, cx)` call.

_Alternative considered_: Add a `confirm: bool` parameter or a separate `request_remove_download` that emits an event the view layer listens for. Rejected as unnecessary indirection — `open_alert_dialog` is already callable directly from each `on_click` handler with no controller-level involvement needed.

### Dialog copy includes the item title

Matches the file-opener removal dialog's pattern of naming the specific thing being removed (`t!("settings.file_opener_remove_confirm_title", ext = ext)`). All three call sites already have the item's title in scope (used for `enqueue_download`'s title argument), so no new data plumbing is needed.

## Risks / Trade-offs

- [Risk] Three call sites duplicate the same dialog-wrapping boilerplate. → Mitigation: the `.title()`/`.description()`/`.on_ok()` chain is short (as seen in `settings_file_openers_view.rs`); a shared helper can be extracted later if a fourth call site appears, but three is within the "three similar lines is better than a premature abstraction" threshold.
