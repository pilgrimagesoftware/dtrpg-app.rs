## Context

`toolbar_view.rs` contains two separate rendering functions: `render_sort_selector` (a `Button` with a `dropdown_menu`) and `render_group_toggle` (a standalone styled `div` with `on_click`). Both are called from `render_toolbar` and laid out side-by-side. The group toggle uses custom background coloring to reflect active state; the sort button has no visual indicator that it opens a menu.

`PopupMenuItem::separator()` is available in `gpui_component::menu` and renders a visual divider between menu item groups. `Button::dropdown_caret(true)` renders a chevron at the trailing edge of the button label.

## Goals / Non-Goals

**Goals:**
- Add `grouped: bool` and the group `Entity<LibraryController>` to `render_sort_selector`'s parameters so it can add the group toggle inside the menu
- Add `PopupMenuItem::separator()` between sort items and the group toggle item
- Add a checkable "Group by Publisher" `PopupMenuItem` that calls `ctrl.set_grouped(!grouped, cx)` on click
- Add `.dropdown_caret(true)` to the sort button
- Remove `render_group_toggle` function and its call site in `render_toolbar`

**Non-Goals:**
- Changing the sort or grouping controller logic
- Reordering other toolbar items
- Adding keyboard shortcuts or accessibility labels beyond what `.checked()` provides

## Decisions

### Pass `grouped` into `render_sort_selector`

`render_sort_selector` already receives `Entity<LibraryController>`; adding `grouped: bool` gives it the state needed to set `.checked(grouped)` on the group item and to pass `!grouped` into `set_grouped`. The entity clone pattern already used for the four sort items applies identically here.

### Use `PopupMenuItem::separator()` between sort items and group item

The sort items (Title, Publisher, Date Added, Pages) are a single-select group; the group toggle is a separate boolean control. A separator makes the semantic boundary clear. `PopupMenuItem::separator()` is the established API — no custom rendering needed.

### Add `.dropdown_caret(true)` to the sort button

`Button::dropdown_caret(true)` appends a chevron glyph that is already styled to match the button's label color. This is the low-effort, consistent-with-the-library way to signal the button opens a menu. No custom icon or layout is needed.

### Remove `render_group_toggle` entirely

The function becomes dead code after this change. Per project guidelines, unused code is deleted rather than left in place with a comment.

## Risks / Trade-offs

- **Parameter count on `render_sort_selector`**: The function already takes 6 parameters; adding `grouped: bool` makes 7. This is acceptable for a render helper — no struct abstraction is warranted yet.
- **Group state not in label**: The sort button label still shows only the current sort method (e.g., "Title"), not whether grouping is active. The grouped state is visible only when the menu is open (via the checkmark). This is an accepted trade-off — the label would become unwieldy if it also reflected grouping.
