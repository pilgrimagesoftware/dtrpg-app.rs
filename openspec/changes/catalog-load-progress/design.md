## Context

The DTRPG API does not return a total item count in the pagination metadata (`PaginationMeta` only has `items_per_page` and `current_page`). However, every paginated response includes a `links.last` URL that contains the last page number as a query parameter (e.g., `?page=42&pageSize=100`). From these two values we can derive an upper-bound estimate of total items.

The `ActivityController` and the activity panel UI already support `Option<f32>` progress on `ActivityItem`. The `update_progress(id, f32, cx)` method is in place. No changes are needed to the activity layer - only the catalog load flow needs to drive it.

## Goals / Non-Goals

**Goals:**

- Determine an estimated total item count using the `links.last` URL from the first page response.
- Seed the catalog load activity entry with that estimate so the progress bar renders immediately.
- Increment progress after each batch of items arrives.
- Show 100% progress when the final page completes.

**Non-Goals:**

- Exact total item counts (the last-page heuristic is an upper bound; the final page may have fewer items than `items_per_page`).
- A separate pre-flight count API call (adds latency with no benefit beyond what the first page already provides).
- Changing the `ActivityController`, `ActivityItem`, or activity panel UI (already sufficient).

## Decisions

**Total count from first-page `links.last` rather than a dedicated count endpoint.**

The DTRPG API has no count-only endpoint. A `pageSize=1` pre-flight request wastes a round-trip and delays the first item appearing in the UI. The first real-page response is free and already in flight; parsing `links.last` gives us the estimate with zero additional latency.

Rationale: `links.last = "...?page=N&pageSize=P"` → estimated total = `N * P`. The actual last page may have fewer items, so the bar may jump to 100% before mathematically reaching it - acceptable for a UX-only progress indicator.

**Extend `SdkLibraryGateway` to expose `last_page_from_links(links: &PaginationLinks) -> Option<u32>` as a module-level helper rather than a trait method.**

The last-page parsing is pure function logic with no I/O. It belongs as a free function in `sdk.rs`, not a trait method. The paged fetch loop calls it on the first page response and passes the total upward.

**Thread the total through `list_items_paged` via a new `on_total` callback parameter on `SdkLibraryGateway::list_order_products_paged`.**

The existing `LibraryService::list_items_paged` uses `on_page`. We add a parallel `on_total: Option<&mut dyn FnMut(usize)>` parameter to the internal gateway trait method only (not the public `LibraryService` trait). `RustSdkLibraryService::list_items_paged` calls `on_total` with the estimated total after the first page, then calls `on_page` for each subsequent page. The public `LibraryService` trait gets a new default method `estimated_item_count_hint` that returns `None` for stubs; the real implementation overrides it.

Alternative considered: add `item_count()` to `SdkLibraryGateway` as an extra API call. Rejected - adds latency, breaks existing test stubs, and provides no benefit over the first-page approach.

**`LibraryController::start_load` drives progress.**

The existing `start_load` already calls `on_page` per page and has access to `weak_activity`. After the estimated total is known it calls `activity.set_progress(id, 0.0, cx)` to make the bar visible, then after each page it updates progress as `items_loaded as f32 / estimated_total as f32`, clamped to 1.0.

## Risks / Trade-offs

- **Estimate overshoot**: If the final page is short the bar may read ~96% when the load is actually done, then jump to 100% on completion. This is cosmetically acceptable - the `complete()` call always finalizes progress.
- **`links.last` absent**: Some responses may omit `links.last` (e.g., single-page results, API changes). In this case `on_total` is never called and the bar remains indeterminate (existing behavior). No regression.
- **Breaking internal signature**: Adding `on_total` to `SdkLibraryGateway::list_order_products_paged` requires updating test stubs. Low cost - stubs can pass `None`.

## Migration Plan

All changes are additive within the same binary. No database, config, or API changes. Deploy as a regular feature build.
