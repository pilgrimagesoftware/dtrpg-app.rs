## Context

`settings_file_openers_view.rs` renders the file openers section with two button types: an "Add" button in the header row (text "Add" on accent background) and a "Remove" button per entry row (text "Remove" in a bordered div). Neither has a tooltip. Removal fires immediately on click with no confirmation. `render_file_openers_section` and `render_entry_row` are pure rendering functions that accept no `cx: &App` parameter. `AlertDialog` from `gpui_component::dialog` requires `cx: &mut App` to construct.

## Goals / Non-Goals

**Goals:**
- Add button: swap text `"Add"` for `"+"`, add a tooltip
- Remove button: swap text `"Remove"` for `"×"`, add a tooltip
- Wrap remove with `AlertDialog::new(cx).confirm()` so the actual removal only fires on confirmation
- Pass `cx: &App` to the render functions that need it (same pattern as `render_toolbar` + `render_avatar_button` in the avatar-circular-image change)

**Non-Goals:**
- Adding an actual app picker dialog to the "Add" flow (deferred to a future change)
- Changing entry row layout or stale-path handling

## Decisions

### Use `AlertDialog` from `gpui_component::dialog`

`AlertDialog::new(cx).confirm()` wraps a trigger element and shows a confirmation modal on click. `.on_ok(|_, _, cx| { ... remove ... true })` fires the removal only if the user confirms; returning `true` closes the dialog. `.title()` and `.description()` provide context. This avoids any custom state management.

### Add `cx: &App` to render functions

`render_file_openers_section` and `render_entry_row` gain a `cx: &App` parameter. The call site in `settings_view.rs` passes `cx` through. This follows the same pattern established in `render_toolbar` / `render_avatar_button`.

### Alert dialog content identifies the entry

Title: `"Remove file opener?"`. Description: `format!("Remove the .{ext} opener for {app}?")` — gives the user enough context to confirm or cancel.

### Icon choices — Unicode glyphs

- Add: `"+"` — universally understood
- Remove: `"×"` — the multiplication/close glyph, consistent with the "remove" convention

Tooltips use `gpui_component::tooltip::Tooltip` (already imported in the codebase).

## Risks / Trade-offs

- **`cx: &App` propagation**: Two function signatures change. All callers are in the same crate, so the change is contained and safe.
- **`AlertDialog` and GPUI render cycle**: `AlertDialog::new(cx)` uses `cx` to set up internal view state. This must be called during the render function (which is already in the render cycle), so calling it in `render_entry_row` is correct.
