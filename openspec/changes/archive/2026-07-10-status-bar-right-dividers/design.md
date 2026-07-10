## Context

`render_status_bar` (`ui/views/status_bar_view.rs`) builds a `gpui_component::status_bar::StatusBar`
via chained `.left(...)`/`.right(...)` calls. The left side already separates
`library_summary` from `active_tab_summary` with `.left(Separator::vertical())`. The right
side chains `theme_picker`, `activity_indicator`, `notification_indicator` back-to-back
with no equivalent separator, which this change corrects.

## Goals / Non-Goals

**Goals:**
- Visually separate the three right-side controls consistently with the existing
  left-side divider pattern, using the same `Separator::vertical()` component already
  imported in this file.

**Non-Goals:**
- No change to spacing/padding within `StatusBar` itself, no new divider component, no
  change to the left-side layout (already correct).
- No behavior change to the theme picker, activity indicator, or notification button.

## Decisions

- **Reuse `gpui_component::separator::Separator::vertical()`**, the same type already
  used on the left side and already imported in this file — no alternative considered,
  this is the established, single divider primitive in the codebase for this exact
  purpose.
- **Two separators, not a wrapping container.** `StatusBar::right` is called once per
  item and appends in call order, so two additional `.right(Separator::vertical())`
  calls — one between the theme picker and activity indicator, one between the activity
  indicator and notification indicator — produce the same "N items, N-1 dividers"
  layout as the left side without introducing a different structural pattern (e.g. a
  `Vec<AnyElement>` with manual `.intersperse`-style joining) for a 3-item, always-static
  list.

## Risks / Trade-offs

- [Risk] None identified — this is a purely additive, visual-only change to a single
  render function with no state, no new dependencies, and no behavior change.
