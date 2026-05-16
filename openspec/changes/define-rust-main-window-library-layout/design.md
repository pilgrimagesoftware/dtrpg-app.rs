## Context

The Rust frontend uses GPUI and already has library controller/view modules for search, sort, grouping, list/tree display, and SDK-backed data loading. The new shared main-window contract adds account menu, grid presentation, collapsed filter summary, view summary, background sync detail, and thumbnail responsiveness expectations.

## Goals / Non-Goals

**Goals:**

- Map shared main-window regions to GPUI view modules and Rust controller state.
- Reuse existing Rust search, sort, grouping, and library SDK adapter behavior where possible.
- Define Rust state for browsing, account menu, sync status, and thumbnail loading.
- Require list/tree and grid modes to share one filtered/sorted result set.
- Keep sync and thumbnail work non-blocking from the GPUI main-window interaction path.

**Non-Goals:**

- Redefine Rust SDK contracts.
- Replace the GPUI shell architecture.
- Implement credential storage details beyond UI state and action routing.
- Add download or file-management actions.

## Decisions

Extend the existing Rust library controller state instead of creating a parallel browsing controller.
Rationale: current Rust modules already own search, view mode, grouping, sorting, selection, and summary behavior.

Add grid presentation as another view over the same filtered result set.
Rationale: grid mode should change layout and thumbnail presentation, not search/filter semantics.

Model account menu and sync status as explicit controller-facing state.
Rationale: account and sync indicators must remain low profile but still be testable and inspectable.

Treat thumbnails as optional asynchronous presentation data.
Rationale: title and size metadata must remain visible even if cover art is unavailable or loading.

## Risks / Trade-offs

- Existing Rust controls may need reshaping to become disclosable. Mitigation: keep search/filter state and control rendering separate so the collapsed summary can reuse state.
- GPUI menu support may differ from native platform menus. Mitigation: implement an account popover/menu affordance that satisfies the interaction contract even if the underlying primitive differs.
- Background sync may require service changes. Mitigation: first model sync status and keep service calls off the interaction path, then wire real progress as SDK support allows.
