## Context

`render_sort_selector` in `dtrpg-ui/src/ui/views/toolbar_view.rs` builds the sort dropdown as a
`gpui_component::button::Button` with a text label and `.dropdown_caret(true)`, no leading icon.
The layout switcher immediately below it (`render_layout_switcher`) already uses
`Icon::empty().path("icons/<name>.svg")` per `Tab`, so the icon-loading mechanism and asset
convention are established; this change only needs to apply the same mechanism to `Button`.

## Goals / Non-Goals

**Goals:**

- Add a leading icon to the sort dropdown button so its purpose is recognizable without reading
  the label.

**Non-Goals:**

- Changing the icon based on current sort method or direction (the layout switcher's per-option
  icons are a different pattern — one icon per tab; the sort button is a single control with one
  fixed icon regardless of state).
- Any change to menu contents, sort methods, or the group-by-publisher toggle in the same
  dropdown.

## Decisions

### Use `assets/icons/arrow-down-up.svg`

Existing sort-related assets are `arrow-down-up.svg`, `list-sort-ascending.svg`,
`list-sort-descending.svg`, `sort-asc.svg`, `sort-desc.svg`. The button's icon is static (does
not flip with the current sort direction — direction is a separate, less prominent menu item
inside the dropdown), so a direction-agnostic glyph is correct; `arrow-down-up.svg` is the only
one that doesn't imply a specific direction.

**Alternative considered**: swapping the icon between `sort-asc.svg`/`sort-desc.svg` based on
`SortDirection`. Rejected — the button label already doesn't reflect direction either (it shows
the sort *method*, e.g. "Title"), and making only the icon direction-aware would be inconsistent
and add state-tracking for no clear benefit given the direction toggle already lives one level
down in the same dropdown.

### `Button::icon(...)`, same call shape as `Tab::icon(...)`

`gpui_component::button::Button` exposes `pub fn icon(mut self, icon: impl Into<ButtonIcon>) -> Self`,
accepting the same `Icon` type already used via `Icon::empty().path(...)` in
`render_layout_switcher`. Apply it identically on the `Button::new("sort-selector")` chain.

## Risks / Trade-offs

- **None of consequence** — this is an additive, single-call visual change with an existing
  asset and an existing, already-used API. No behavior, state, or persistence changes.
