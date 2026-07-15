## Why

Adding or removing a catalog item from a collection via the "Manage Collections" dialog
uses an optimistic-update pattern (`LibraryController::add_item_to_collection` /
`remove_item_from_collection`): the UI updates immediately, then the change is confirmed
against the API in the background. On failure, both methods roll back the optimistic
update and emit `CollectionMemberAddFailed`/`CollectionMemberRemoveFailed`, which
`root_view.rs` turns into a top-right toast notification. Neither method ever calls
`ActivityController::start`/`error`, so the failure never reaches `alert_log` — the toast
appears, but the error is unrecoverable once dismissed; it never shows up in "Window >
Show Alert History". This is inconsistent with `create_collection` and
`create_collection_and_add_member`, which already register with `ActivityController` and
correctly appear in both the toast and the alert history.

## What Changes

- Add `ActivityController::log_alert(label: impl Into<Arc<str>>, message: String, cx)` — a
  new method that appends directly to the durable `alert_log` without requiring a
  corresponding in-progress entry. Unlike `error(id, ...)`, this is for failures in
  optimistic-update flows that never call `start` in the first place (registering a
  transient "in progress" activity for a synchronous, already-applied optimistic change
  would misrepresent it as a background operation with a spinner).
- `add_item_to_collection` and `remove_item_from_collection` call `activity.log_alert(...)`
  in their failure branches, alongside the existing `cx.emit(...Failed)` call, so the
  failure is now recorded in the alert history in addition to showing the toast.
- No change to the toast notification behavior, the optimistic-update/rollback mechanism,
  or the dialog's own inline error display — this only adds the missing alert log entry.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `alert-history-view`: collection membership add/remove failures (in addition to
  collection creation failures, which already work) are recorded in the durable alert log.

## Impact

- `dtrpg-ui/src/controllers/activity.rs` — add `log_alert` method
- `dtrpg-ui/src/controllers/library.rs` — call `log_alert` from `add_item_to_collection`'s
  and `remove_item_from_collection`'s failure branches
