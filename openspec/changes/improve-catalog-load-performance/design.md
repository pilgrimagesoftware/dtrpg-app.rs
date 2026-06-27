## Context

The current incremental load pipeline works as follows:

1. A GPUI async task spawns a background executor task that calls `list_items_paged`, which loops over API pages, sending each via `std::sync::mpsc::channel`.
2. The async task receives pages one at a time with `background_executor().spawn(async move { receiver.recv() })` and immediately calls `this.update(...)` → `append_catalog_page`.
3. `append_catalog_page` runs on the GPUI main thread: it extends the catalog, recomputes `section_counts` (O(n) pass), recomputes `publisher_entries` (O(n) pass + HashMap + sort), and emits `LibraryChanged`.
4. `LibraryChanged` triggers `LibraryRootView::render`, which calls `snapshot()` → `visible_items()`: a full filter pass, clone of all matching items, and sort — all O(n log n).

With 1 000 items across 10 pages, the main thread runs steps 3–4 ten times. By the final page the catalog has grown to its full size, so the last few renders dominate. Every user interaction during this period (keyboard, mouse, scrolling) must wait for the current render to finish.

## Goals / Non-Goals

**Goals:**
- Reduce the number of main-thread render passes during load from O(pages) to O(seconds / 0.5).
- Eliminate redundant `visible_items()` re-computation across repeated renders of unchanged state.
- Keep the incremental "items appear as they load" user experience — just with lower frequency.

**Non-Goals:**
- Virtualized list rendering (a separate `uniform_list`-based change).
- Moving sort/filter work to a background thread (requires more invasive locking).
- Reducing page size or the number of API requests.

## Decisions

### Batched flush with a 500 ms timer

The background load loop accumulates items in a local `Vec<LibraryItem>` buffer. Rather than sending each page to the controller immediately, the async task tracks the time of the last flush. When either 500 ms have elapsed since the last flush or the load completes, it flushes the buffer to the controller via `this.update(...)`.

Timer mechanism: use `async_cx.background_executor().timer(Duration::from_millis(500))` (GPUI's `BackgroundExecutor::timer`). The loop `select!`s between the next recv message and the timer; whichever fires first is handled, and if the timer fires the buffer is flushed and the timer is reset.

**Why 500 ms**: Long enough to accumulate several pages before rendering; short enough that the user sees visible progress every half second. Configurable in the future if needed.

**Alternative**: Flush every N items (e.g., 500). Rejected — a fixed count is less predictable on slow networks where pages arrive slowly and the user expects to see items sooner.

**Alternative**: Throttle `LibraryChanged` emissions at the GPUI level. Not possible — GPUI does not provide built-in event debouncing.

**Implementation note**: GPUI does not expose `tokio::time::sleep` in the async task context. Use `async_cx.background_executor().timer(duration)` which returns a future that resolves after the given duration, compatible with `futures::select!` or manual `futures::future::select`.

### Visible-items cache

Add `visible_cache: Option<Vec<LibraryItem>>` to `LibraryController`. Mutating methods (`set_filter`, `set_sort`, `set_search_query`, `append_catalog_page`, `reload`) clear the cache by setting it to `None`. `visible_items()` populates and returns the cache when it is `None`; subsequent calls within the same render cycle return the cached value directly.

`snapshot()` calls `visible_items()` once per render. If the controller state has not changed since the last render (same catalog, same filter, same sort, same query), the cache is valid and `visible_items()` returns in O(1).

**Why a cache instead of pre-computing on mutation**: Mutating methods run on the main thread too; pre-computing `visible_items()` there would shift the cost without removing it. The cache amortizes cost across renders — only the first render after a change pays the O(n log n) price.

**Cache invalidation is exact**: every method that changes state that affects the visible set already calls `cx.emit(LibraryChanged)`. We simply clear the cache in those same methods, before emitting. No extra invalidation is needed.

### Sidebar stats at flush boundaries only

`section_counts` and `publisher_entries` are recomputed in `append_catalog_page_batch` (the new batch-flush method) rather than in `append_catalog_page` (which is removed). They run once per 500 ms flush, not once per API page.

## Risks / Trade-offs

- [Risk] The 500 ms timer means the last batch of items may not appear until half a second after the final API page arrives. → Mitigation: the load task flushes immediately on `Err(_)` (channel closed = load complete), so the final flush is not delayed.
- [Risk] Using `futures::select!` requires the `futures` crate. → Mitigation: `futures` is already a transitive dependency through `gpui`; check `Cargo.lock` and add it explicitly to `dtrpg-ui` if not already a direct dep.
- [Risk] The visible cache holds a clone of all visible items. For a 1 000-item catalog this is ~a few MB. → Mitigation: acceptable; the cache lives only as long as the controller and is already the same data the view was getting from the previous non-cached design.
- [Risk] A future caller adds a mutation method and forgets to clear the cache. → Mitigation: add a `#[inline]` `fn invalidate_visible_cache(&mut self)` helper and call it from all mutation methods so the pattern is obvious and grep-able.
