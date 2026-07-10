## Context

`render_collection_row` (`sidebar_view.rs`) builds a `context_menu` with "Reload" and "Delete" items; "Delete" calls `ctrl.delete_collection(col_id, cx)` directly on click. The context menu's `on_click` handlers already receive `(&ClickEvent, &mut Window, &mut App)` — the `Window` parameter is currently discarded as `_`, but `window.open_alert_dialog` needs it. The row's collection name (`row.name: Arc<str>`) is already in scope for the filter/display, so no new data plumbing is needed for the dialog copy.

## Goals / Non-Goals

**Goals:**
- Selecting "Delete" from a collection's context menu shows a confirmation dialog naming the collection before the server delete request fires.
- Reuse the existing `window.open_alert_dialog` + `AlertDialog::confirm()` pattern (`settings_file_openers_view.rs`) — no new dialog component.

**Non-Goals:**
- No change to `delete_collection`'s behavior, error handling, or the collections service API call.
- No "don't ask again" preference.

## Decisions

### Wrap the existing "Delete" `on_click` handler in `open_alert_dialog`

Capture `window` (currently `_`) in the `on_click` closure and call `window.open_alert_dialog(cx, |alert, _, _| alert.confirm().title(...).description(...).on_ok(move |_, _, cx| { entity.update(cx, |ctrl, cx| ctrl.delete_collection(col_id, cx)); true }))`, matching the file-opener removal call site exactly. Single call site — no shared helper needed at this scale.

## Risks / Trade-offs

- [Risk] None beyond the standard UX cost of one extra click to confirm — acceptable for a destructive, irreversible server action.
