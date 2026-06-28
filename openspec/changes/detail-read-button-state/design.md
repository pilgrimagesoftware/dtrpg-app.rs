## Context

The detail panel in `detail_panel_view.rs` renders a "Read" button unconditionally with full interactivity regardless of `item.status`. The `is_downloaded` flag is already derived from `item.status == ItemStatus::Downloaded` and used to conditionally render the "Downloaded" label and the "Reveal in Finder" button — but the Read button ignores it. The existing tooltip pattern used elsewhere in the codebase is `gpui_component::tooltip::Tooltip`, which is already imported in this file.

## Goals / Non-Goals

**Goals:**
- Disable the Read button visually and interactively when `!is_downloaded`
- Show a tooltip on the disabled button explaining the download prerequisite
- Keep the button in the layout at all times (no conditional show/hide)

**Non-Goals:**
- Implementing the actual Read action (it is already a stub/placeholder)
- Changing the Download or Reveal buttons
- Adding a download-on-demand flow from the Read button

## Decisions

### Disabled styling: dim opacity, no cursor_pointer, no on_click

When `!is_downloaded`, the Read button: omits `.cursor_pointer()`, omits `.on_click()`, and applies a reduced `opacity` or uses `text_tertiary` for the label instead of `accent_on`. The button background remains `accent` but at reduced opacity (`opacity(0.4)`) so the user can see it is a real action that is temporarily unavailable, not a hidden feature. This is consistent with how GPUI renders disabled-feeling elements elsewhere in the codebase.

### Tooltip only on the disabled state

`.tooltip(...)` is added only on the disabled (not-downloaded) variant. When `is_downloaded`, no tooltip is needed on the Read button. This avoids cluttering the always-enabled case with noise.

### id on both states

Both the enabled and disabled variants carry `.id("detail-read")` so GPUI can track the element across renders. The disabled variant needs an id to host the tooltip.

## Risks / Trade-offs

- **No true GPUI "disabled" prop**: GPUI divs don't have a first-class `disabled` attribute. The disabled appearance is achieved via styling + omitting click handlers. This is the established pattern in the codebase.
- **Opacity on accent background**: `opacity(0.4)` on the accent-colored button visually communicates disabled without needing a separate disabled color token. If the design system later adds a `disabled_bg` token, prefer that.
