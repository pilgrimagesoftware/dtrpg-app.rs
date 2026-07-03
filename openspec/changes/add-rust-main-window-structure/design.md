## Context

`rust-main-window-library-layout` defined the current GPUI baseline: a disclosable search/filter
area, an account menu popover, and shared list/tree/grid browsing state. `shared-main-window-structure`
now requires a title bar, a persistent sidebar, tabbed content with popover/tab detail, and a
consolidated status bar. Substantial pieces already exist in progress in this repository: sidebar
collections/publishers work, gpui-component primitive adoption, activity panel, alert history, and
account menu work. This change coordinates those pieces into the shared structure instead of
rebuilding them.

## Goals / Non-Goals

**Goals:**

- Map `shared-main-window-structure` onto GPUI using `gpui-component` primitives.
- Reuse existing sidebar, activity panel, notification, and account menu work rather than
  duplicating it.
- Define the tab strip, catalog tab header, and popover/tab detail interaction model.
- Reference the gpui-components gallery demo for tab strip, status bar, and popover patterns.

**Non-Goals:**

- Redefine activity panel or notification panel internal content beyond relocating their entry
  points into the status bar.
- Specify exact pixel dimensions or animation timing.
- Change the underlying catalog/collection/publisher data model.

## Decisions

Build the tab strip on `gpui-component`'s `Tab`/`TabBar` components rather than a custom
implementation.
Rationale: the gallery demo already demonstrates a segmented tab strip with overflow handling;
reusing it reduces GPUI-specific edge cases.

Keep the catalog tab's header (search/sort/view mode) as the existing `catalog-menu` and
`sort-menu-group-toggle` controls, relocated rather than rebuilt.
Rationale: those controls already have working state management; moving their container avoids
regressions in filter/sort behavior.

Model the popover detail view with `gpui-component`'s `Popover`, anchored to the clicked catalog
row/card.
Rationale: matches the gallery demo's popover pattern and avoids a second detail-rendering
implementation separate from the existing detail panel.

## Risks / Trade-offs

- **Risk: Existing detail panel work (`detail-panel-resizable-and-wrapping`) targets a
  single-pane layout, not a tab** -> Mitigation: reuse its content rendering inside a new tab
  container rather than duplicating attribute/file-list rendering logic.
- **Risk: Status bar consolidation duplicates existing activity/notification indicators** ->
  Mitigation: status bar hosts existing activity and notification components, relocated, not
  reimplemented.
- **Risk: Tab overflow menu conflicts with existing window menu** -> Mitigation: scope the "more"
  menu to the tab strip only, distinct from `window-menu`.

## Migration Plan

1. Land `shared-main-window-structure` in `dtrpg-app` (already complete).
2. Implement the title bar, extending existing account menu work with a sign-out action.
3. Extend the sidebar with default section counts; Collections/Publishers carry forward.
4. Implement the tab strip and catalog tab header, relocating existing search/sort/view controls.
5. Implement popover and expanded detail tab interactions, reusing existing detail panel content.
6. Implement the status bar, relocating theme, activity, and notification indicators.
7. Verify against `rust-main-window-library-layout`'s carried-forward browsing-state requirements.
